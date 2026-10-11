"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { sampleCount } from "@/lib/facts";
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

interface BuildInfo {
  version: string;
  js: string;
  wasm: string;
  wasmBytes?: number;
  questionSet?: string;
  builtAt?: string;
}

type LoadResult =
  | { status: "success" }
  | { status: "not_built" }
  | { status: "error"; message: string };

let appLoaded = false;
let appLoadingPromise: Promise<LoadResult> | null = null;
let progressCallback: ((pct: number | null) => void) | null = null;
const globalLogs: string[] = [];

if (typeof window !== "undefined") {
  const origError = console.error;
  console.error = (...args: unknown[]) => {
    origError(...args);
    const msg = args.map((a) => (typeof a === "object" ? JSON.stringify(a) : String(a))).join(" ");
    globalLogs.push(`[error] ${msg}`);
  };

  const origWarn = console.warn;
  console.warn = (...args: unknown[]) => {
    origWarn(...args);
    const msg = args.map((a) => (typeof a === "object" ? JSON.stringify(a) : String(a))).join(" ");
    globalLogs.push(`[warn] ${msg}`);
  };

  window.addEventListener("error", (e) => {
    globalLogs.push(`[uncaught] ${e.message} at ${e.filename}:${e.lineno}:${e.colno}`);
  });

  window.addEventListener("unhandledrejection", (e) => {
    const reason = e.reason ? e.reason.stack || e.reason.message || String(e.reason) : "unknown";
    globalLogs.push(`[unhandledrejection] ${reason}`);
  });
}

async function doLoadApp(): Promise<LoadResult> {
  let jsFile = "";
  let wasmFile = "";
  let basePath = "/runtime";
  let wasmBytes: number | null = null;

  try {
    const buildRes = await fetch("/runtime/build.json");
    if (buildRes.ok) {
      const buildInfo: BuildInfo = await buildRes.json();
      if (buildInfo?.js && buildInfo?.wasm) {
        jsFile = buildInfo.js;
        wasmFile = buildInfo.wasm;
        basePath = "/runtime";
        wasmBytes = buildInfo.wasmBytes || null;
      }
    } else {
      const manifestRes = await fetch("/app/manifest.json");
      if (manifestRes.ok) {
        const manifest = await manifestRes.json();
        if (manifest?.js && manifest?.wasm) {
          jsFile = manifest.js;
          wasmFile = manifest.wasm;
          basePath = "/app";
        }
      }
    }
  } catch (err) {
    globalLogs.push(`[build.json fetch failed] ${String(err)}`);
  }

  if (!jsFile || !wasmFile) {
    return { status: "not_built" };
  }

  try {
    const wasmRes = await fetch(`${basePath}/${wasmFile}`);
    if (!wasmRes.ok) {
      return {
        status: "error",
        message: `Failed to download application (${wasmRes.status})`,
      };
    }

    const contentLength = wasmRes.headers.get("content-length");
    const total = contentLength ? parseInt(contentLength, 10) : wasmBytes;
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
          progressCallback?.(Math.min(100, Math.round((received / total) * 100)));
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
      progressCallback?.(null);
      bytes = await wasmRes.arrayBuffer();
    }

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const wasmModule: any = await import(
      /* webpackIgnore: true */ /* @vite-ignore */ `${basePath}/${jsFile}`
    );
    await wasmModule.default({ module_or_path: bytes });

    const isDark = document.documentElement.classList.contains("dark");
    await wasmModule.run(isDark);
    appLoaded = true;
    return { status: "success" };
  } catch (err: unknown) {
    const msg = err instanceof Error ? err.stack || err.message : String(err);
    globalLogs.push(`[doLoadApp exception] ${msg}`);
    return {
      status: "error",
      message: msg,
    };
  }
}

export default function AppPage() {
  const [supported, setSupported] = useState<boolean | null>(null);
  const [notBuilt, setNotBuilt] = useState<boolean>(false);
  const [progress, setProgress] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showDiag, setShowDiag] = useState(false);
  const [diagInfo, setDiagInfo] = useState<string>("");

  useEffect(() => {
    const isSupported = isGpuSupported();
    setSupported(isSupported);
    if (!isSupported) {
      setLoading(false);
      return;
    }

    if (appLoaded) {
      setLoading(false);
    }

    let active = true;
    progressCallback = (pct) => {
      if (active) setProgress(pct);
    };

    if (!appLoadingPromise) {
      appLoadingPromise = doLoadApp();
    }

    appLoadingPromise.then((res) => {
      if (!active) return;
      if (res.status === "not_built") {
        appLoadingPromise = null;
        setNotBuilt(true);
        setLoading(false);
      } else if (res.status === "error") {
        appLoadingPromise = null;
        setError(res.message);
        setLoading(false);
      } else {
        setLoading(false);
      }
    });

    // Check DOM canvas status periodically
    const interval = setInterval(() => {
      if (!active) return;
      const canvases = document.querySelectorAll("canvas");
      const canvasDetails = Array.from(canvases).map((c, i) => {
        const rect = c.getBoundingClientRect();
        const comp = window.getComputedStyle(c);
        return `#${i}: ${rect.width}x${rect.height} (pos:${comp.position}, z:${comp.zIndex}, disp:${comp.display}, vis:${comp.visibility})`;
      });

      const info = [
        `AppLoaded: ${appLoaded}`,
        `Canvases found: ${canvases.length}`,
        ...canvasDetails,
        `Recent Logs (${globalLogs.length}):`,
        ...globalLogs.slice(-6),
      ].join("\n");
      setDiagInfo(info);
    }, 1000);

    return () => {
      active = false;
      progressCallback = null;
      clearInterval(interval);
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

  if (notBuilt) {
    return (
      <main className="flex min-h-[100dvh] flex-col items-center justify-center p-6 text-center">
        <h1 className="text-xl font-semibold">
          The web app is not built here. Run scripts/build-web.
        </h1>
        <p className="mt-2 text-sm text-muted-foreground">
          WebAssembly artifacts have not been compiled in this environment.
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
    <div className="relative h-[100dvh] w-screen overflow-hidden bg-transparent text-foreground select-none pointer-events-none">
      {showDiag && (
        <div className="fixed bottom-2 left-2 z-50 pointer-events-auto max-w-lg max-h-72 overflow-auto rounded bg-black/90 p-3 font-mono text-[11px] text-green-400 border border-green-800 shadow-xl backdrop-blur">
          <div className="flex justify-between items-center mb-1 text-white border-b border-zinc-700 pb-1">
            <span className="font-bold">App Diagnostics</span>
            <button onClick={() => setShowDiag(false)} className="text-zinc-400 hover:text-white">
              ✕
            </button>
          </div>
          <pre className="whitespace-pre-wrap">{diagInfo}</pre>
        </div>
      )}

      {loading && (
        <div className="fixed inset-0 z-40 pointer-events-auto flex flex-col items-center justify-center bg-background p-6">
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
        <div className="fixed inset-0 z-40 pointer-events-auto flex flex-col items-center justify-center bg-background p-6 text-center">
          <p className="text-base font-semibold text-destructive">Failed to start application</p>
          <p className="mt-1 font-mono text-xs text-muted-foreground max-w-xl break-words whitespace-pre-wrap">
            {error}
          </p>
          <div className="mt-6">
            <Button asChild variant="outline" size="sm" className="rounded-full">
              <Link href="/download">Use Download page instead</Link>
            </Button>
          </div>
        </div>
      )}
    </div>
  );
}
