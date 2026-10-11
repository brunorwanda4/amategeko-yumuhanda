"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import {
  ChevronLeftIcon,
  ChevronRightIcon,
  DownloadIcon,
  ExternalLinkIcon,
  GlobeIcon,
  RotateCcwIcon,
  ShuffleIcon,
} from "lucide-react";

import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { getRandomQuestions, initialPreviewQuestions, type QuizQuestion } from "@/lib/quiz-data";

const uiCopy = {
  en: {
    questionLabel: "Question",
    of: "of",
    tapPrompt: "Tap an option to check if it is correct",
    previous: "Previous",
    next: "Next",
    finish: "Finish and View Results",
    shuffle: "Shuffle",
    resultsTitle: "Results",
    passed: "Passed",
    failed: "Failed",
    modeDesc: "Practice Demo · 10 Questions",
    correctTag: "Correct",
    wrongTag: "Wrong",
    unansweredTag: "Unanswered",
    ctaTitle: "Ready for your official driving test?",
    ctaDesc: "Practice all 400+ official questions with realistic timer exams, offline study on Windows and Android, or try the full web version right in your browser.",
    downloadApp: "Download Application",
    openWebApp: "Open Full Web App",
    tryAgain: "Try again",
    newSet: "Try 10 new questions",
  },
  rw: {
    questionLabel: "Ikibazo cya",
    of: "kuri",
    tapPrompt: "Kanda ku gisubizo kimwe ngo urebe niba ari cyo",
    previous: "Ibibanza",
    next: "Ibikurikira",
    finish: "Soza urebe amanota",
    shuffle: "Hindura ibibazo",
    resultsTitle: "Ibisubizo",
    passed: "Watsinze",
    failed: "Watsinzwe",
    modeDesc: "Imyitozo y'Icyitegererezo · Ibibazo 10",
    correctTag: "Iby'ukuri",
    wrongTag: "Ibyibeshye",
    unansweredTag: "Bidasubijwe",
    ctaTitle: "Witeguye gutsindira perimi yawe?",
    ctaDesc: "Iga ibibazo byose 404 by'amategeko y'umuhanda nta interineti ikenewe kuri Windows na Android, cyangwa koresha porogaramu yose ako kanya mu mushakisha wawe.",
    downloadApp: "Kura porogaramu (Download)",
    openWebApp: "Fungura porogaramu ku rubuga",
    tryAgain: "Ongera usubiremo",
    newSet: "Fata ibindi bibazo 10 bishya",
  },
} as const;

