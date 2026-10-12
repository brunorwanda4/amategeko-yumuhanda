import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";
import Image from "next/image";
import { REPO_URL, SITE_NAME } from "@/lib/config";

export const baseOptions: BaseLayoutProps = {
  nav: {
    title: (
      <span className="flex items-center gap-2.5 font-semibold text-sm tracking-tight">
        <Image src="/logo.png" alt={SITE_NAME} width={24} height={24} className="size-6 rounded-sm" />
        <span>{SITE_NAME}</span>
      </span>
    ),
    transparentMode: "none",
  },
  links: [
    {
      text: "Open app",
      url: "/app",
      active: "nested-url",
    },
    {
      text: "Docs",
      url: "/docs",
      active: "nested-url",
    },
    {
      text: "Download",
      url: "/download",
      active: "nested-url",
      on: "nav",
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
