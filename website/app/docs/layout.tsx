import type { ReactNode } from "react";
import { DocsLayout } from "fumadocs-ui/layouts/docs";
import { source } from "@/lib/source";
import { REPO_URL, RELEASES_URL, SITE_NAME, AUTHOR_URL } from "@/lib/config";
import Image from "next/image";

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <DocsLayout
      tree={source.pageTree}
      githubUrl={REPO_URL}
      nav={{
        title: (
          <span className="flex items-center gap-2 font-semibold">
            <Image
              src="/logo.png"
              alt="Amategeko y'Umuhanda logo"
              width={24}
              height={24}
              className="rounded-sm"
            />
            {SITE_NAME}
          </span>
        ),
      }}
      links={[
        {
          text: "Docs",
          url: "/docs",
          active: "nested-url",
        },
        {
          text: "Download",
          url: RELEASES_URL,
          active: "nested-url",
        },
        {
          text: "GitHub",
          url: REPO_URL,
          active: "nested-url",
        },
      ]}
    >
      {children}
    </DocsLayout>
  );
}