export function QuizPreview() {
  const [questions, setQuestions] = useState<QuizQuestion[]>(initialPreviewQuestions);
  const [currentIndex, setCurrentIndex] = useState<number>(0);
  const [answers, setAnswers] = useState<Record<number, number>>({});
  const [language, setLanguage] = useState<"en" | "rw">("en");
  const [showResults, setShowResults] = useState<boolean>(false);

  // Load 10 random questions on client mount
  useEffect(() => {
    setQuestions(getRandomQuestions(10));
  }, []);

  const t = uiCopy[language];
  const total = questions.length;
  const currentQ = questions[currentIndex] || questions[0];
  const localized = currentQ ? (currentQ[language] || currentQ.en) : null;

  const correctIndex = currentQ
    ? currentQ.correct === "a"
      ? 0
      : currentQ.correct === "b"
      ? 1
      : currentQ.correct === "c"
      ? 2
      : 3
    : 0;

  const currentAnswer = answers[currentIndex];
  const isAnswered = currentAnswer !== undefined;

  // Score statistics
  const answeredCount = Object.keys(answers).length;
  const correctCount = Object.entries(answers).reduce((acc, [qIdxStr, chosen]) => {
    const q = questions[Number(qIdxStr)];
    if (!q) return acc;
    const correct = q.correct === "a" ? 0 : q.correct === "b" ? 1 : q.correct === "c" ? 2 : 3;
    return acc + (chosen === correct ? 1 : 0);
  }, 0);
  const wrongCount = answeredCount - correctCount;
  const unansweredCount = total - answeredCount;
  const scorePercent = total > 0 ? Math.round((correctCount / total) * 100) : 0;
  const isPassed = scorePercent >= 70;

  function handleSelectOption(optionIndex: number) {
    if (isAnswered) return;
    setAnswers((prev) => ({
      ...prev,
      [currentIndex]: optionIndex,
    }));
  }

  function handleNext() {
    if (currentIndex < total - 1) {
      setCurrentIndex((prev) => prev + 1);
    } else {
      setShowResults(true);
    }
  }

  function handlePrevious() {
    if (currentIndex > 0) {
      setCurrentIndex((prev) => prev - 1);
    }
  }

  function handleNewRandomSet() {
    setQuestions(getRandomQuestions(10));
    setAnswers({});
    setCurrentIndex(0);
    setShowResults(false);
  }

  function handleTryAgain() {
    setAnswers({});
    setCurrentIndex(0);
    setShowResults(false);
  }

  return (
    <div className="overflow-hidden rounded-2xl border border-zinc-800/80 bg-[#0c0c0e] text-zinc-100 shadow-2xl transition-all">
      {/* Top Header Bar */}
      <div className="flex h-14 items-center justify-between border-b border-zinc-800/80 px-4 sm:px-6 bg-zinc-950/60">
        <div className="flex items-center gap-3">
          {!showResults ? (
            <span className="text-sm sm:text-base font-semibold text-white tracking-tight">
              {t.questionLabel} {currentIndex + 1} {t.of} {total}
            </span>
          ) : (
            <span className="text-base font-bold text-white tracking-tight">
              {t.resultsTitle}
            </span>
          )}
        </div>

        {/* Right side controls: Language toggle and shuffle */}
        <div className="flex items-center gap-2">
          {/* Language Switcher replaces the timer */}
          <button
            className="flex h-8 items-center gap-1.5 rounded-lg border border-zinc-800 bg-zinc-900/80 px-2.5 text-xs font-medium text-zinc-300 hover:border-zinc-700 hover:text-white transition-colors cursor-pointer"
            onClick={() => setLanguage(language === "en" ? "rw" : "en")}
            title="Change language"
            type="button"
          >
            <GlobeIcon className="size-3.5 text-zinc-400" />
            <span>{language === "en" ? "English" : "Kinyarwanda"}</span>
          </button>

          {!showResults && (
            <button
              className="flex h-8 items-center gap-1.5 rounded-lg border border-zinc-800 bg-zinc-900/80 px-2.5 text-xs font-medium text-zinc-300 hover:border-zinc-700 hover:text-white transition-colors cursor-pointer"
              onClick={handleNewRandomSet}
              title="Get new random 10 questions"
              type="button"
            >
              <ShuffleIcon className="size-3.5 text-zinc-400" />
              <span className="hidden sm:inline">{t.shuffle}</span>
            </button>
          )}
        </div>
      </div>

      {/* Segmented Progress Bar (Directly below header) */}
      {!showResults && (
        <div className="flex gap-1.5 px-4 sm:px-6 pt-3 pb-2 bg-zinc-950/30">
          {questions.map((q, idx) => {
            const ans = answers[idx];
            const qCorrect = q.correct === "a" ? 0 : q.correct === "b" ? 1 : q.correct === "c" ? 2 : 3;
            const isCurrent = currentIndex === idx;
            const isCor = ans !== undefined && ans === qCorrect;
            const isWro = ans !== undefined && ans !== qCorrect;

            return (
              <button
                aria-label={`Question ${idx + 1}`}
                className={cn(
                  "h-1 rounded-full flex-1 transition-all cursor-pointer",
                  isCurrent
                    ? "bg-white ring-1 ring-white/50"
                    : isCor
                    ? "bg-emerald-500 hover:bg-emerald-400"
                    : isWro
                    ? "bg-rose-500 hover:bg-rose-400"
                    : "bg-zinc-800 hover:bg-zinc-700"
                )}
                key={q.id}
                onClick={() => setCurrentIndex(idx)}
                type="button"
              />
            );
          })}
        </div>
      )}

      {/* Main Container */}
      {!showResults ? (
        /* Quiz Question Screen */
        <div className="p-5 sm:p-8 md:p-10">
          <div className={cn(
            "mx-auto max-w-4xl",
            currentQ.image ? "grid gap-8 md:grid-cols-[1.1fr_1.3fr] items-center" : "max-w-3xl"
          )}>
            {/* If question has an image: Left side illustration card */}
            {currentQ.image && (
              <div className="flex min-h-[260px] sm:min-h-[320px] items-center justify-center rounded-2xl border border-zinc-800/80 bg-[#141417] p-6 shadow-inner">
                <img
                  alt={`Road sign for question ${currentQ.id}`}
                  className="max-h-60 sm:max-h-72 w-auto object-contain transition-transform hover:scale-105"
                  src={`/questions/${currentQ.image}`}
                />
              </div>
            )}

            {/* Right side (or full width): Question and Options */}
            <div className="flex flex-col justify-center">
              <h2 className="text-xl sm:text-2xl font-bold tracking-tight text-white leading-snug">
                {localized?.question}
              </h2>
              <p className="mt-2 text-sm text-zinc-400">
                {t.tapPrompt}
              </p>

              {/* Options */}
              <div className="mt-6 grid gap-3">
                {localized?.options.map((option, optIdx) => {
                  const isChosen = currentAnswer === optIdx;
                  const isOptCorrect = isAnswered && optIdx === correctIndex;
                  const isOptWrong = isChosen && !isOptCorrect;

                  return (
                    <button
                      aria-pressed={isChosen}
                      className={cn(
                        "group flex w-full items-center gap-3.5 rounded-xl border p-3.5 text-left transition-all duration-150",
                        !isAnswered && "border-zinc-800/80 bg-zinc-900/60 hover:bg-zinc-800/80 hover:border-zinc-700 text-zinc-200 cursor-pointer",
                        isOptCorrect && "border-emerald-500 bg-emerald-950/40 text-emerald-200",
                        isOptWrong && "border-rose-500 bg-rose-950/40 text-rose-200",
                        isAnswered && !isChosen && !isOptCorrect && "border-zinc-850 bg-zinc-900/30 text-zinc-500 opacity-60 cursor-default"
                      )}
                      disabled={isAnswered}
                      key={optIdx}
                      onClick={() => handleSelectOption(optIdx)}
                      type="button"
                    >
                      <span
                        className={cn(
                          "flex size-7 shrink-0 items-center justify-center rounded-lg border font-mono text-xs font-bold transition-colors",
                          isOptCorrect
                            ? "border-emerald-400 bg-emerald-500 text-white"
                            : isOptWrong
                            ? "border-rose-400 bg-rose-500 text-white"
                            : "border-zinc-800 bg-zinc-950 text-zinc-400 group-hover:border-zinc-700 group-hover:text-zinc-200"
                        )}
                      >
                        {String.fromCharCode(65 + optIdx)}
                      </span>
                      <span className="flex-1 text-sm font-medium leading-relaxed">
                        {option}
                      </span>
                    </button>
                  );
                })}
              </div>
            </div>
          </div>

          {/* Bottom Navigation */}
          <div className="mx-auto mt-10 flex max-w-4xl items-center justify-between border-t border-zinc-800/80 pt-5">
            <Button
              className="gap-1.5 rounded-xl border-zinc-800 bg-zinc-900 text-zinc-300 hover:bg-zinc-800 hover:text-white cursor-pointer"
              disabled={currentIndex === 0}
              onClick={handlePrevious}
              size="default"
              variant="outline"
            >
              <ChevronLeftIcon className="size-4" />
              <span>{t.previous}</span>
            </Button>

            <Button
              className={cn(
                "gap-1.5 rounded-xl font-medium transition-all cursor-pointer",
                currentIndex === total - 1
                  ? "bg-emerald-600 hover:bg-emerald-500 text-white shadow-lg shadow-emerald-900/30"
                  : isAnswered
                  ? "bg-white text-black hover:bg-zinc-200"
                  : "border-zinc-800 bg-zinc-900 text-zinc-300 hover:bg-zinc-800 hover:text-white"
              )}
              onClick={handleNext}
              size="default"
              variant={currentIndex === total - 1 || isAnswered ? "default" : "outline"}
            >
              <span>{currentIndex === total - 1 ? t.finish : t.next}</span>
              <ChevronRightIcon className="size-4" />
            </Button>
          </div>
        </div>
      ) : (
        /* Results Screen - Clean Hero Presentation without question list */
        <div className="p-6 sm:p-10 max-w-3xl mx-auto space-y-6">
          {/* Top Score Summary Card */}
          <div className="rounded-2xl border border-zinc-800/80 bg-[#121214] p-6 sm:p-8">
            <div className="flex flex-col sm:flex-row items-center gap-6 sm:gap-8">
              {/* Circular score badge (e.g. 8/10) */}
              <div
                className={cn(
                  "flex size-24 sm:size-28 shrink-0 flex-col items-center justify-center rounded-full border-4 shadow-lg",
                  isPassed
                    ? "border-emerald-500 bg-emerald-950/20 text-emerald-400 shadow-emerald-950/30"
                    : "border-rose-500 bg-rose-950/20 text-rose-400 shadow-rose-950/30"
                )}
              >
                <span className="text-2xl sm:text-3xl font-extrabold text-white leading-none">
                  {correctCount}
                </span>
                <span className="text-xs font-semibold text-zinc-400 mt-0.5">
                  / {total}
                </span>
              </div>

              {/* Status and Stats */}
              <div className="flex-1 text-center sm:text-left space-y-3">
                <div className="flex flex-wrap items-center justify-center sm:justify-start gap-3">
                  <span className="text-3xl sm:text-4xl font-extrabold text-white tracking-tight">
                    {scorePercent}%
                  </span>
                  <span
                    className={cn(
                      "rounded-md px-2.5 py-0.5 text-xs font-bold uppercase tracking-wider",
                      isPassed
                        ? "bg-emerald-500/20 text-emerald-400 border border-emerald-500/30"
                        : "bg-rose-500/20 text-rose-400 border border-rose-500/30"
                    )}
                  >
                    {isPassed ? t.passed : t.failed}
                  </span>
                </div>

                <p className="text-xs sm:text-sm text-zinc-400">
                  {t.modeDesc}
                </p>

                {/* Status Badges */}
                <div className="flex flex-wrap items-center justify-center sm:justify-start gap-2 pt-1">
                  <span className="rounded-md bg-emerald-950/60 border border-emerald-800/40 px-2.5 py-1 text-xs font-medium text-emerald-400">
                    {t.correctTag} {correctCount}
                  </span>
                  <span className="rounded-md bg-rose-950/60 border border-rose-800/40 px-2.5 py-1 text-xs font-medium text-rose-400">
                    {t.wrongTag} {wrongCount}
                  </span>
                  {unansweredCount > 0 && (
                    <span className="rounded-md bg-zinc-800/80 border border-zinc-700/40 px-2.5 py-1 text-xs font-medium text-zinc-400">
                      {t.unansweredTag} {unansweredCount}
                    </span>
                  )}
                </div>
              </div>
            </div>
          </div>

          {/* Action Callout Banner: Download Application or Open Web Demo */}
          <div className="relative overflow-hidden rounded-2xl border border-primary/30 bg-gradient-to-r from-primary/10 via-zinc-900 to-primary/5 p-6 sm:p-8">
            <div className="space-y-4 text-center sm:text-left">
              <div className="space-y-2">
                <h3 className="text-lg sm:text-xl font-bold text-white tracking-tight">
                  {t.ctaTitle}
                </h3>
                <p className="text-xs sm:text-sm text-zinc-300 leading-relaxed max-w-xl">
                  {t.ctaDesc}
                </p>
              </div>
              <div className="flex flex-col sm:flex-row items-center gap-3 pt-2">
                <Button asChild className="w-full sm:w-auto rounded-full bg-primary font-semibold text-primary-foreground shadow-lg hover:bg-primary/90" size="default">
                  <Link href="/download">
                    <DownloadIcon className="mr-2 size-4" />
                    <span>{t.downloadApp}</span>
                  </Link>
                </Button>
                <Button asChild className="w-full sm:w-auto rounded-full border-zinc-700 bg-zinc-900 text-zinc-200 hover:bg-zinc-800 hover:text-white" size="default" variant="outline">
                  <Link href="/app">
                    <ExternalLinkIcon className="mr-2 size-4" />
                    <span>{t.openWebApp}</span>
                  </Link>
                </Button>
              </div>
            </div>
          </div>

          {/* Retake & Next Set Controls */}
          <div className="flex flex-wrap items-center justify-center gap-3 pt-2">
            <Button
              className="gap-2 rounded-xl border-zinc-800 bg-zinc-900 text-zinc-300 hover:bg-zinc-800 hover:text-white cursor-pointer"
              onClick={handleTryAgain}
              variant="outline"
            >
              <RotateCcwIcon className="size-4" />
              <span>{t.tryAgain}</span>
            </Button>

            <Button
              className="gap-2 rounded-xl bg-white text-black hover:bg-zinc-200 font-medium cursor-pointer"
              onClick={handleNewRandomSet}
              variant="default"
            >
              <ShuffleIcon className="size-4" />
              <span>{t.newSet}</span>
            </Button>
          </div>
        </div>
      )}
    </div>
  );
}