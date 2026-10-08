"use client";

import Image from "next/image";
import { useState } from "react";

import { ImageIcon } from "@/components/icons";
import { cn } from "@/lib/utils";

export function ScreenshotFrame({ src, alt, kind = "desktop", priority = false, className }: {
  src: string;
  alt: string;
  kind?: "desktop" | "phone";
  priority?: boolean;
  className?: string;
}) {
  const [missing, setMissing] = useState(false);
  const phone = kind === "phone";
  return (
    <figure className={cn("relative overflow-hidden border border-white/15 bg-neutral-900 shadow-2xl", phone ? "aspect-[9/19] rounded-[2rem] p-1.5" : "aspect-[16/10] rounded-xl p-1.5", className)}>
      {missing ? (
        <div className="flex size-full flex-col items-center justify-center gap-3 rounded-[inherit] bg-neutral-800 text-neutral-400">
          <ImageIcon className="size-8" />
          <span className="text-xs">Screenshot coming soon</span>
        </div>
      ) : (
        <div className="relative size-full overflow-hidden rounded-[inherit] bg-neutral-800">
          <Image src={src} alt={alt} fill priority={priority} sizes={phone ? "(max-width: 768px) 35vw, 260px" : "(max-width: 768px) 92vw, 900px"} className="object-cover object-top" onError={() => setMissing(true)} />
        </div>
      )}
    </figure>
  );
}
