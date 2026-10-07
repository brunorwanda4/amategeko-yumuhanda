import type { ReactNode } from "react";
import { RootProvider } from "fumadocs-ui/provider/next";
import { SITE_NAME, SITE_DESCRIPTION } from "@/lib/config";
import type { Metadata } from "next";
import "fumadocs-ui/style.css";
import StaticSearchDialog from "@/components/search-dialog";

export const metadata: Metadata = {
  title: {
    default: SITE_NAME,
    template: `%s | ${SITE_NAME}`,
  },
  description: SITE_DESCRIPTION,
  icons: {
    icon: "/favicon.png",
  },
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body>
        <RootProvider
          search={{
            SearchDialog: StaticSearchDialog,
          }}
        >
          {children}
        </RootProvider>
      </body>
    </html>
  );
}
