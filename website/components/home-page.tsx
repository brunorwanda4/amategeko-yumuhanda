import type { Metadata } from "next";
import Image from "next/image";
import Link from "next/link";

import { GiftIcon, LanguagesIcon, OfflineIcon, PlayIcon } from "@/components/icons";
import { HeroEffect } from "@/components/hero-effect";
import { ScreenshotFrame } from "@/components/screenshot-frame";
import { SITE_DESCRIPTION, SITE_NAME } from "@/lib/config";

export const metadata: Metadata = {
  title: { absolute: SITE_NAME },
  description: SITE_DESCRIPTION,
  openGraph: { title: SITE_NAME, description: SITE_DESCRIPTION, url: "/", images: [{ url: "/logo.png", alt: `${SITE_NAME} logo` }] },
};

const modes = [
  { name: "Easy", description: "Learn at your own pace, with the answer shown at once.", badge: "Learn", badgeClass: "bg-success-soft text-success-foreground" },
  { name: "Medium", description: "Take a timed practice exam with familiar controls.", badge: "Timed", badgeClass: "bg-warning-soft text-warning-foreground" },
  { name: "Hard", description: "Use stricter rules, less time, and no going back.", badge: "Strict", badgeClass: "bg-error-soft text-error-foreground" },
] as const;

const reasons = [
  [GiftIcon, "Free", "No subscriptions or locked features."],
  [OfflineIcon, "Works offline", "Study without mobile data or Wi-Fi."],
  [LanguagesIcon, "English + Kinyarwanda", "Switch to the language that helps you learn."],
] as const;

export default function WebsiteHomePage() {
  return (
    <main>
      <section className="hero-noise relative overflow-hidden text-white">
        <HeroEffect />
        <div className="relative mx-auto max-w-6xl px-5 pb-0 pt-20 sm:px-8 sm:pt-28">
          <div className="relative z-10 max-w-3xl">
            <span className="inline-flex rounded-full border border-white/20 bg-white/5 px-4 py-2 text-xs font-medium text-white/80 backdrop-blur">Free · Works offline · Windows &amp; Android</span>
            <h1 className="mt-7 text-balance text-5xl font-semibold tracking-[-0.05em] sm:text-7xl">
              <span className="block">Study Amategeko y&apos;Umuhanda,</span>
              <span className="mt-2 block"><span className="rounded-full bg-white px-4 py-0.5 text-black dark:bg-black dark:text-white dark:ring-1 dark:ring-white/25">anywhere.</span></span>
            </h1>
            <p className="mt-7 max-w-xl text-base leading-7 text-white/70 sm:text-lg">Practice quizzes in English and Kinyarwanda. No ads. No internet needed.</p>
            <div className="mt-8 flex flex-wrap gap-3">
              <Link href="/download" className="inline-flex min-h-11 items-center justify-center rounded-lg bg-white px-5 text-sm font-semibold text-black hover:bg-neutral-200">Download</Link>
              <Link href="/docs" className="inline-flex min-h-11 items-center justify-center rounded-lg border border-white/25 bg-white/5 px-5 text-sm font-semibold text-white hover:bg-white/10">Read the docs</Link>
            </div>
          </div>
          <Image src="/logo.png" alt="" width={360} height={360} className="pixel-logo pointer-events-none absolute right-8 top-20 hidden opacity-20 lg:block" />
          <div className="relative z-10 mt-20 h-56 sm:h-80 lg:h-96">
            <ScreenshotFrame src="/screenshots/desktop-home.png" alt="Desktop home screen of Amategeko y'Umuhanda" priority className="absolute inset-x-0 top-0 mx-auto max-w-4xl rounded-b-none" />
            <ScreenshotFrame src="/screenshots/mobile-home.png" alt="Mobile home screen of Amategeko y'Umuhanda" kind="phone" priority className="absolute -bottom-14 right-2 hidden w-44 sm:block lg:right-10 lg:w-56" />
          </div>
        </div>
      </section>

      <section className="mx-auto max-w-6xl px-5 py-20 sm:px-8 sm:py-28">
        <h2 className="text-3xl font-semibold tracking-tight sm:text-4xl">Practice your way</h2>
        <div className="mt-8 grid gap-4 md:grid-cols-3">
          {modes.map((mode) => (
            <article key={mode.name} className="rounded-xl border bg-card p-6 text-card-foreground">
              <div className="flex items-center justify-between gap-4"><PlayIcon className="size-9" /><span className={`rounded-full px-3 py-1 text-xs font-semibold ${mode.badgeClass}`}>{mode.badge}</span></div>
              <h3 className="mt-8 text-xl font-semibold">{mode.name}</h3>
              <p className="mt-2 text-sm leading-6 text-muted-foreground">{mode.description}</p>
            </article>
          ))}
        </div>
      </section>

      <section className="border-y bg-muted/35">
        <div className="mx-auto max-w-6xl px-5 py-20 sm:px-8">
          <h2 className="text-3xl font-semibold tracking-tight">Why this app</h2>
          <div className="mt-10 grid gap-8 md:grid-cols-3">
            {reasons.map(([Icon, title, description]) => (
              <article key={title} className="flex gap-4"><Icon className="mt-0.5 size-7 shrink-0" /><div><h3 className="font-semibold">{title}</h3><p className="mt-1 text-sm leading-6 text-muted-foreground">{description}</p></div></article>
            ))}
          </div>
        </div>
      </section>

      <section className="mx-auto max-w-6xl px-5 py-20 sm:px-8 sm:py-28">
        <div className="grid items-center gap-8 lg:grid-cols-[1fr_0.35fr]">
          <ScreenshotFrame src="/screenshots/desktop-home.png" alt="Desktop home screen of Amategeko y'Umuhanda" />
          <div className="grid grid-cols-2 gap-4 lg:grid-cols-1">
            <ScreenshotFrame src="/screenshots/mobile-home.png" alt="Mobile home screen of Amategeko y'Umuhanda" kind="phone" />
            <ScreenshotFrame src="/screenshots/mobile-stats.png" alt="Mobile statistics screen of Amategeko y'Umuhanda" kind="phone" />
          </div>
        </div>
      </section>

      <section className="border-t">
        <div className="mx-auto flex max-w-6xl flex-col gap-6 px-5 py-20 sm:flex-row sm:items-center sm:justify-between sm:px-8">
          <h2 className="text-3xl font-semibold tracking-tight">Ready to practice?</h2>
          <div className="flex gap-3">
            <Link href="/download" className="inline-flex min-h-11 items-center rounded-lg bg-primary px-5 text-sm font-semibold text-primary-foreground">Download</Link>
            <Link href="/docs" className="inline-flex min-h-11 items-center rounded-lg border px-5 text-sm font-semibold">Docs</Link>
          </div>
        </div>
      </section>
    </main>
  );
}
