import type { ReactNode } from "react";
import { RootProvider } from "fumadocs-ui/provider/next";
import { SITE_NAME, SITE_DESCRIPTION, SITE_URL } from "@/lib/config";
import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import StaticSearchDialog from "@/components/search-dialog";
import "./globals.css";

const geistSans = Geist({
  subsets: ["latin"],
  variable: "--font-sans",
});

const geistMono = Geist_Mono({
  subsets: ["latin"],
  variable: "--font-mono",
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
      className={`${geistSans.variable} ${geistMono.variable}`}
      suppressHydrationWarning
    >
      <body className="font-sans antialiased">
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
