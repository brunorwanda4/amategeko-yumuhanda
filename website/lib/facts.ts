import samples from "@/content/samples/questions.json";
import { APP_QUESTION_COUNT } from "./config";

function getSampleCount(): number {
  if (typeof window === "undefined" && typeof process !== "undefined" && process.versions?.node) {
    const fs = eval("require")("node:fs");
    const path = eval("require")("node:path");
    const candidates = [
      path.join(process.cwd(), "content/samples/questions.json"),
      path.join(process.cwd(), "website/content/samples/questions.json"),
    ];

    const filePath = candidates.find((p: string) => fs.existsSync(p));
    if (!filePath) {
      throw new Error(
        `Sample questions file is missing: could not find questions.json in candidates: ${candidates.join(
          ", ",
        )}`,
      );
    }

    const raw = fs.readFileSync(filePath, "utf-8");
    if (!raw.trim()) {
      throw new Error(`Sample questions file is empty at: ${filePath}`);
    }

    const parsed = JSON.parse(raw);
    if (!Array.isArray(parsed) || parsed.length === 0) {
      throw new Error(`Sample questions file at ${filePath} contains no questions`);
    }

    return parsed.length;
  }

  if (!Array.isArray(samples) || samples.length === 0) {
    throw new Error("Sample questions file is missing or empty");
  }

  return samples.length;
}

export const sampleCount = getSampleCount();
export { APP_QUESTION_COUNT };
