import type { Metadata } from "next";
import Image from "next/image";
import Link from "next/link";
import {
  FaGithub,
  FaInstagram,
  FaLinkedin,
  FaWhatsapp,
  FaXTwitter,
} from "react-icons/fa6";

import { Button } from "@/components/ui/button";
import {
  AUTHOR_INSTAGRAM_URL,
  AUTHOR_LINKEDIN_URL,
  AUTHOR_NAME,
  AUTHOR_TWITTER_URL,
  AUTHOR_URL,
  AUTHOR_WHATSAPP_URL,
  REPO_URL,
  SITE_NAME,
} from "@/lib/config";
import { cn } from "@/lib/utils";

const description = "Why Rwanda Bruno built Amategeko y'Umuhanda.";

export const metadata: Metadata = {
  title: "About",
  description,
  openGraph: {
    title: `About | ${SITE_NAME}`,
    description,
    url: "/about",
    images: [{ url: "/logo.png", alt: `${SITE_NAME} logo` }],
  },
};

const stats = [
  { value: "6 hrs", label: "Studied before the exam", isHighlight: false },
  { value: "17/20", label: "Exam score", isHighlight: true },
  { value: "2 days", label: "To build the first version", isHighlight: false },
] as const;

const steps = [
  {
    step: "Step 1",
    title: "Reading was boring",
    description: "Paid, online-only websites were not a good way to study.",
    ok: false,
  },
  {
    step: "Step 2",
    title: "Built my own app",
    description: "Two days to make a free, offline quiz app.",
    ok: false,
  },
  {
    step: "Step 3",
    title: "Passed with 17/20",
    description: "After only 6 hours of studying with it.",
    ok: true,
  },
  {
    step: "Step 4",
    title: "Added more features",
    description: "Quiz modes, stats, and two languages to help other people.",
    ok: false,
  },
] as const;

const authorSocials = [
  {
    name: "GitHub",
    handle: "brunorwanda4",
    href: AUTHOR_URL,
    Icon: FaGithub,
  },
  {
    name: "X (Twitter)",
    handle: "@rwanda_bruno",
    href: AUTHOR_TWITTER_URL,
    Icon: FaXTwitter,
  },
  {
    name: "LinkedIn",
    handle: "Rwanda Bruno",
    href: AUTHOR_LINKEDIN_URL,
    Icon: FaLinkedin,
  },
  {
    name: "Instagram",
    handle: "@bruno_rwanda",
    href: AUTHOR_INSTAGRAM_URL,
    Icon: FaInstagram,
  },
  {
    name: "WhatsApp",
    handle: "+250 781 419 525",
    href: AUTHOR_WHATSAPP_URL,
    Icon: FaWhatsapp,
  },
] as const;

