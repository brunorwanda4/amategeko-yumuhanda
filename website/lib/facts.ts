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

export interface CreditRow {
  name: string;
  purpose: string;
  author: string;
  license: string;
  linkLabel: string;
  href: string;
}

export interface CreditGroup {
  title: string;
  rows: CreditRow[];
}

function parseCreditCell(value: string): string {
  return value.trim().replaceAll("\\|", "|");
}

export function getCreditGroups(): CreditGroup[] {
  if (typeof window !== "undefined" || typeof process === "undefined" || !process.versions?.node) {
    throw new Error("Credits can only be loaded while building the website.");
  }

  const fs = eval("require")("node:fs");
  const path = eval("require")("node:path");
  const candidates = [
    path.join(process.cwd(), "../docs/CREDITS.md"),
    path.join(process.cwd(), "docs/CREDITS.md"),
  ];
  const filePath = candidates.find((candidate: string) => fs.existsSync(candidate));
  if (!filePath) {
    throw new Error(`Credits file is missing: ${candidates.join(", ")}`);
  }

  const markdown = fs.readFileSync(filePath, "utf-8") as string;
  const groups: CreditGroup[] = [];
  let current: CreditGroup | undefined;

  for (const line of markdown.split(/\r?\n/)) {
    if (line.startsWith("## ")) {
      current = { title: line.slice(3).trim(), rows: [] };
      groups.push(current);
      continue;
    }
    if (!current || !line.startsWith("|") || line.includes("|---")) continue;

    const cells = line.slice(1, -1).split("|").map(parseCreditCell);
    if (cells[0] === "Name" || cells.length !== 5) continue;
    const link = cells[4].match(/^\[([^\]]+)]\((https?:\/\/[^)]+)\)$/);
    if (!link) {
      throw new Error(`Invalid credit link in ${current.title}: ${cells[4]}`);
    }
    current.rows.push({
      name: cells[0],
      purpose: cells[1],
      author: cells[2],
      license: cells[3],
      linkLabel: link[1],
      href: link[2],
    });
  }

  return groups.filter((group) => group.rows.length > 0);
}
