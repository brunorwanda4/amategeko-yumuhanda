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
from PIL import Image

def find_pdf_path():
    candidates = [
        os.path.join(os.path.expanduser("~"), "Downloads", "ibibazo byamategeko y'umuhanda.pdf"),
        os.path.join("..", "ibibazo byamategeko y'umuhanda.pdf"),
        "ibibazo_byamategeko_y_umuhanda.pdf",
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
                    has_red = False
                    for s in l.get("spans", []):
                        t = s.get("text", "")
                        c = s.get("color", 0)
                        r = (c >> 16) & 0xFF
                        g = (c >> 8) & 0xFF
                        bv = c & 0xFF
                        if r > 150 and g < 100 and bv < 100:
                            has_red = True
                        line_text += t

                    norm = line_text.replace("\u2019", "'").replace("\u2018", "'").replace("`", "'")
                    lines.append((norm, has_red))

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
    def is_option_line(line):
        return bool(re.match(r'^\s*\(?\s*[a-dA-D]\s*[\)\.]', line))

    def extract_q_num_match(line, expected):
        m = re.match(r'^\s*(\d{1,3})\s*[\.\)]\s*(.*)', line)
        if not m:
            m = re.match(r'^\s*(\d{1,3})([A-Za-z\u00C0-\u024F\u1E00-\u1EFF\"\'\(\?\!].*)', line)
        if m:
            num = int(m.group(1))
            rest = m.group(2).strip()
            if is_option_line(rest):
                return None
            if expected - 3 <= num <= expected + 35:
                return (num, rest)
        return None

    grouped_questions = []
    current_q = None
    expected_q_num = 1

    for item in stream:
        if item["type"] == "image":
            if current_q is not None:
                current_q["images"].append(item)
        elif item["type"] == "text":
            for line_text, has_red in item["lines"]:
                line_str = line_text.strip()
                if not line_str:
                    continue
                q_match = extract_q_num_match(line_str, expected_q_num)
                if q_match:
                    q_num, q_rest = q_match
                    if current_q is not None:
                        grouped_questions.append(current_q)
                    current_q = {
                        "pdf_num": q_num,
                        "lines": [(q_rest, has_red)] if q_rest else [],
                        "images": [],
                        "page": item["page"]
                    }
                    expected_q_num = q_num
                else:
                    if current_q is not None:
                        current_q["lines"].append((line_str, has_red))

    if current_q is not None:
        grouped_questions.append(current_q)

    print(f"Extracted {len(grouped_questions)} question blocks from PDF stream.")

    # Step 3: Parse options, detect parenthesized / red answers, and crop images
    opt_marker_re = re.compile(r'(?:(?<=^)|(?<=\n)|(?<=\s\s))\s*(\(?\s*([a-dA-D])\s*(?:[\)\.]|\s*\)))\s*')

    final_questions = []
    review_list = []
    used_ids = set()

    for idx, q in enumerate(grouped_questions):
        # Assign a unique sequential ID while keeping close to original numbering
        assigned_id = idx + 1

        full_text = "\n".join(l[0] for l in q["lines"])
        red_lines = [l[0] for l in q["lines"] if l[1]]

        markers = list(opt_marker_re.finditer(full_text))
        if not markers:
            review_list.append({
                "id": assigned_id,
                "pdf_num": q["pdf_num"],
                "reason": "no_option_markers",
                "text": full_text[:120]
            })
            continue

        prompt = full_text[:markers[0].start()].strip()
        prompt = re.sub(r'^\d{1,3}\s*[\.\)]?\s*', '', prompt)

        options = {}
        correct = None

        for i, m in enumerate(markers):
            raw_marker = m.group(1).strip()
            letter = m.group(2).lower()
            if raw_marker.startswith("(") and (raw_marker.endswith(")") or ")" in raw_marker):
                correct = letter

            opt_start = m.end()
            opt_end = markers[i+1].start() if i+1 < len(markers) else len(full_text)
            opt_text = full_text[opt_start:opt_end].strip()
            opt_text = re.sub(r'\s*\n\s*', ' ', opt_text)
            options[letter] = opt_text

        # If parenthesis was omitted in PDF, detect option with red text
        if not correct:
            for letter, opt_text in options.items():
                for rl in red_lines:
                    if (opt_text and opt_text[:12] in rl) or (rl and rl[:12] in opt_text) or (f"{letter})" in rl) or (f"{letter}." in rl):
                        correct = letter
                        break
                if correct:
                    break

        has_image = len(q["images"]) > 0
        image_filename = f"q{assigned_id}.png" if has_image else None

        # Crop and save image if present
        if has_image:
            p_idx = q["images"][0]["page"]
            union_rect = fitz.Rect(q["images"][0]["rect"])
            for im in q["images"]:
                if im["page"] == p_idx:
                    union_rect.include_rect(im["rect"])

            # Add 2pt margin
            union_rect.x0 = max(0, union_rect.x0 - 2)
            union_rect.y0 = max(0, union_rect.y0 - 2)
            union_rect.x1 = min(792, union_rect.x1 + 2)
            union_rect.y1 = min(612, union_rect.y1 + 2)

            page = doc[p_idx]
            pix = page.get_pixmap(clip=union_rect, dpi=144)
            img_path = os.path.join(images_dir, image_filename)
            pix.save(img_path)

        question_obj = {
            "id": assigned_id,
            "text": prompt,
            "options": options,
            "correct": correct if correct else "a",
            "image": image_filename,
            "has_image": has_image
        }

        # Apply overrides if available
        str_id = str(assigned_id)
        if str_id in overrides:
            ov = overrides[str_id]
            for k, v in ov.items():
                question_obj[k] = v

        final_questions.append(question_obj)

        # Validate
        reasons = []
        if len(question_obj["options"]) not in (3, 4):
            reasons.append(f"options_count_{len(question_obj['options'])}")
        if not question_obj.get("correct") or question_obj["correct"] not in question_obj["options"]:
            reasons.append("invalid_or_missing_correct_answer")
        if not question_obj.get("text"):
            reasons.append("empty_prompt")

        if reasons:
            review_list.append({
                "id": assigned_id,
                "pdf_num": q["pdf_num"],
                "reasons": reasons,
                "question": question_obj
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
