"use client";

import { useEffect, useState } from "react";
import Link from "next/link";

import {
  DEFAULT_ANDROID_DOWNLOAD_URL,
  DEFAULT_WINDOWS_DOWNLOAD_URL,
  LATEST_RELEASE_API_URL,
  RELEASES_URL,
} from "@/lib/config";
import { cn } from "@/lib/utils";

type ReleaseAsset = { name: string; size: number; browser_download_url: string };
type LatestRelease = {
  tag_name: string;
  published_at: string;
  html_url: string;
  assets: ReleaseAsset[];
};
type CachedRelease = { savedAt: number; release: LatestRelease };

const CACHE_KEY = "amategeko-latest-release";
const TEN_MINUTES = 10 * 60 * 1000;

export function DownloadContent() {
  const [windowsUrl, setWindowsUrl] = useState<string>(DEFAULT_WINDOWS_DOWNLOAD_URL);
  const [androidUrl, setAndroidUrl] = useState<string>(DEFAULT_ANDROID_DOWNLOAD_URL);
  const [releaseUrl, setReleaseUrl] = useState<string>(RELEASES_URL);
  const [recommendedOs, setRecommendedOs] = useState<"win" | "and" | null>(null);

  useEffect(() => {
    const ua = navigator.userAgent;
    if (/Android/i.test(ua)) {
      setRecommendedOs("and");
    } else if (/Windows/i.test(ua)) {
      setRecommendedOs("win");
    }

    let cancelled = false;
    async function load() {
      try {
        const cached = sessionStorage.getItem(CACHE_KEY);
        if (cached) {
          const parsed = JSON.parse(cached) as CachedRelease;
          if (Date.now() - parsed.savedAt < TEN_MINUTES) {
            if (!cancelled) applyRelease(parsed.release);
            return;
          }
        }
        const response = await fetch(LATEST_RELEASE_API_URL, {
          headers: { Accept: "application/vnd.github+json" },
        });
        if (!response.ok) return;
        const latest = (await response.json()) as LatestRelease;
        sessionStorage.setItem(
          CACHE_KEY,
          JSON.stringify({ savedAt: Date.now(), release: latest } satisfies CachedRelease),
        );
        if (!cancelled) applyRelease(latest);
      } catch {
        // Fallbacks are already set to RELEASES_URL
      }
    }

    function applyRelease(rel: LatestRelease) {
      if (rel.html_url) setReleaseUrl(rel.html_url);
      const win = rel.assets.find(
        (asset) =>
          asset.name.toLowerCase().endsWith(".exe") || asset.name.toLowerCase().includes("windows"),
      );
      const android = rel.assets.find(
        (asset) =>
          asset.name.toLowerCase().endsWith(".apk") || asset.name.toLowerCase().includes("android"),
      );
      if (win?.browser_download_url) setWindowsUrl(win.browser_download_url);
      if (android?.browser_download_url) setAndroidUrl(android.browser_download_url);
    }

    void load();
    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <main className="mx-auto max-w-[880px] px-6 pt-12 pb-24 sm:pt-[72px]">
      <div className="mb-5 text-[13px] text-muted-foreground">Product / Download</div>
      <h1 className="mb-3 text-4xl font-semibold leading-[1.05] tracking-[-0.04em] sm:text-5xl lg:text-[56px]">
        Download
      </h1>
      <p className="mb-8 text-lg text-muted-foreground">
        Choose your device and start studying offline.
      </p>
      <a
        href={releaseUrl}
        target="_blank"
        rel="noopener noreferrer"
        className="inline-block border-b border-foreground font-medium transition-opacity hover:opacity-70"
      >
        All releases
      </a>

      <div className="mt-14">
        <h2 className="mb-3 text-[28px] font-semibold leading-[1.1] tracking-[-0.03em]">Install</h2>
      </div>

      <div className="mt-10 grid grid-cols-1 gap-4 md:grid-cols-2">
        <div
          id="win"
          className={cn(
            "relative flex flex-col rounded-2xl border bg-card p-7 transition-colors",
            recommendedOs === "win"
              ? "border-foreground"
              : "border-border hover:border-muted-foreground/40",
          )}
        >
          <div className="flex min-h-11 items-center justify-between">
            <span className="grid size-11 place-items-center rounded-xl border border-border bg-background">
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
                aria-hidden="true"
              >
                <rect x="3" y="4" width="18" height="12" rx="2" />
                <path d="M8 20h8M12 16v4" />
              </svg>
            </span>
            {recommendedOs === "win" && (
              <span className="rounded-full bg-success-soft px-2.5 py-0.5 text-xs font-medium text-success-foreground">
                Recommended for you
              </span>
            )}
          </div>
          <h3 className="mt-4 mb-1 text-[22px] font-semibold tracking-[-0.02em]">Windows</h3>
          <div className="mb-5 font-mono text-xs text-muted-foreground">
            .exe installer · Windows 10 or later
          </div>
          <a
            className="mb-6 inline-flex h-10 w-full items-center justify-center gap-2 rounded-full border border-foreground bg-foreground px-[18px] text-sm font-medium text-background transition-opacity hover:opacity-90"
            href={windowsUrl}
            download
          >
            Download for Windows
          </a>
          <ol className="mb-5 grid list-none gap-3 p-0">
            <li className="flex items-start gap-3 text-sm">
              <span className="mt-px flex size-[22px] shrink-0 items-center justify-center rounded-full border border-border font-mono text-[11px] font-medium text-muted-foreground">
                1
              </span>
              <span>Download the installer.</span>
            </li>
            <li className="flex items-start gap-3 text-sm">
              <span className="mt-px flex size-[22px] shrink-0 items-center justify-center rounded-full border border-border font-mono text-[11px] font-medium text-muted-foreground">
                2
              </span>
              <span>Open the file and follow the steps.</span>
            </li>
            <li className="flex items-start gap-3 text-sm">
              <span className="mt-px flex size-[22px] shrink-0 items-center justify-center rounded-full border border-border font-mono text-[11px] font-medium text-muted-foreground">
                3
              </span>
              <span>Launch the app from the Start menu.</span>
            </li>
          </ol>
          <div className="mt-auto flex items-start gap-2.5 rounded-[10px] bg-info-soft p-3 text-[13px] leading-relaxed text-info-foreground">
            <svg
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="mt-0.5 shrink-0"
              aria-hidden="true"
            >
              <circle cx="12" cy="12" r="9" />
              <path d="M12 11v5M12 8h.01" />
            </svg>
            <span>
              If SmartScreen warns you, choose <b>More info</b>, then <b>Run anyway</b>.
            </span>
          </div>
        </div>

        <div
          id="and"
          className={cn(
            "relative flex flex-col rounded-2xl border bg-card p-7 transition-colors",
            recommendedOs === "and"
              ? "border-foreground"
              : "border-border hover:border-muted-foreground/40",
          )}
        >
          <div className="flex min-h-11 items-center justify-between">
            <span className="grid size-11 place-items-center rounded-xl border border-border bg-background">
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
                aria-hidden="true"
              >
                <rect x="7" y="2" width="10" height="20" rx="2.5" />
                <path d="M11 18h2" />
              </svg>
            </span>
            {recommendedOs === "and" && (
              <span className="rounded-full bg-success-soft px-2.5 py-0.5 text-xs font-medium text-success-foreground">
                Recommended for you
              </span>
            )}
          </div>
          <h3 className="mt-4 mb-1 text-[22px] font-semibold tracking-[-0.02em]">Android</h3>
          <div className="mb-5 font-mono text-xs text-muted-foreground">
            .apk file · Android 8 or later
          </div>
          <a
            className="mb-6 inline-flex h-10 w-full items-center justify-center gap-2 rounded-full border border-foreground bg-foreground px-[18px] text-sm font-medium text-background transition-opacity hover:opacity-90"
            href={androidUrl}
            download
          >
            Download APK
          </a>
          <ol className="mb-5 grid list-none gap-3 p-0">
            <li className="flex items-start gap-3 text-sm">
              <span className="mt-px flex size-[22px] shrink-0 items-center justify-center rounded-full border border-border font-mono text-[11px] font-medium text-muted-foreground">
                1
              </span>
              <span>Download the APK.</span>
            </li>
            <li className="flex items-start gap-3 text-sm">
              <span className="mt-px flex size-[22px] shrink-0 items-center justify-center rounded-full border border-border font-mono text-[11px] font-medium text-muted-foreground">
                2
              </span>
              <span>Open the downloaded file.</span>
            </li>
            <li className="flex items-start gap-3 text-sm">
              <span className="mt-px flex size-[22px] shrink-0 items-center justify-center rounded-full border border-border font-mono text-[11px] font-medium text-muted-foreground">
                3
              </span>
              <span>Approve the install when Android asks.</span>
            </li>
          </ol>
          <div className="mt-auto flex items-start gap-2.5 rounded-[10px] bg-warning-soft p-3 text-[13px] leading-relaxed text-warning-foreground">
            <svg
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="mt-0.5 shrink-0"
              aria-hidden="true"
            >
              <path d="M12 3l9 16H3z" />
              <path d="M12 10v4M12 17h.01" />
            </svg>
            <span>Allow “install unknown apps” for your browser, then open the file.</span>
          </div>
        </div>
      </div>

      <div className="mt-4 flex items-start gap-2.5 rounded-[10px] bg-success-soft p-3 text-[13px] leading-relaxed text-success-foreground">
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
          className="mt-0.5 shrink-0"
          aria-hidden="true"
        >
          <path d="M21 12a9 9 0 11-4-7.5" />
          <path d="M8 11l4 4 9-9" />
        </svg>
        <span>The app checks for updates and always asks before downloading.</span>
      </div>

      <div className="mt-8 flex flex-wrap items-center justify-between gap-4 border-t border-border pt-6">
        <span className="flex items-center gap-2.5 text-sm text-muted-foreground">
          <span className="rounded-full bg-warning-soft px-2.5 py-0.5 text-xs font-medium text-warning-foreground">
            Soon
          </span>
          iPhone: not available yet.
        </span>
        <Link
          href="/docs/install"
          className="border-b border-foreground text-sm font-medium transition-opacity hover:opacity-70"
        >
          Read the installation guide
        </Link>
      </div>
    </main>
  );
}
