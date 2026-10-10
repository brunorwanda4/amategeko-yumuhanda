"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { Button } from "@/components/ui/button";

function isGpuSupported(): boolean {
  if (typeof window === "undefined") return true;
  if ("gpu" in navigator) return true;
  try {
    const canvas = document.createElement("canvas");
    return !!(canvas.getContext("webgl2") || canvas.getContext("experimental-webgl2"));
  } catch {
    return false;
  }
}

let appLoaded = false;
let appLoadingPromise: Promise<void> | null = null;

export default function AppPage() {
  const [supported, setSupported] = useState<boolean | null>(null);
  const [progress, setProgress] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const isSupported = isGpuSupported();
    setSupported(isSupported);
    if (!isSupported) {
      setLoading(false);
      return;
    }

    if (appLoaded) {
      setLoading(false);
      return;
    }

    let canceled = false;

    async function doLoadApp() {
      let jsFile = "web.js";
      let wasmFile = "web_bg.wasm";

      try {
        const manifestRes = await fetch("/app/manifest.json");
        if (manifestRes.ok) {
          const manifest = await manifestRes.json();
          if (manifest.js) jsFile = manifest.js;
          if (manifest.wasm) wasmFile = manifest.wasm;
        }
      } catch {
        // Fallback to unhashed defaults
      }

      const wasmRes = await fetch(`/app/${wasmFile}`);
      if (!wasmRes.ok) {
        throw new Error(`Failed to download application (${wasmRes.status})`);
      }

      const contentLength = wasmRes.headers.get("content-length");
      const total = contentLength ? parseInt(contentLength, 10) : null;
      let bytes: ArrayBuffer;

      if (total && wasmRes.body) {
        const reader = wasmRes.body.getReader();
        let received = 0;
        const chunks: Uint8Array[] = [];

        while (true) {
          const { done, value } = await reader.read();
          if (done) break;
          if (value) {
            chunks.push(value);
            received += value.length;
            if (!canceled) {
              setProgress(Math.min(100, Math.round((received / total) * 100)));
            }
          }
        }

        const combined = new Uint8Array(received);
        let offset = 0;
        for (const chunk of chunks) {
          combined.set(chunk, offset);
          offset += chunk.length;
        }
        bytes = combined.buffer;
      } else {
        if (!canceled) setProgress(null);
        bytes = await wasmRes.arrayBuffer();
      }

      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const wasmModule: any = await import(
        /* webpackIgnore: true */ /* @vite-ignore */ `/app/${jsFile}`
      );
      await wasmModule.default({ module_or_path: bytes });

      const isDark = document.documentElement.classList.contains("dark");
      await wasmModule.run(isDark);
      appLoaded = true;
    }

    async function loadApp() {
      try {
        if (!appLoadingPromise) {
          appLoadingPromise = doLoadApp();
        }
        await appLoadingPromise;
        if (!canceled) {
          setLoading(false);
        }
      } catch (err: unknown) {
        appLoadingPromise = null;
        if (!canceled) {
          setError(err instanceof Error ? err.message : String(err));
          setLoading(false);
        }
      }
    }

    loadApp();

    return () => {
      canceled = true;
    };
  }, []);

  if (supported === false) {
    return (
      <main className="flex min-h-[100dvh] flex-col items-center justify-center p-6 text-center">
        <h1 className="text-xl font-semibold">
          This browser is not supported. Use the Download page.
        </h1>
        <p className="mt-2 text-sm text-muted-foreground">
          Hardware-accelerated WebGPU or WebGL2 is required to run the application in the browser.
        </p>
        <div className="mt-6">
          <Button asChild size="lg" className="rounded-full">
            <Link href="/download">Download</Link>
          </Button>
        </div>
      </main>
    );
  }

  return (
    <div className="relative h-[100dvh] w-screen overflow-hidden bg-background text-foreground select-none">
      <Link
        href="/"
        className="fixed right-2 top-2 z-50 rounded bg-background/60 px-1.5 py-0.5 text-xs text-muted-foreground backdrop-blur opacity-60 transition-opacity hover:opacity-100 hover:text-foreground"
      >
        Back to site
      </Link>

      {loading && (
        <div className="fixed inset-0 z-40 flex flex-col items-center justify-center bg-background p-6">
          <div className="w-full max-w-xs space-y-3 text-center">
            <p className="text-sm font-medium text-muted-foreground">
              {progress !== null ? `Loading app… ${progress}%` : "Loading app…"}
            </p>
            <div className="h-1.5 w-full overflow-hidden rounded-full bg-secondary">
              {progress !== null ? (
                <div
                  className="h-full bg-primary transition-all duration-150"
                  style={{ width: `${progress}%` }}
                />
              ) : (
                <div className="h-full w-1/3 animate-[indeterminate_1.5s_infinite_linear] rounded-full bg-primary" />
              )}
            </div>
          </div>
        </div>
      )}

      {error && (
        <div className="fixed inset-0 z-40 flex flex-col items-center justify-center bg-background p-6 text-center">
          <p className="text-base font-semibold text-destructive">Failed to start application</p>
          <p className="mt-1 font-mono text-xs text-muted-foreground">{error}</p>
          <div className="mt-6">
            <Button asChild variant="outline" size="sm" className="rounded-full">
              <Link href="/download">Use Download page instead</Link>
            </Button>
          </div>
        </div>
      )}

      <style jsx global>{`
        html,
        body {
          margin: 0;
          padding: 0;
          width: 100%;
          height: 100dvh;
          overflow: hidden !important;
          overscroll-behavior: none !important;
          touch-action: none !important;
        }
        body > canvas {
          position: fixed !important;
          inset: 0 !important;
          width: 100vw !important;
          height: 100dvh !important;
          display: block !important;
          touch-action: none !important;
          overscroll-behavior: none !important;
        }
        @keyframes indeterminate {
          0% {
            transform: translateX(-100%);
          }
          100% {
            transform: translateX(350%);
          }
        }
      `}</style>
    </div>
  );
}
