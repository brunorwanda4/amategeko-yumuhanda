import type { Metadata } from "next";
import Link from "next/link";
import { CirclePlayIcon, Clock3Icon, TriangleAlertIcon } from "lucide-react";

import { HomeFeatures } from "@/components/home-features";
import { ModeCard } from "@/components/mode-card";
import { QuizPreview } from "@/components/quiz-preview";
import { Button } from "@/components/ui/button";
import { SITE_DESCRIPTION, SITE_NAME } from "@/lib/config";

export const metadata: Metadata = {
  title: { absolute: SITE_NAME },
  description: SITE_DESCRIPTION,
  openGraph: {
    title: SITE_NAME,
    description: SITE_DESCRIPTION,
    url: "/",
    images: [{ url: "/logo.png", alt: `${SITE_NAME} logo` }],
  },
};

const modes = [
  {
    name: "Easy",
    description: "Learn at your own pace, with the correct answer shown right away.",
    badge: "Learn",
    badgeClassName: "border-success bg-success-soft text-success-foreground",
    Icon: CirclePlayIcon,
    bars: 1,
  },
  {
    name: "Medium",
    description: "Take a timed practice exam with familiar controls.",
    badge: "Timed",
    badgeClassName: "border-warning bg-warning-soft text-warning-foreground",
    Icon: Clock3Icon,
    bars: 2,
  },
  {
    name: "Hard",
    description: "Stricter rules, less time, and no going back to change answers.",
    badge: "Strict",
    badgeClassName: "border-error bg-error-soft text-error-foreground",
    Icon: TriangleAlertIcon,
    bars: 3,
  },
] as const;

export default function WebsiteHomePage() {
  return (
    <main>
      <section className="relative overflow-hidden border-b">
        <div className="hero-grid pointer-events-none absolute inset-0" aria-hidden="true" />
        <div className="relative mx-auto max-w-6xl px-5 pt-20 sm:px-8 sm:pt-24">
          <div>
            <span className="inline-flex items-center gap-2 rounded-full border bg-card px-3 py-1 text-xs font-medium text-muted-foreground">
              <span className="size-1.5 rounded-full bg-success" aria-hidden="true" />
              Free, works offline on Windows and Android
            </span>
            <h1 className="mt-6 max-w-[12.5em] text-4xl font-semibold leading-[1.02] tracking-[-0.04em] sm:text-6xl lg:text-[4.75rem]">
              Pass the Rwanda driving theory test.{" "}
              <span className="text-muted-foreground">Study anywhere.</span>
            </h1>
            <p className="mt-5 max-w-[34em] text-base leading-7 text-muted-foreground sm:text-lg">
              Practice quizzes in English and Kinyarwanda. No ads, no sign-up, and no internet
              needed.
            </p>
            <div className="mt-8 flex flex-wrap gap-3">
              <Button asChild className="rounded-full" size="lg">
                <Link href="/download">Download</Link>
              </Button>
              <Button asChild className="rounded-full" size="lg" variant="outline">
                <Link href="/app">Open in browser</Link>
              </Button>
              <Button asChild className="rounded-full" size="lg" variant="outline">
                <Link href="/docs">Read the docs</Link>
              </Button>
            </div>
          </div>
          <div className="mx-auto mt-16 max-w-[920px]">
            <QuizPreview />
          </div>
        </div>
      </section>

      <section className="border-b">
        <div className="mx-auto max-w-6xl px-5 py-20 sm:px-8 sm:py-24">
          <h2 className="text-3xl font-semibold tracking-tight sm:text-4xl">Practice your way</h2>
          <p className="mt-3 max-w-2xl text-muted-foreground">
            Three modes, from relaxed learning to exam pressure.
          </p>
          <div className="mt-12 grid gap-4 md:grid-cols-3">
            {modes.map((mode) => (
              <ModeCard {...mode} key={mode.name} />
            ))}
          </div>
        </div>
      </section>

      <section className="border-b">
        <div className="mx-auto max-w-6xl px-5 py-20 sm:px-8 sm:py-24">
          <h2 className="text-3xl font-semibold tracking-tight sm:text-4xl">Why this app</h2>
          <p className="mt-3 max-w-2xl text-muted-foreground">
            Built for learners who study on the bus, at home, or anywhere the signal is weak.
          </p>
          <div className="mt-12">
            <HomeFeatures />
          </div>
        </div>
      </section>

      <section>
        <div className="mx-auto flex max-w-6xl flex-col gap-8 px-5 py-20 sm:flex-row sm:items-center sm:justify-between sm:px-8 sm:py-24">
          <div>
            <h2 className="text-3xl font-semibold tracking-tight sm:text-4xl">
              Ready to practice?
            </h2>
            <p className="mt-2 text-muted-foreground">Free for Windows and Android.</p>
          </div>
          <div className="flex flex-wrap gap-3">
            <Button asChild className="rounded-full" size="lg">
              <Link href="/download">Download</Link>
            </Button>
            <Button asChild className="rounded-full" size="lg" variant="outline">
              <Link href="/app">Open in browser</Link>
            </Button>
            <Button asChild className="rounded-full" size="lg" variant="outline">
              <Link href="/docs">Docs</Link>
            </Button>
          </div>
        </div>
      </section>
    </main>
  );
}
