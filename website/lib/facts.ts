import quizQuestions from "./quiz-questions.json";
import { APP_QUESTION_COUNT } from "./config";

function getQuestionCount(): number {
  let count = 0;
  if (typeof window === "undefined" && typeof process !== "undefined" && process.versions?.node) {
    const fs = eval("require")("node:fs");
    const path = eval("require")("node:path");
    const candidates = [
      path.join(process.cwd(), "lib/quiz-questions.json"),
      path.join(process.cwd(), "website/lib/quiz-questions.json"),
    ];

    const filePath = candidates.find((p: string) => fs.existsSync(p));
    if (!filePath) {
      throw new Error(
        `Quiz questions file is missing: could not find quiz-questions.json in candidates: ${candidates.join(
          ", ",
        )}`,
      );
    }

    const raw = fs.readFileSync(filePath, "utf-8");
    if (!raw.trim()) {
      throw new Error(`Quiz questions file is empty at: ${filePath}`);
    }

    const parsed = JSON.parse(raw);
    if (!Array.isArray(parsed) || parsed.length === 0) {
      throw new Error(`Quiz questions file at ${filePath} contains no questions`);
    }

    count = parsed.length;
  } else {
    if (!Array.isArray(quizQuestions) || quizQuestions.length === 0) {
      throw new Error("Quiz questions file is missing or empty");
    }
    count = quizQuestions.length;
  }

  if (count !== APP_QUESTION_COUNT) {
    throw new Error(
      `Question count mismatch: quiz-questions.json has ${count} questions, but APP_QUESTION_COUNT in config.ts is ${APP_QUESTION_COUNT}.`,
    );
  }

  return count;
}

export const questionCount = getQuestionCount();
export { APP_QUESTION_COUNT };