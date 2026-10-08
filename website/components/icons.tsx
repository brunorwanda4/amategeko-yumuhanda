import type { SVGProps } from "react";

type IconProps = SVGProps<SVGSVGElement>;

const defaults = {
  fill: "none",
  viewBox: "0 0 24 24",
  stroke: "currentColor",
  strokeWidth: 1.8,
  strokeLinecap: "round" as const,
  strokeLinejoin: "round" as const,
  "aria-hidden": true,
};

export function PlayIcon(props: IconProps) {
  return <svg {...defaults} {...props}><path d="m9 7 8 5-8 5V7Z" /><circle cx="12" cy="12" r="9" /></svg>;
}

export function GiftIcon(props: IconProps) {
  return <svg {...defaults} {...props}><path d="M4 10h16v10H4zM2.5 6.5h19V10h-19zM12 6.5V20" /><path d="M12 6.5C10 6.5 7 5.4 7 3.7c0-1 1-1.7 2-1.7 1.8 0 3 2.6 3 4.5Zm0 0c2 0 5-1.1 5-2.8 0-1-1-1.7-2-1.7-1.8 0-3 2.6-3 4.5Z" /></svg>;
}

export function OfflineIcon(props: IconProps) {
  return <svg {...defaults} {...props}><path d="M6.4 6.4A9.7 9.7 0 0 1 12 4.7c4 0 7.4 2.3 9 5.7M3 3l18 18M3 10.4A10 10 0 0 1 4.7 8M6.8 14.2A7.1 7.1 0 0 1 12 12c1 0 2 .2 2.8.6M9.5 17.1c.7-.5 1.6-.8 2.5-.8" /><path d="M12 20h.01" /></svg>;
}

export function LanguagesIcon(props: IconProps) {
  return <svg {...defaults} {...props}><path d="M4 5h7M7.5 3v2M5 8c1.5 2.8 3.5 4.7 6 5.8M10 8c-.8 2.1-2.3 4-4.7 5.6M13 19l4-10 4 10M14.5 15.5h5" /></svg>;
}

export function DownloadIcon(props: IconProps) {
  return <svg {...defaults} {...props}><path d="M12 3v12m-4-4 4 4 4-4M5 20h14" /></svg>;
}

export function WindowsIcon(props: IconProps) {
  return <svg {...defaults} {...props}><path d="m3 5 8-1v8H3V5Zm10-1.2L21 3v9h-8V3.8ZM3 14h8v8l-8-1v-7Zm10 0h8v9l-8-.8V14Z" /></svg>;
}

export function AndroidIcon(props: IconProps) {
  return <svg {...defaults} {...props}><path d="M6 9h12v9H6zM8 9a4 4 0 0 1 8 0M8.5 4.5 7 2.5m8.5 2L17 2.5M8 12h.01M16 12h.01M4 10v6m16-6v6M9 18v3m6-3v3" /></svg>;
}

export function ImageIcon(props: IconProps) {
  return <svg {...defaults} {...props}><rect x="3" y="4" width="18" height="16" rx="2" /><circle cx="9" cy="10" r="2" /><path d="m5 18 4.5-4 3.5 3 2.5-2 3.5 3" /></svg>;
}
