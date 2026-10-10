import type { Metadata } from "next";

import { DownloadContent } from "@/components/download-content";
import { SITE_NAME } from "@/lib/config";

const description = "Download Amategeko y'Umuhanda for Windows or Android.";

export const metadata: Metadata = {
  title: "Download",
  description,
  openGraph: {
    title: `Download | ${SITE_NAME}`,
    description,
    url: "/download",
    images: [{ url: "/logo.png", alt: `${SITE_NAME} logo` }],
  },
};

export default function DownloadPage() {
  return <DownloadContent />;
}
