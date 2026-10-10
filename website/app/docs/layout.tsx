import type { ReactNode } from "react";
import { DocsLayout } from "fumadocs-ui/layouts/docs";
import { docsTree } from "@/lib/source";
import { baseOptions } from "@/app/layout.config";
import { RELEASES_URL } from "@/lib/config";

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <DocsLayout
      tree={docsTree}
      {...baseOptions}
      containerProps={{
        style: {
          gridTemplate: `"sidebar header toc"
"sidebar toc-popover toc"
"sidebar main toc" 1fr / var(--fd-sidebar-col) minmax(0, 1fr) var(--fd-toc-width)`,
          width: "100%",
          maxWidth: "100%",
        },
      }}
      links={[
        {
          text: "Docs",
          url: "/docs",
          active: "nested-url",
        },
        {
          text: "Releases",
          url: RELEASES_URL,
          external: true,
        },
      ]}
    >
      {children}
    </DocsLayout>
  );
}
