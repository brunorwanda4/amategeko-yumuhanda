import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";
import Image from "next/image";
import Link from "next/link";
import { ThemeSwitch } from "fumadocs-ui/layouts/shared/slots/theme-switch";
import { REPO_URL, SITE_NAME } from "@/lib/config";

export const baseOptions: BaseLayoutProps = {
  nav: {
    title: (
      <span className="flex items-center gap-2.5 font-semibold text-base tracking-tight">
        <Image
          src="/logo.png"
          alt={SITE_NAME}
          width={24}
          height={24}
          className="rounded-sm"
        />
        <span>{SITE_NAME}</span>
      </span>
    ),
    children: (
      <div className="flex items-center gap-2 ms-auto lg:hidden">
        <ThemeSwitch />
        <Link
          href="/download"
          className="inline-flex h-8 items-center justify-center rounded-full bg-fd-primary px-3.5 text-xs font-medium text-fd-primary-foreground hover:opacity-90"
        >
          Download
        </Link>
      </div>
    ),
    transparentMode: "none",
  },
  links: [
    {
      text: "Docs",
      url: "/docs",
      active: "nested-url",
    },
    {
      text: "Download",
      url: "/download",
      active: "nested-url",
    },
    {
      text: "About",
      url: "/about",
      active: "nested-url",
    },
    {
      type: "button",
      text: "Download",
      url: "/download",
      secondary: true,
    },
  ],
  githubUrl: REPO_URL,
};
