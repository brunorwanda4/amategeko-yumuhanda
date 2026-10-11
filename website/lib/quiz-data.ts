import rawQuestions from "@/lib/quiz-questions.json";

export interface QuizQuestion {
  id: number;
  correct: "a" | "b" | "c" | "d";
  image: string | null;
  rw: {
    question: string;
    options: string[];
  };
  en: {
    question: string;
    options: string[];
  };
}

export const allQuestions: QuizQuestion[] = rawQuestions as QuizQuestion[];

export function getRandomQuestions(count: number = 10): QuizQuestion[] {
  const pool = [...allQuestions];
  for (let i = pool.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    const temp = pool[i];
    pool[i] = pool[j];
    pool[j] = temp;
  }
  return pool.slice(0, count);
}

export const initialPreviewQuestions: QuizQuestion[] = allQuestions.slice(0, 10);
