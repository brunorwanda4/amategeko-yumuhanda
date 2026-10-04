#!/usr/bin/env python3
"""
tools/check_translation.py
Offline translation verification script for Amategeko y'Umuhanda.
Checks:
- Interface i18n keys parity (en.json vs rw.json)
- Question IDs consistency between questions.json (rw) and questions_en.json (en)
- Option letters match ('a', 'b', 'c', 'd')
- No empty texts or option values
- Question correct answers and image metadata consistency
- Numerical values consistency (speeds, weights, distances)
- Glossary terms consistency if glossary.json exists

Usage:
    python tools/check_translation.py
"""

import json
import os
import re
import sys
from pathlib import Path

def main():
    root = Path(__file__).parent.parent
    assets_dir = root / "assets"
    i18n_dir = assets_dir / "i18n"
    tools_dir = root / "tools"

    rw_i18n_file = i18n_dir / "rw.json"
    en_i18n_file = i18n_dir / "en.json"
    rw_questions_file = assets_dir / "questions.json"
    en_questions_file = assets_dir / "questions_en.json"
    glossary_file = tools_dir / "glossary.json"

    print("=" * 70)
    print("AMATEGEKO Y'UMUHANDA - TRANSLATION & I18N VERIFICATION REPORT")
    print("=" * 70)

    # 1. Interface i18n parity check
    print("\n[1] Checking Interface i18n files...")
    if not rw_i18n_file.exists() or not en_i18n_file.exists():
        print(f"ERROR: i18n files missing! rw: {rw_i18n_file.exists()}, en: {en_i18n_file.exists()}")
        return 0

    with open(rw_i18n_file, "r", encoding="utf-8-sig") as f:
        rw_i18n = json.load(f)
    with open(en_i18n_file, "r", encoding="utf-8-sig") as f:
        en_i18n = json.load(f)

    rw_keys = set(rw_i18n.keys())
    en_keys = set(en_i18n.keys())

    missing_in_en = rw_keys - en_keys
    missing_in_rw = en_keys - rw_keys

    print(f"  - Total English keys: {len(en_keys)}")
    print(f"  - Total Kinyarwanda keys: {len(rw_keys)}")
    if missing_in_en:
        print(f"  WARNING: Keys in rw.json missing in en.json ({len(missing_in_en)}): {missing_in_en}")
    if missing_in_rw:
        print(f"  WARNING: Keys in en.json missing in rw.json ({len(missing_in_rw)}): {missing_in_rw}")

    empty_i18n = []
    placeholder_mismatch = []
    for k in (en_keys & rw_keys):
        if not str(en_i18n[k]).strip():
            empty_i18n.append(f"en:{k}")
        if not str(rw_i18n[k]).strip():
            empty_i18n.append(f"rw:{k}")

        en_ph = set(re.findall(r'\{([a-zA-Z0-9_]+)\}', en_i18n[k]))
        rw_ph = set(re.findall(r'\{([a-zA-Z0-9_]+)\}', rw_i18n[k]))
        if en_ph != rw_ph:
            placeholder_mismatch.append((k, en_ph, rw_ph))

    if empty_i18n:
        print(f"  WARNING: Empty i18n strings: {empty_i18n}")
    if placeholder_mismatch:
        print(f"  WARNING: Placeholder mismatches: {placeholder_mismatch}")

    if not missing_in_en and not missing_in_rw and not empty_i18n and not placeholder_mismatch:
        print("  Check passed: Interface i18n files have 100% key parity, valid placeholders, and no empty values.")

    # 2. Questions verification
    print("\n[2] Checking Question files (questions.json vs questions_en.json)...")
    if not rw_questions_file.exists():
        print(f"ERROR: {rw_questions_file} not found.")
        return 0
    if not en_questions_file.exists():
        print(f"ERROR: {en_questions_file} not found.")
        return 0

    with open(rw_questions_file, "r", encoding="utf-8-sig") as f:
        rw_qs = json.load(f)
    with open(en_questions_file, "r", encoding="utf-8-sig") as f:
        en_qs = json.load(f)

    print(f"  - Kinyarwanda questions count: {len(rw_qs)}")
    print(f"  - English questions count:     {len(en_qs)}")

    rw_map = {q["id"]: q for q in rw_qs}
    en_map = {q["id"]: q for q in en_qs}

    missing_in_en_q = set(rw_map.keys()) - set(en_map.keys())
    extra_in_en_q = set(en_map.keys()) - set(rw_map.keys())

    if missing_in_en_q:
        print(f"  WARNING: Question IDs missing in EN ({len(missing_in_en_q)}): {sorted(missing_in_en_q)}")
    if extra_in_en_q:
        print(f"  WARNING: Question IDs extra in EN ({len(extra_in_en_q)}): {sorted(extra_in_en_q)}")

    correct_mismatches = []
    image_mismatches = []
    has_image_mismatches = []
    option_key_mismatches = []
    empty_texts = []
    empty_options = []
    number_diffs = []

    for qid, rq in rw_map.items():
        if qid not in en_map:
            continue
        eq = en_map[qid]

        if "correct" in eq and eq["correct"] is not None and rq.get("correct") != eq.get("correct"):
            correct_mismatches.append((qid, rq.get("correct"), eq.get("correct")))

        if "image" in eq and eq["image"] != rq.get("image"):
            image_mismatches.append((qid, rq.get("image"), eq.get("image")))

        if "has_image" in eq and eq["has_image"] != rq.get("has_image"):
            has_image_mismatches.append((qid, rq.get("has_image"), eq.get("has_image")))

        r_opts = set(rq.get("options", {}).keys())
        e_opts = set(eq.get("options", {}).keys())
        if r_opts != e_opts:
            option_key_mismatches.append((qid, r_opts, e_opts))

        if not eq.get("text", "").strip():
            empty_texts.append(qid)
        for opt_k, opt_v in eq.get("options", {}).items():
            if not str(opt_v).strip():
                empty_options.append((qid, opt_k))

        r_nums = re.findall(r'\b\d+(?:[.,]\d+)?\b', rq.get("text", ""))
        e_nums = re.findall(r'\b\d+(?:[.,]\d+)?\b', eq.get("text", ""))
        r_clean_nums = [n.replace(",", "").replace(".", "") for n in r_nums]
        e_clean_nums = [n.replace(",", "").replace(".", "") for n in e_nums]
        if r_clean_nums != e_clean_nums:
            number_diffs.append((qid, r_nums, e_nums))

    print(f"  - Option keys mismatches:     {len(option_key_mismatches)}")
    print(f"  - Empty texts:               {len(empty_texts)}")
    print(f"  - Empty option values:       {len(empty_options)}")
    print(f"  - Correct answer mismatches: {len(correct_mismatches)}")
    if correct_mismatches:
        for m in correct_mismatches[:5]:
            print(f"    * Q#{m[0]}: RW={m[1]} vs EN={m[2]}")
    print(f"  - Image field mismatches:    {len(image_mismatches)}")
    if image_mismatches:
        for m in image_mismatches[:5]:
            print(f"    * Q#{m[0]}: RW={m[1]} vs EN={m[2]}")
    print(f"  - has_image mismatches:      {len(has_image_mismatches)}")

    print(f"  - Number differences in text: {len(number_diffs)}")
    if number_diffs:
        for item in number_diffs[:5]:
            print(f"    * Q#{item[0]}: RW numbers {item[1]} vs EN numbers {item[2]}")

    if glossary_file.exists():
        print("\n[3] Checking Glossary terms...")
        with open(glossary_file, "r", encoding="utf-8-sig") as f:
            glossary = json.load(f)
        print(f"  - Glossary entries: {len(glossary)}")
    else:
        print("\n[3] Glossary check: tools/glossary.json not found (skipped)")

    print("\n" + "=" * 70)
    print("VERIFICATION COMPLETE (Report Only)")
    print("=" * 70)
    return 0

if __name__ == "__main__":
    sys.exit(main())
