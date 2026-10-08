"use client";

import { useEffect, useState, type ReactNode } from "react";

import { AndroidIcon, DownloadIcon, WindowsIcon } from "@/components/icons";
import { Skeleton } from "@/components/ui/skeleton";
import { LATEST_RELEASE_API_URL, RELEASES_URL } from "@/lib/config";

type ReleaseAsset = { name: string; size: number; browser_download_url: string };
type LatestRelease = { tag_name: string; published_at: string; html_url: string; assets: ReleaseAsset[] };
type CachedRelease = { savedAt: number; release: LatestRelease };

const CACHE_KEY = "amategeko-latest-release";
const TEN_MINUTES = 10 * 60 * 1000;

export function ReleaseDownloads() {
  const [release, setRelease] = useState<LatestRelease | null>(null);
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    let cancelled = false;
    async function load() {
      try {
        const cached = sessionStorage.getItem(CACHE_KEY);
        if (cached) {
          const parsed = JSON.parse(cached) as CachedRelease;
          if (Date.now() - parsed.savedAt < TEN_MINUTES) {
            if (!cancelled) setRelease(parsed.release);
            return;
          }
        }
        const response = await fetch(LATEST_RELEASE_API_URL, { headers: { Accept: "application/vnd.github+json" } });
        if (!response.ok) throw new Error("Release request failed");
        const latest = (await response.json()) as LatestRelease;
        const hasWindows = latest.assets.some((asset) => asset.name.endsWith("-windows-setup.exe"));
        const hasAndroid = latest.assets.some((asset) => asset.name.endsWith("-android-arm64.apk"));
        if (!hasWindows || !hasAndroid) throw new Error("Release assets missing");
        sessionStorage.setItem(CACHE_KEY, JSON.stringify({ savedAt: Date.now(), release: latest } satisfies CachedRelease));
        if (!cancelled) setRelease(latest);
      } catch {
        if (!cancelled) setFailed(true);
      }
    }
    void load();
    return () => { cancelled = true; };
  }, []);

  if (failed) {
    return <a className="font-medium underline underline-offset-4" href={RELEASES_URL} rel="noopener noreferrer" target="_blank">All releases</a>;
  }
  if (!release) {
    return <div className="grid gap-5 md:grid-cols-2" aria-label="Loading latest release"><ReleaseSkeleton /><ReleaseSkeleton /></div>;
  }
  const windows = release.assets.find((asset) => asset.name.endsWith("-windows-setup.exe"));
  const android = release.assets.find((asset) => asset.name.endsWith("-android-arm64.apk"));
  if (!windows || !android) return null;
  return (
    <div className="grid gap-5 md:grid-cols-2">
      <ReleaseCard title="Windows" detail="Installer (.exe)" asset={windows} release={release} icon={<WindowsIcon className="size-8" />} />
      <ReleaseCard title="Android" detail="APK · arm64 · Android 8+" asset={android} release={release} icon={<AndroidIcon className="size-8" />} />
    </div>
  );
}

function ReleaseCard({ title, detail, asset, release, icon }: { title: string; detail: string; asset: ReleaseAsset; release: LatestRelease; icon: ReactNode }) {
  const version = release.tag_name.replace(/^v/, "");
  const size = `${(asset.size / 1024 / 1024).toFixed(1)} MB`;
  const date = new Intl.DateTimeFormat("en", { year: "numeric", month: "short", day: "numeric" }).format(new Date(release.published_at));
  return (
    <article className="flex flex-col rounded-xl border bg-card p-6 text-card-foreground">
      <div className="flex items-start justify-between gap-5">
        <div className="flex items-center gap-3">{icon}<div><h2 className="text-xl font-semibold">{title}</h2><p className="text-sm text-muted-foreground">{detail}</p></div></div>
        <span className="rounded-full bg-success-soft px-3 py-1 text-xs font-semibold text-success-foreground">v{version}</span>
      </div>
      <dl className="mt-8 grid grid-cols-2 gap-4 text-sm"><div><dt className="text-muted-foreground">Size</dt><dd className="mt-1 font-medium">{size}</dd></div><div><dt className="text-muted-foreground">Released</dt><dd className="mt-1 font-medium">{date}</dd></div></dl>
      <div className="mt-8 flex flex-wrap items-center gap-4">
        <a href={asset.browser_download_url} className="inline-flex min-h-11 items-center gap-2 rounded-lg bg-primary px-5 text-sm font-semibold text-primary-foreground"><DownloadIcon className="size-4" />Download</a>
        <a className="text-sm font-medium underline underline-offset-4" href={release.html_url} rel="noopener noreferrer" target="_blank">Release notes</a>
      </div>
    </article>
  );
}

function ReleaseSkeleton() {
  return (
    <div className="rounded-xl border p-6">
      <div className="flex justify-between gap-4"><Skeleton className="h-12 w-36" /><Skeleton className="h-7 w-16 rounded-full" /></div>
      <div className="mt-8 grid grid-cols-2 gap-4"><Skeleton className="h-12" /><Skeleton className="h-12" /></div>
      <Skeleton className="mt-8 h-11 w-32" />
    </div>
  );
}
