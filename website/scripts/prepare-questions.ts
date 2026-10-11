import fs from "node:fs";
import path from "node:path";

interface QuizQuestionItem {
  id: number;
  correct: string;
  image?: string | null;
  rw: {
    question: string;
    options: string[];
  };
  en?: {
    question: string;
    options: string[];
  };
}

interface AppQuestion {
  id: number;
  text: string;
  options: Record<string, string>;
  correct: string;
  image: string | null;
  has_image: boolean;
}

const OPTION_KEYS = ["a", "b", "c", "d"];

function normalizeText(s: string | null | undefined): string {
  if (!s) return "";
  return s
    .replace(/[\u2018\u2019\u0060\u00B4']/g, "'")
    .replace(/\s+/g, " ")
    .trim();
}

function run() {
  const projectRoot = path.resolve(__dirname, "../..");
  const quizQuestionsPath = path.join(projectRoot, "website/lib/quiz-questions.json");
  const assetsRwPath = path.join(projectRoot, "assets/questions.json");
  const assetsEnPath = path.join(projectRoot, "assets/questions_en.json");
  const assetsImagesDir = path.join(projectRoot, "assets/images");
  const genDir = path.join(projectRoot, "crates/web/generated");
  const runtimeImagesDir = path.join(projectRoot, "website/public/runtime/images");

  if (!fs.existsSync(quizQuestionsPath)) {
    console.error(`Error: File not found: ${quizQuestionsPath}`);
    process.exit(1);
  }

  const quizData: QuizQuestionItem[] = JSON.parse(
    fs.readFileSync(quizQuestionsPath, "utf8")
  );

  let appRwQuestions: AppQuestion[] = [];
  let appEnQuestions: AppQuestion[] = [];

  if (fs.existsSync(assetsRwPath)) {
    appRwQuestions = JSON.parse(fs.readFileSync(assetsRwPath, "utf8"));
  }
  if (fs.existsSync(assetsEnPath)) {
    appEnQuestions = JSON.parse(fs.readFileSync(assetsEnPath, "utf8"));
  }

  const appRwMap = new Map<number, AppQuestion>(
    appRwQuestions.map((q) => [q.id, q])
  );
  const appEnMap = new Map<number, AppQuestion>(
    appEnQuestions.map((q) => [q.id, q])
  );

  const fatalErrors: string[] = [];
  const warnings: string[] = [];

  const seenIds = new Set<number>();
  const duplicateIds: number[] = [];
  const invalidOptionCountIds: number[] = [];
  const invalidCorrectKeyIds: number[] = [];
  const mismatchCorrectIds: { id: number; quiz: string; app: string }[] = [];
  const missingImages: { id: number; image: string }[] = [];

  let rwCount = 0;
  let enCount = 0;
  let withImageCount = 0;

  const generatedRw: AppQuestion[] = [];
  const generatedEn: AppQuestion[] = [];

  for (const q of quizData) {
    if (seenIds.has(q.id)) {
      duplicateIds.push(q.id);
    }
    seenIds.add(q.id);

    const rwOptions = q.rw?.options || [];
    if (rwOptions.length < 3 || rwOptions.length > 4) {
      invalidOptionCountIds.push(q.id);
    }

    const validLetters = OPTION_KEYS.slice(0, rwOptions.length);
    if (!validLetters.includes(q.correct)) {
      invalidCorrectKeyIds.push(q.id);
    }

    const appQ = appRwMap.get(q.id);
    if (appQ && appQ.correct !== q.correct) {
      mismatchCorrectIds.push({
        id: q.id,
        quiz: q.correct,
        app: appQ.correct,
      });
    }

    if (q.image) {
      withImageCount++;
      const srcImagePath = path.join(assetsImagesDir, q.image);
      if (!fs.existsSync(srcImagePath)) {
        missingImages.push({ id: q.id, image: q.image });
      }
    }

    // Build Kinyarwanda question object
    const rwOptionsObj: Record<string, string> = {};
    for (let i = 0; i < rwOptions.length; i++) {
      rwOptionsObj[OPTION_KEYS[i]] = rwOptions[i];
    }
    generatedRw.push({
      id: q.id,
      text: q.rw.question,
      options: rwOptionsObj,
      correct: q.correct,
      image: q.image || null,
      has_image: Boolean(q.image),
    });
    rwCount++;

    // Build English question object (fallback to Kinyarwanda if missing)
    const enOptions = q.en?.options || [];
    const enHasText = Boolean(q.en?.question && q.en.question.trim().length > 0);

    if (enHasText && enOptions.length >= 3 && enOptions.length <= 4) {
      enCount++;
      const enOptionsObj: Record<string, string> = {};
      for (let i = 0; i < enOptions.length; i++) {
        enOptionsObj[OPTION_KEYS[i]] = enOptions[i];
      }
      generatedEn.push({
        id: q.id,
        text: q.en!.question,
        options: enOptionsObj,
        correct: q.correct,
        image: q.image || null,
        has_image: Boolean(q.image),
      });
    } else {
      warnings.push(
        `[WARN] Question #${q.id}: English translation missing or incomplete; falling back to Kinyarwanda.`
      );
      generatedEn.push({
        id: q.id,
        text: q.rw.question,
        options: rwOptionsObj,
        correct: q.correct,
        image: q.image || null,
        has_image: Boolean(q.image),
      });
    }

    // Text comparison warnings
    if (appQ) {
      if (normalizeText(q.rw.question) !== normalizeText(appQ.text)) {
        warnings.push(
          `[WARN] Question #${q.id} (rw): text differs between quiz-questions.json and assets/questions.json after normalization.`
        );
      }
    }
    const appEnQ = appEnMap.get(q.id);
    if (appEnQ && q.en?.question) {
      if (normalizeText(q.en.question) !== normalizeText(appEnQ.text)) {
        warnings.push(
          `[WARN] Question #${q.id} (en): text differs between quiz-questions.json and assets/questions_en.json after normalization.`
        );
      }
    }
  }

  // Cross-source ID differences
  for (const q of quizData) {
    if (!appRwMap.has(q.id)) {
      warnings.push(`[WARN] Question #${q.id} is in quiz-questions.json but not in assets/questions.json.`);
    }
  }
  for (const appQ of appRwQuestions) {
    if (!seenIds.has(appQ.id)) {
      warnings.push(`[WARN] Question #${appQ.id} is in assets/questions.json but not in quiz-questions.json.`);
    }
  }

  // Check fatal conditions
  if (duplicateIds.length > 0) {
    fatalErrors.push(`Duplicate question IDs found: ${duplicateIds.join(", ")}`);
  }
  if (invalidOptionCountIds.length > 0) {
    fatalErrors.push(
      `Questions with option count not in [3, 4]: ${invalidOptionCountIds.join(", ")}`
    );
  }
  if (invalidCorrectKeyIds.length > 0) {
    fatalErrors.push(
      `Questions where 'correct' is not a valid option letter: ${invalidCorrectKeyIds.join(", ")}`
    );
  }
  if (mismatchCorrectIds.length > 0) {
    fatalErrors.push(
      `Questions where 'correct' differs from assets/questions.json: ${mismatchCorrectIds
        .map((m) => `#${m.id} (quiz='${m.quiz}', app='${m.app}')`)
        .join(", ")}`
    );
  }
  if (missingImages.length > 0) {
    fatalErrors.push(
      `Missing referenced image files in assets/images/: ${missingImages
        .map((m) => `#${m.id} -> ${m.image}`)
        .join(", ")}`
    );
  }

  if (fatalErrors.length > 0) {
    console.error("==================================================");
    console.error("FATAL ERRORS in prepare-questions:");
    for (const err of fatalErrors) {
      console.error(`- ${err}`);
    }
    console.error("==================================================");
    process.exit(1);
  }

  // Write output files
  fs.mkdirSync(genDir, { recursive: true });
  fs.mkdirSync(runtimeImagesDir, { recursive: true });

  const outRwPath = path.join(genDir, "questions.json");
  const outEnPath = path.join(genDir, "questions_en.json");

  fs.writeFileSync(outRwPath, JSON.stringify(generatedRw, null, 2), "utf8");
  fs.writeFileSync(outEnPath, JSON.stringify(generatedEn, null, 2), "utf8");

  // Copy images
  for (const q of quizData) {
    if (q.image) {
      const srcPath = path.join(assetsImagesDir, q.image);
      const destPath = path.join(runtimeImagesDir, q.image);
      fs.copyFileSync(srcPath, destPath);
    }
  }

  // Print Summary
  console.log("==================================================");
  console.log("Question Bank Preparation Summary");
  console.log("==================================================");
  console.log(`Total questions:        ${quizData.length}`);
  console.log(`Kinyarwanda (rw):       ${rwCount}`);
  console.log(`English (en):           ${enCount}`);
  console.log(`With image:             ${withImageCount}`);
  console.log(`Warnings:               ${warnings.length}`);
  if (warnings.length > 0) {
    console.log("--- Warnings ---");
    for (const w of warnings) {
      console.log(w);
    }
  }
  console.log("==================================================");
  console.log("Successfully generated:");
  console.log(`- ${outRwPath}`);
  console.log(`- ${outEnPath}`);
  console.log(`- Copied ${withImageCount} images to ${runtimeImagesDir}`);
}

run();
