"use client";

import { useEffect } from "react";

export default function RootPage() {
  useEffect(() => {
    window.location.replace("/docs");
  }, []);

  return (
    <main className="mx-auto flex min-h-screen max-w-xl items-center justify-center p-6 text-center">
      <meta httpEquiv="refresh" content="0;url=/docs" />
      <p>
        Continue to{" "}
        <a className="underline underline-offset-4" href="/docs">
          Amategeko y&apos;Umuhanda documentation
        </a>
        .
      </p>
    </main>
  );
}
