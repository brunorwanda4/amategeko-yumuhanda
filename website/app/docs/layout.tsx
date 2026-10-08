import type { ReactNode } from "react";
import { DocsLayout } from "fumadocs-ui/layouts/docs";
import { source } from "@/lib/source";
import { REPO_URL, SITE_NAME } from "@/lib/config";
import Image from "next/image";
import { SiteFooter } from "@/components/site-footer";

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <>
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
          text: "Home",
          url: "/",
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
        },
        {
          text: "About",
          url: "/about",
          active: "nested-url",
        },
      ]}
    >
      {children}
    </DocsLayout>
    <SiteFooter />
    </>
  );
}
