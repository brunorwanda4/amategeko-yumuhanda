import type { ReactNode } from "react";
import { HomeLayout } from "fumadocs-ui/layouts/home";

import { SiteFooter } from "@/components/site-footer";
import { SiteLogo } from "@/components/site-logo";
import { REPO_URL } from "@/lib/config";

export function HomeShell({ children }: { children: ReactNode }) {
  return (
    <div className="site-home-layout">
      <HomeLayout
        githubUrl={REPO_URL}
        nav={{ title: <SiteLogo /> }}
        links={[
          { text: "Docs", url: "/docs", active: "nested-url" },
          { text: "Download", url: "/download", active: "nested-url" },
          { text: "About", url: "/about", active: "nested-url" },
        ]}
      >
        {children}
      </HomeLayout>
      <SiteFooter />
    </div>
  );
}
