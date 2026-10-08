import type { Metadata } from "next";
import Image from "next/image";
import Link from "next/link";

import { AUTHOR_NAME, AUTHOR_URL, REPO_URL, SITE_NAME } from "@/lib/config";

const description = "Why Rwanda Bruno built Amategeko y'Umuhanda.";

export const metadata: Metadata = {
  title: "About",
  description,
  openGraph: { title: `About | ${SITE_NAME}`, description, url: "/about", images: [{ url: "/logo.png", alt: `${SITE_NAME} logo` }] },
};

export default function AboutPage() {
  return (
    <main className="mx-auto max-w-3xl px-5 py-16 sm:px-8 sm:py-24">
      <h1 className="text-4xl font-semibold tracking-tight sm:text-5xl">Why I built this</h1>
      <div className="mt-8 flex max-w-2xl flex-col gap-5 text-base leading-8 text-muted-foreground sm:text-lg">
        <p>I wanted to study Amategeko y&apos;Umuhanda, but reading was boring. Other websites were not good either: you have to pay, and they only work online. They do not work offline.</p>
        <p>So I built my own. It was my first time studying, and I studied for only 6 hours, and I scored 17/20 on the exam. It took me 2 days to build this app. After I passed, I added more features to help other people.</p>
        <p>It is free, it works offline, and you can use it too.</p>
      </div>
      <section className="mt-14 rounded-xl border bg-card p-6 text-card-foreground sm:p-8" aria-label="Author">
        <div className="flex flex-col gap-6 sm:flex-row sm:items-center">
          <Image src="https://github.com/brunorwanda4.png" alt="Rwanda Bruno" width={88} height={88} unoptimized className="rounded-full border" />
          <div className="flex-1"><h2 className="text-xl font-semibold">{AUTHOR_NAME}</h2><p className="mt-1 text-sm text-muted-foreground">Built this app</p></div>
          <a href={AUTHOR_URL} rel="noopener noreferrer" target="_blank" className="inline-flex min-h-11 items-center justify-center rounded-lg bg-primary px-5 text-sm font-semibold text-primary-foreground">GitHub</a>
        </div>
      </section>
      <nav aria-label="Project links" className="mt-6 flex gap-5 text-sm">
        <a href={REPO_URL} rel="noopener noreferrer" target="_blank" className="underline underline-offset-4">Source code</a>
        <Link href="/docs" className="underline underline-offset-4">Documentation</Link>
      </nav>
    </main>
  );
}
