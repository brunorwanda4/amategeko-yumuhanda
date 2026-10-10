"use client";

import { useState } from "react";
import { CheckIcon, XIcon } from "lucide-react";

import { Button } from "@/components/ui/button";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { cn } from "@/lib/utils";

const copy = {
  en: {
    question: "What must you do when you reach a STOP sign?",
    options: ["Stop completely", "Slow down", "Continue if clear"],
    correct: "Correct. Stop fully before continuing when the road is clear.",
    incorrect: "Not quite. This sign requires a complete stop.",
  },
  rw: {
    question: "Ugomba gukora iki ugeze ku kimenyetso cya STOP?",
    options: ["Guhagarara burundu", "Kugabanya umuvuduko", "Gukomeza inzira ibaye nyabagendwa"],
    correct: "Ni byo. Hagarara burundu mbere yo gukomeza inzira ibaye nyabagendwa.",
    incorrect: "Ongera ugerageze. Iki kimenyetso gisaba guhagarara burundu.",
  },
} as const;

export function QuizPreview() {
  const [language, setLanguage] = useState<keyof typeof copy>("en");
  const [answer, setAnswer] = useState<number | null>(null);
  const current = copy[language];

  function changeLanguage(value: string) {
    if (value === "en" || value === "rw") {
      setLanguage(value);
      setAnswer(null);
    }
  }

  return (
    <div className="overflow-hidden rounded-t-2xl border border-b-0 bg-card shadow-2xl shadow-foreground/5">
      <div className="flex h-11 items-center border-b bg-muted/50 px-4">
        <div className="flex gap-2" aria-hidden="true">
          <span className="size-2.5 rounded-full bg-error" />
          <span className="size-2.5 rounded-full bg-warning" />
          <span className="size-2.5 rounded-full bg-success" />
        </div>
        <span className="mx-auto pr-12 font-mono text-xs text-muted-foreground">Practice · Easy</span>
      </div>

      <div className="grid md:min-h-96 md:grid-cols-[13.75rem_1fr]">
        <aside className="hidden border-r bg-muted/20 p-5 md:block">
          <p className="text-xs font-medium text-muted-foreground">Progress</p>
          <div className="mt-3 grid grid-cols-5 gap-2">
            {Array.from({ length: 10 }, (_, index) => (
              <span
                className={cn(
                  "grid aspect-square place-items-center rounded-md border font-mono text-xs text-muted-foreground",
                  [0, 1, 3].includes(index) && "border-success/20 bg-success-soft text-success-foreground",
                  index === 2 && "border-error/20 bg-error-soft text-error-foreground",
                  index === 4 && "border-foreground text-foreground",
                )}
                key={index}
              >
                {index + 1}
              </span>
            ))}
          </div>

          <p className="mt-6 text-xs font-medium text-muted-foreground">Language</p>
          <ToggleGroup
            aria-label="Quiz preview language"
            className="mt-3 w-full"
            onValueChange={changeLanguage}
            type="single"
            value={language}
            variant="outline"
          >
            <ToggleGroupItem className="flex-1" value="en">English</ToggleGroupItem>
            <ToggleGroupItem className="flex-1" value="rw">Kinyarwanda</ToggleGroupItem>
          </ToggleGroup>

          <p className="mt-6 text-xs font-medium text-muted-foreground">Score</p>
          <p className="mt-2 text-2xl font-semibold tracking-tight">3 / 4</p>
        </aside>

        <div className="p-5 text-left sm:p-7 lg:p-8">
          <div className="mx-auto max-w-2xl">
            <div className="grid size-14 place-items-center bg-error font-mono text-xs font-bold text-error-foreground [clip-path:polygon(30%_0,70%_0,100%_30%,100%_70%,70%_100%,30%_100%,0_70%,0_30%)]">
              STOP
            </div>
            <h2 className="mt-5 text-xl font-semibold tracking-tight sm:text-2xl">{current.question}</h2>
            <div className="mt-5 grid gap-3">
              {current.options.map((option, index) => {
                const chosen = answer === index;
                const correct = answer !== null && index === 0;

                return (
                  <Button
                    aria-pressed={chosen}
                    className={cn(
                      "h-auto min-h-12 justify-start whitespace-normal bg-background px-4 py-3 text-left dark:bg-background",
                      correct && "border-success bg-success-soft text-success-foreground hover:bg-success-soft",
                      chosen && index !== 0 && "border-error bg-error-soft text-error-foreground hover:bg-error-soft",
                    )}
                    key={option}
                    onClick={() => answer === null && setAnswer(index)}
                    variant="outline"
                  >
                    <span className="grid size-6 shrink-0 place-items-center rounded-md border font-mono text-xs">
                      {String.fromCharCode(65 + index)}
                    </span>
                    <span>{option}</span>
                  </Button>
                );
              })}
            </div>

            {answer === null ? (
              <p className="mt-4 min-h-6 text-sm text-muted-foreground">Pick an answer to try it.</p>
            ) : (
              <div className={cn("mt-4 flex min-h-6 gap-3 text-sm", answer === 0 ? "text-success-foreground" : "text-error-foreground")}>
                {answer === 0 ? <CheckIcon className="size-5 shrink-0" /> : <XIcon className="size-5 shrink-0" />}
                <p>{answer === 0 ? current.correct : current.incorrect}</p>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
