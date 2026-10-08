import type { ReactNode } from "react";
import { RootProvider } from "fumadocs-ui/provider/next";
import { SITE_NAME, SITE_DESCRIPTION } from "@/lib/config";
import type { Metadata } from "next";
import { Bricolage_Grotesque } from "next/font/google";
import "fumadocs-ui/style.css";
import StaticSearchDialog from "@/components/search-dialog";
import { SITE_URL } from "@/lib/config";
import "./globals.css";

const bricolageGrotesque = Bricolage_Grotesque({
  subsets: ["latin"],
  display: "swap",
  variable: "--font-title",
});

export const metadata: Metadata = {
  metadataBase: new URL(SITE_URL),
  title: {
    default: SITE_NAME,
    template: `%s | ${SITE_NAME}`,
  },
  description: SITE_DESCRIPTION,
  icons: {
    icon: "/favicon.png",
  },
  openGraph: {
    type: "website",
    siteName: SITE_NAME,
    title: SITE_NAME,
    description: SITE_DESCRIPTION,
    images: [{ url: "/logo.png", alt: `${SITE_NAME} logo` }],
  },
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html
      lang="en"
      className={bricolageGrotesque.variable}
      suppressHydrationWarning
    >
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