export default function AboutPage() {
  return (
    <main className="mx-auto max-w-[720px] px-6 pt-12 pb-24 sm:pt-20 sm:pb-28">
      {/* Eyebrow */}
      <div className="mb-4 flex items-center gap-1.5 text-[13px] font-medium text-muted-foreground">
        <span>About</span>
        <span className="text-muted-foreground/60">/</span>
        <span>The story</span>
      </div>

      {/* Main Title */}
      <h1 className="mb-8 text-4xl font-semibold tracking-[-0.045em] leading-[1.05] sm:text-5xl lg:text-[60px]">
        Why I built this
      </h1>

      {/* Story Narrative */}
      <div className="space-y-5 text-base leading-[1.75] text-muted-foreground sm:text-[19px]">
        <p>
          I wanted to study Amategeko y&apos;Umuhanda, but reading was boring.
          Other websites were not good either:{" "}
          <strong className="font-semibold text-foreground">
            you have to pay, and they only work online.
          </strong>{" "}
          They do not work offline.
        </p>
        <p>
          So I built my own. It was my first time studying, and I studied for
          only 6 hours, and I scored 17/20 on the exam. It took me 2 days to
          build this app. After I passed, I added more features to help other
          people.
        </p>
        <p>
          <strong className="font-semibold text-foreground">
            It is free, it works offline, and you can use it too.
          </strong>
        </p>
      </div>

      {/* Stats Cards */}
      <div className="my-10 grid grid-cols-1 gap-3 sm:grid-cols-3">
        {stats.map((stat) => (
          <div
            key={stat.label}
            className="rounded-[14px] border bg-card p-5 transition-colors"
          >
            <span
              className={cn(
                "block text-4xl font-semibold leading-[1.1] tracking-[-0.04em]",
                stat.isHighlight ? "text-success" : "text-foreground"
              )}
            >
              {stat.value}
            </span>
            <span className="mt-1.5 block text-[13px] leading-snug text-muted-foreground">
              {stat.label}
            </span>
          </div>
        ))}
      </div>

      {/* Timeline Section */}
      <section className="mt-16">
        <h2 className="mb-6 text-2xl font-semibold tracking-[-0.03em]">
          How it happened
        </h2>
        <ol
          className="relative ml-2 list-none border-l pl-7"
          aria-label="Timeline of how the app was built"
        >
          {steps.map((step) => (
            <li key={step.step} className="relative pb-7 last:pb-0">
              <span
                className={cn(
                  "absolute -left-[33.5px] top-[7px] size-[11px] rounded-full border-2",
                  step.ok
                    ? "border-success bg-success"
                    : "border-muted-foreground/60 bg-background"
                )}
                aria-hidden="true"
              />
              <small className="font-mono text-xs text-muted-foreground">
                {step.step}
              </small>
              <h3 className="mt-0.5 text-base font-semibold tracking-[-0.01em] text-foreground">
                {step.title}
              </h3>
              <p className="mt-0.5 text-[15px] leading-normal text-muted-foreground">
                {step.description}
              </p>
            </li>
          ))}
        </ol>
      </section>

      {/* Author Card */}
      <section
        className="mt-16 rounded-[18px] border bg-card p-6 sm:p-7"
        aria-label="Author"
      >
        <div className="flex flex-col gap-5 sm:flex-row sm:items-center">
          <Image
            src="https://github.com/brunorwanda4.png"
            alt={AUTHOR_NAME}
            width={76}
            height={76}
            unoptimized
            className="size-[76px] shrink-0 rounded-full border border-border object-cover"
          />
          <div className="flex-1">
            <h3 className="text-[22px] font-semibold tracking-[-0.02em] leading-tight text-foreground">
              {AUTHOR_NAME}
            </h3>
            <p className="mt-1 text-sm text-muted-foreground">Built this app</p>
          </div>
          <Button
            asChild
            className="w-full rounded-full px-5 font-medium sm:ml-auto sm:w-auto"
            size="default"
          >
            <a href={AUTHOR_URL} rel="noopener noreferrer" target="_blank">
              <FaGithub className="size-4" aria-hidden="true" />
              <span>GitHub</span>
            </a>
          </Button>
        </div>

        {/* Social Links */}
        <div className="mt-5 border-t border-border pt-5">
          <p className="mb-3 text-xs font-medium tracking-wider text-muted-foreground uppercase">
            Connect with me
          </p>
          <div className="flex flex-wrap gap-2">
            {authorSocials.map((social) => (
              <a
                key={social.name}
                href={social.href}
                target="_blank"
                rel="noopener noreferrer"
                title={`${social.name}: ${social.handle}`}
                aria-label={`${social.name}: ${social.handle}`}
                className="group inline-flex items-center gap-2 rounded-full border border-border bg-background px-3.5 py-1.5 text-xs font-medium text-muted-foreground transition-all hover:border-foreground/30 hover:bg-accent hover:text-foreground hover:shadow-xs"
              >
                <social.Icon
                  className="size-3.5 shrink-0 transition-transform group-hover:scale-110"
                  aria-hidden="true"
                />
                <span>{social.name}</span>
              </a>
            ))}
          </div>
        </div>
      </section>

      {/* Project Links */}
      <nav aria-label="Project links" className="mt-6 flex flex-wrap gap-6 text-sm">
        <a
          href={REPO_URL}
          rel="noopener noreferrer"
          target="_blank"
          className="border-b border-foreground pb-0.5 font-medium transition-opacity hover:opacity-70"
        >
          Source code
        </a>
        <Link
          href="/docs"
          className="border-b border-foreground pb-0.5 font-medium transition-opacity hover:opacity-70"
        >
          Documentation
        </Link>
      </nav>

      {/* Call to Action */}
      <section
        className="mt-18 flex flex-col gap-5 rounded-[18px] border bg-card p-7 sm:flex-row sm:items-center sm:justify-between sm:p-8"
        aria-label="Call to action"
      >
        <div>
          <h2 className="text-2xl font-semibold tracking-tight text-foreground">
            Ready to practice?
          </h2>
          <p className="mt-1 text-[15px] text-muted-foreground">
            Free, offline, and no ads.
          </p>
        </div>
        <Button
          asChild
          className="w-full rounded-full px-6 font-medium sm:w-auto"
          size="lg"
        >
          <Link href="/download">Download</Link>
        </Button>
      </section>
    </main>
  );
}
