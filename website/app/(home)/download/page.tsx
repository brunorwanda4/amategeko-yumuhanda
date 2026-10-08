import type { Metadata } from "next";
import Link from "next/link";

import { ReleaseDownloads } from "@/components/release-downloads";
import { SITE_NAME } from "@/lib/config";

const description = "Download Amategeko y'Umuhanda for Windows or Android.";

export const metadata: Metadata = {
  title: "Download",
  description,
  openGraph: { title: `Download | ${SITE_NAME}`, description, url: "/download", images: [{ url: "/logo.png", alt: `${SITE_NAME} logo` }] },
};

export default function DownloadPage() {
  return (
    <main className="mx-auto max-w-5xl px-5 py-16 sm:px-8 sm:py-24">
      <header className="max-w-2xl"><h1 className="text-4xl font-semibold tracking-tight sm:text-5xl">Download</h1><p className="mt-4 text-lg text-muted-foreground">Choose your device and start studying offline.</p></header>
      <section className="mt-12" aria-label="Latest release downloads"><ReleaseDownloads /></section>
      <section className="mt-20">
        <h2 className="text-2xl font-semibold">Install</h2>
        <div className="mt-7 grid gap-8 md:grid-cols-2">
          <article>
            <h3 className="font-semibold">Windows</h3>
            <ol className="mt-3 list-inside list-decimal text-sm leading-7 text-muted-foreground"><li>Download the installer.</li><li>Open the file and follow the steps.</li><li>Launch the app from the Start menu.</li></ol>
            <aside className="mt-4 rounded-lg bg-info-soft p-4 text-sm text-info-foreground">If SmartScreen warns you, choose More info, then Run anyway.</aside>
          </article>
          <article>
            <h3 className="font-semibold">Android</h3>
            <ol className="mt-3 list-inside list-decimal text-sm leading-7 text-muted-foreground"><li>Download the APK.</li><li>Open the downloaded file.</li><li>Approve the install when Android asks.</li></ol>
            <aside className="mt-4 rounded-lg bg-warning-soft p-4 text-sm text-warning-foreground">Allow &apos;install unknown apps&apos; for your browser, then open the file.</aside>
          </article>
        </div>
        <aside className="mt-8 rounded-lg bg-info-soft p-4 text-sm text-info-foreground">The app checks for updates and asks before downloading.</aside>
        <p className="mt-5 text-sm text-muted-foreground">iPhone: not available yet.</p>
        <Link href="/docs/install" className="mt-6 inline-block text-sm font-medium underline underline-offset-4">Read the installation guide</Link>
      </section>
    </main>
  );
}
