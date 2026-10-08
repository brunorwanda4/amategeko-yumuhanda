"use client";

import dynamic from "next/dynamic";
import { useEffect, useState } from "react";

const Ripple = dynamic(() => import("@/components/canvasui/Ripple").then((module) => module.Ripple), { ssr: false });

export function HeroEffect() {
  const [enabled, setEnabled] = useState(false);
  useEffect(() => {
    const query = window.matchMedia("(min-width: 768px) and (prefers-reduced-motion: no-preference)");
    const update = () => setEnabled(query.matches);
    update();
    query.addEventListener("change", update);
    return () => query.removeEventListener("change", update);
  }, []);
  if (!enabled) return null;
  return (
    <Ripple className="pointer-events-none absolute inset-0" trigger="none" interval={5} amplitude={0.35} wavelength={110} rings={2} refraction={28} dispersion={0} shine={0.45}>
      <div className="size-full" />
    </Ripple>
  );
}
