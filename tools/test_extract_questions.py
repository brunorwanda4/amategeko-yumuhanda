import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(__file__))

from extract_questions import (
    append_missing_override_questions,
    assign_stable_ids,
    extract_question_heading,
    parse_question_text,
)


def line(text, red=False):
    return {
        "text": text,
        "red_mask": [red] * len(text),
        "is_bold": False,
    }


class ExtractQuestionsTests(unittest.TestCase):
    def test_question_heading_accepts_pdf_numbering_errors(self):
        self.assertEqual(
            extract_question_heading("8. Iki cyapa gisobanura iki?", False),
            (8, "Iki cyapa gisobanura iki?"),
        )
        self.assertEqual(
            extract_question_heading("225Wegereye inzira", True),
            (225, "Wegereye inzira"),
        )
        self.assertIsNone(extract_question_heading("20km/h", False))

    def test_parenthesized_answer_supports_dot_inside_parentheses(self):
        prompt, options, correct, reasons = parse_question_text(
            [line("Ikibazo"), line("a. Oya"), line("(b.) Yego", red=True)]
        )
        self.assertEqual(prompt, "Ikibazo")
        self.assertEqual(options, {"a": "Oya", "b": "Yego"})
        self.assertEqual(correct, "b")
        self.assertEqual(reasons, [])

    def test_red_answer_is_selected_without_guessing(self):
        _, _, correct, reasons = parse_question_text(
            [
                line("Ikibazo"),
                line("a) Icya mbere"),
                line("b) Icya kabiri", red=True),
                line("c) Icya gatatu"),
            ]
        )
        self.assertEqual(correct, "b")
        self.assertEqual(reasons, [])

    def test_missing_answer_is_reported(self):
        _, _, correct, reasons = parse_question_text(
            [line("Ikibazo"), line("a) Kimwe"), line("b) Kabiri")]
        )
        self.assertIsNone(correct)
        self.assertIn("missing_or_ambiguous_correct_answer", reasons)

    def test_existing_ids_are_preserved_and_new_rows_are_appended(self):
        parsed = [{"text": "Old"}, {"text": "Recovered"}]
        existing = [{"id": 7, "text": "Old"}]
        assigned = assign_stable_ids(parsed, existing)
        self.assertEqual([question["id"] for question in assigned], [7, 8])

    def test_complete_override_can_recover_missing_source_record(self):
        questions = [{"id": 1, "text": "Extracted"}]
        override = {
            "id": 2,
            "text": "Recovered",
            "options": {"a": "Yego", "b": "Oya", "c": "Nta na kimwe"},
            "correct": "a",
            "image": None,
            "has_image": False,
        }
        recovered = append_missing_override_questions(
            questions, {"1": {"text": "Updated"}, "2": override}
        )
        self.assertEqual(recovered, [override])
        self.assertEqual([question["id"] for question in questions], [1, 2])


if __name__ == "__main__":
    unittest.main()
