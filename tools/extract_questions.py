#!/usr/bin/env python3
"""
tools/extract_questions.py

One-time PDF question extraction and image cropper for "Amategeko y'Umuhanda".
Source: ibibazo_byamategeko_y_umuhanda.pdf (75 landscape pages, 2 columns per page).
Outputs:
  - assets/questions.json
  - assets/images/q{id}.png
  - tools/needs_review.json
Supports overrides via assets/overrides.json.
"""

import os
import sys
import re
import json
import fitz  # PyMuPDF


OPTION_MARKER_RE = re.compile(
    r"(?:(?<=^)|(?<=\n)|(?<=\s\s))\s*"
    r"((?:\(\s*([a-dA-D])\s*(?:\.\s*)?\))|(?:([a-dA-D])\s*[\)\.]))\s*"
)

# The PDF omits question 272's heading, so its image/options are attached to
# question 271 during raw grouping. Keep the reviewed question 271 on its
# established app ID even though its override no longer matches that raw text.
SOURCE_ID_OVERRIDES = {(40, 271): 230}

# Question 272 is visibly present on source page 42, but its heading was lost
# when the source was converted to the landscape/two-up PDF. A matching copy
# preserves the heading. Keep the recovered record at the end of the app data
# so existing question IDs and saved progress remain stable.
RECOVERED_IMAGE_RECTS = {404: (41, (91.6, 72.5, 198.1, 179.0))}


def normalize_text(text):
    return text.replace("\u2019", "'").replace("\u2018", "'").replace("`", "'")


def stable_text_key(text):
    normalized = normalize_text(text).replace("\ufffd", "'")
    return " ".join(normalized.split()).casefold()


def stable_question_key(question):
    option_values = question.get("options", {}).values()
    return stable_text_key(question.get("text", "")), tuple(
        stable_text_key(value.lstrip(") ")) for value in option_values
    )


def is_red(color):
    red = (color >> 16) & 0xFF
    green = (color >> 8) & 0xFF
    blue = color & 0xFF
    return red > 150 and green < 100 and blue < 100


def is_option_line(line):
    return bool(re.match(r"^\s*\(?\s*[a-dA-D]\s*[\)\.]", line))


def extract_question_heading(line, is_bold):
    """Return a PDF question number and prompt without trusting number order.

    The source contains backward jumps, duplicates, and a question numbered 8
    between 409 and 410. Joined headings such as ``225Wegereye`` are accepted
    only when the PDF marks the line as bold, which excludes image labels such
    as ``20km/h``.
    """
    match = re.match(r"^\s*(\d{1,3})\s*[\.\)]\s*(.*)", line)
    if not match and is_bold:
        match = re.match(
            r"^\s*(\d{1,3})([A-Za-z\u00C0-\u024F\u1E00-\u1EFF\"'\(\?\!].*)",
            line,
        )
    if not match:
        return None

    rest = match.group(2).strip()
    if is_option_line(rest):
        return None
    return int(match.group(1)), rest


def join_lines_with_red_mask(lines):
    full_text = ""
    red_mask = []
    for index, line in enumerate(lines):
        if index:
            full_text += "\n"
            red_mask.append(False)
        full_text += line["text"]
        red_mask.extend(line["red_mask"])
    return full_text, red_mask


def parse_question_text(lines):
    """Parse prompt/options and derive the answer from PDF markup.

    Parentheses are authoritative. When the source omits them, the option with
    the most red characters is used. The function never guesses answer ``a``.
    """
    full_text, red_mask = join_lines_with_red_mask(lines)
    markers = list(OPTION_MARKER_RE.finditer(full_text))
    if not markers:
        return None, {}, None, ["no_option_markers"]

    prompt = full_text[: markers[0].start()].strip()
    options = {}
    marker_letters = []
    parenthesized_answers = []
    red_counts = {}

    for index, marker in enumerate(markers):
        letter = (marker.group(2) or marker.group(3)).lower()
        marker_letters.append(letter)
        if marker.group(2):
            parenthesized_answers.append(letter)

        option_start = marker.end()
        option_end = (
            markers[index + 1].start() if index + 1 < len(markers) else len(full_text)
        )
        option_text = full_text[option_start:option_end].strip()
        options[letter] = re.sub(r"\s*\n\s*", " ", option_text)
        red_counts[letter] = red_counts.get(letter, 0) + sum(
            red_mask[marker.start() : option_end]
        )

    reasons = []
    if len(marker_letters) != len(set(marker_letters)):
        reasons.append("duplicate_option_markers")

    distinct_parenthesized = list(dict.fromkeys(parenthesized_answers))
    if len(distinct_parenthesized) == 1:
        correct = distinct_parenthesized[0]
    elif len(distinct_parenthesized) > 1:
        correct = None
        reasons.append("multiple_parenthesized_answers")
    else:
        highest_red_count = max(red_counts.values(), default=0)
        red_answers = [
            letter
            for letter, count in red_counts.items()
            if count == highest_red_count and count > 0
        ]
        if len(red_answers) == 1:
            correct = red_answers[0]
        else:
            correct = None
            reasons.append("missing_or_ambiguous_correct_answer")

    return prompt, options, correct, reasons


