"use client";

import { useEffect, useState } from "react";

export default function AppSpikePage() {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let canceled = false;

    async function loadApp() {
      const startTime = performance.now();
      try {
        const url = "/app-spike/amategeko_web_spike.js";
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const wasmModule: any = await import(/* webpackIgnore: true */ /* @vite-ignore */ url);
        await wasmModule.default();

        const isDark = document.documentElement.classList.contains("dark");
        await wasmModule.run(isDark);

        const firstFrameTime = performance.now() - startTime;
        console.log(`[AppSpike] Time to first frame: ${firstFrameTime.toFixed(2)}ms`);

        if (!canceled) {
          setLoading(false);
        }
      } catch (err: unknown) {
        console.error("[AppSpike] Failed to initialize:", err);
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

  return (
    <div className="relative w-screen h-screen overflow-hidden bg-background text-foreground">
      {loading && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-background">
          <p className="text-lg font-medium animate-pulse">Loading…</p>
        </div>
      )}
      {error && (
        <div className="fixed inset-0 z-50 flex flex-col items-center justify-center bg-background p-4 text-red-500">
          <p className="text-xl font-bold mb-2">Error loading WASM App</p>
          <p className="text-sm font-mono">{error}</p>
        </div>
      )}
      <style jsx global>{`
        body > canvas {
          position: fixed !important;
          inset: 0 !important;
          width: 100vw !important;
          height: 100vh !important;
          display: block !important;
        }
      `}</style>
    </div>
  );
}