def assign_stable_ids(parsed_questions, existing_questions):
    """Keep existing IDs when prompts still match; append newly recovered rows."""
    existing_by_question = {}
    existing_by_text = {}
    for question in existing_questions:
        existing_by_question.setdefault(stable_question_key(question), []).append(question["id"])
        existing_by_text.setdefault(stable_text_key(question.get("text", "")), []).append(
            question["id"]
        )

    used_ids = set()
    next_id = max((question["id"] for question in existing_questions), default=0) + 1
    for question in parsed_questions:
        source_id = SOURCE_ID_OVERRIDES.get(
            (question.get("_page"), question.get("_pdf_num"))
        )
        if source_id is not None and source_id not in used_ids:
            assigned_id = source_id
        else:
            candidates = existing_by_question.get(stable_question_key(question), [])
            if not candidates:
                prompt_candidates = existing_by_text.get(stable_text_key(question["text"]), [])
                if len(prompt_candidates) == 1:
                    candidates = prompt_candidates
            assigned_id = next(
                (candidate for candidate in candidates if candidate not in used_ids), None
            )
        if assigned_id is None:
            assigned_id = next_id
            next_id += 1
        question["id"] = assigned_id
        used_ids.add(assigned_id)

    return sorted(parsed_questions, key=lambda question: question["id"])


def append_missing_override_questions(final_questions, overrides):
    """Append complete reviewed overrides that have no extracted base record."""
    existing_ids = {question["id"] for question in final_questions}
    recovered = []
    for key, override in overrides.items():
        if not key.isdigit() or not isinstance(override, dict):
            continue
        question_id = int(key)
        required = {"id", "text", "options", "correct", "image", "has_image"}
        if question_id in existing_ids or not required.issubset(override):
            continue
        if override["id"] != question_id:
            continue
        recovered.append(dict(override))

    final_questions.extend(recovered)
    final_questions.sort(key=lambda question: question["id"])
    return recovered

def find_pdf_path():
    candidates = [
        "ibibazo_byamategeko_y_umuhanda.pdf",
        os.path.join(os.path.expanduser("~"), "Downloads", "ibibazo byamategeko y'umuhanda.pdf"),
        os.path.join("..", "ibibazo byamategeko y'umuhanda.pdf"),
    ]
    for p in candidates:
        if os.path.exists(p):
            return p
    return None

def extract_questions():
    pdf_path = find_pdf_path()
    if not pdf_path:
        print("ERROR: Source PDF 'ibibazo byamategeko y'umuhanda.pdf' not found!", file=sys.stderr)
        sys.exit(1)

    print(f"Opening PDF: {pdf_path}")
    doc = fitz.open(pdf_path)
    total_pages = len(doc)
    print(f"Total pages: {total_pages}")

    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    assets_dir = os.path.join(base_dir, "assets")
    images_dir = os.path.join(assets_dir, "images")
    overrides_file = os.path.join(assets_dir, "overrides.json")
    output_json = os.path.join(assets_dir, "questions.json")
    review_json = os.path.join(base_dir, "tools", "needs_review.json")

    os.makedirs(images_dir, exist_ok=True)
    os.makedirs(os.path.dirname(review_json), exist_ok=True)

    overrides = {}
    if os.path.exists(overrides_file):
        try:
            with open(overrides_file, "r", encoding="utf-8") as f:
                overrides = json.load(f)
            print(f"Loaded {len(overrides)} override entries from assets/overrides.json")
        except Exception as e:
            print(f"Warning: Failed to parse overrides.json: {e}", file=sys.stderr)

    existing_questions = []
    if os.path.exists(output_json):
        try:
            with open(output_json, "r", encoding="utf-8") as f:
                existing_questions = json.load(f)
        except (OSError, json.JSONDecodeError) as e:
            print(f"Warning: Existing questions.json could not be read: {e}", file=sys.stderr)

    # Step 1: Collect reading-order stream of elements (text lines with color and image placements)
    stream = []
    for p_idx, page in enumerate(doc):
        page_dict = page.get_text("dict")
        # Extract text blocks
        for b in page_dict.get("blocks", []):
            if b.get("type") == 0:  # text
                rect = fitz.Rect(b["bbox"])
                # Filter headers (page number, RESTRICTED at top) and footers (RESTRICTED at bottom)
                if rect.y0 < 62:
                    continue
                if rect.y1 > 555 and "RESTRICTED" in page.get_text("text", clip=rect):
                    continue

                col = 0 if rect.x0 < 396 else 1
                lines = []
                for l in b.get("lines", []):
                    line_text = ""
                    red_mask = []
                    is_bold = False
                    for s in l.get("spans", []):
                        t = normalize_text(s.get("text", ""))
                        span_is_red = is_red(s.get("color", 0))
                        line_text += t
                        red_mask.extend([span_is_red] * len(t))
                        if t.strip() and "Bold" in s.get("font", ""):
                            is_bold = True

                    lines.append({
                        "text": line_text,
                        "red_mask": red_mask,
                        "is_bold": is_bold,
                    })

                stream.append({
                    "type": "text",
                    "page": p_idx,
                    "col": col,
                    "rect": rect,
                    "lines": lines
                })

        # Extract embedded images and rects
        for img in page.get_images(full=True):
            xref = img[0]
            for r in page.get_image_rects(xref):
                if r.width > 700 and r.height > 500:
                    continue
                col = 0 if r.x0 < 396 else 1
                stream.append({
                    "type": "image",
                    "page": p_idx,
                    "col": col,
                    "rect": r,
                    "xref": xref
                })

    # Sort stream strictly by page, column (left then right), then vertical y0
    stream.sort(key=lambda item: (item["page"], item["col"], item["rect"].y0))

    # Step 2: Group stream into questions
    grouped_questions = []
    current_q = None

    for item in stream:
        if item["type"] == "image":
            if current_q is not None:
                current_q["images"].append(item)
        elif item["type"] == "text":
            for line in item["lines"]:
                line_text = line["text"]
                line_str = line_text.strip()
                if not line_str:
                    continue
                leading_whitespace = len(line_text) - len(line_text.lstrip())
                trailing_whitespace = len(line_text) - len(line_text.rstrip())
                mask_end = len(line["red_mask"]) - trailing_whitespace
                stripped_line = {
                    "text": line_str,
                    "red_mask": line["red_mask"][
                        leading_whitespace : mask_end if trailing_whitespace else None
                    ],
                    "is_bold": line["is_bold"],
                }
                q_match = extract_question_heading(line_str, line["is_bold"])
                if q_match:
                    q_num, q_rest = q_match
                    if current_q is not None:
                        grouped_questions.append(current_q)
                    rest_start = line_str.find(q_rest) if q_rest else len(line_str)
                    current_q = {
                        "pdf_num": q_num,
                        "lines": ([{
                            "text": q_rest,
                            "red_mask": stripped_line["red_mask"][
                                rest_start : rest_start + len(q_rest)
                            ],
                            "is_bold": line["is_bold"],
                        }] if q_rest else []),
                        "images": [],
                        "page": item["page"]
                    }
                else:
                    if current_q is not None:
                        current_q["lines"].append(stripped_line)

    if current_q is not None:
        grouped_questions.append(current_q)

    print(f"Extracted {len(grouped_questions)} question blocks from PDF stream.")

    # Step 3: Parse options, detect parenthesized / red answers, and crop images
    parsed_questions = []
    review_list = []

    for q in grouped_questions:
        prompt, options, correct, parse_reasons = parse_question_text(q["lines"])
        if prompt is None:
            prompt = ""

        has_image = len(q["images"]) > 0
        question_obj = {
            "text": prompt,
            "options": options,
            "correct": correct,
            "image": None,
            "has_image": has_image,
            "_pdf_num": q["pdf_num"],
            "_page": q["page"],
            "_images": q["images"],
            "_parse_reasons": parse_reasons,
        }
        parsed_questions.append(question_obj)

    final_questions = assign_stable_ids(parsed_questions, existing_questions)

    for question_obj in final_questions:
        assigned_id = question_obj["id"]
        q_images = question_obj.pop("_images")
        pdf_num = question_obj.pop("_pdf_num")
        page_number = question_obj.pop("_page")
        parse_reasons = question_obj.pop("_parse_reasons")
        image_filename = f"q{assigned_id}.png" if q_images else None
        question_obj["image"] = image_filename

        if q_images:
            p_idx = q_images[0]["page"]
            union_rect = fitz.Rect(q_images[0]["rect"])
            for image in q_images:
                if image["page"] == p_idx:
                    union_rect.include_rect(image["rect"])

            union_rect.x0 = max(0, union_rect.x0 - 2)
            union_rect.y0 = max(0, union_rect.y0 - 2)
            union_rect.x1 = min(792, union_rect.x1 + 2)
            union_rect.y1 = min(612, union_rect.y1 + 2)

            pix = doc[p_idx].get_pixmap(clip=union_rect, dpi=144)
            pix.save(os.path.join(images_dir, image_filename))

        # Apply overrides if available
        str_id = str(assigned_id)
        if str_id in overrides:
            ov = overrides[str_id]
            for k, v in ov.items():
                question_obj[k] = v
            parse_reasons = []

        # Validate
        reasons = list(parse_reasons)
        if len(question_obj["options"]) not in (3, 4):
            reasons.append(f"options_count_{len(question_obj['options'])}")
        if not question_obj.get("correct") or question_obj["correct"] not in question_obj["options"]:
            reasons.append("invalid_or_missing_correct_answer")
        if not question_obj.get("text"):
            reasons.append("empty_prompt")

        if reasons:
            review_list.append({
                "id": assigned_id,
                "pdf_num": pdf_num,
                "page": page_number + 1,
                "reasons": reasons,
                "question": question_obj
            })

    recovered_questions = append_missing_override_questions(final_questions, overrides)
    for question_obj in recovered_questions:
        assigned_id = question_obj["id"]
        image_source = RECOVERED_IMAGE_RECTS.get(assigned_id)
        if image_source:
            page_number, rect_values = image_source
            crop_rect = fitz.Rect(rect_values)
            crop_rect.x0 = max(0, crop_rect.x0 - 2)
            crop_rect.y0 = max(0, crop_rect.y0 - 2)
            crop_rect.x1 = min(doc[page_number].rect.width, crop_rect.x1 + 2)
            crop_rect.y1 = min(doc[page_number].rect.height, crop_rect.y1 + 2)
            pix = doc[page_number].get_pixmap(clip=crop_rect, dpi=144)
            pix.save(os.path.join(images_dir, question_obj["image"]))

        reasons = []
        if len(question_obj["options"]) not in (3, 4):
            reasons.append(f"options_count_{len(question_obj['options'])}")
        if question_obj.get("correct") not in question_obj["options"]:
            reasons.append("invalid_or_missing_correct_answer")
        if not question_obj.get("text"):
            reasons.append("empty_prompt")
        if question_obj.get("has_image") and not image_source:
            reasons.append("missing_recovered_image_source")
        if reasons:
            review_list.append({
                "id": assigned_id,
                "pdf_num": None,
                "page": None,
                "reasons": reasons,
                "question": question_obj,
            })

    # Step 4: Write output questions.json
    with open(output_json, "w", encoding="utf-8") as f:
        json.dump(final_questions, f, ensure_ascii=False, indent=2)

    # Step 5: Write tools/needs_review.json
    with open(review_json, "w", encoding="utf-8") as f:
        json.dump(review_list, f, ensure_ascii=False, indent=2)

    print("\n" + "="*50)
    print("EXTRACTION AND VALIDATION REPORT")
    print("="*50)
    print(f"Total questions written: {len(final_questions)}")
    print(f"Total images saved in assets/images/: {sum(1 for q in final_questions if q['has_image'])}")
    print(f"Questions needing manual review: {len(review_list)}")
    print(f"Output files:")
    print(f"  - Questions JSON: {output_json}")
    print(f"  - Needs review:   {review_json}")
    print("="*50 + "\n")

if __name__ == "__main__":
    extract_questions()
