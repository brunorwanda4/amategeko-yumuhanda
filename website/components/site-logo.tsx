import Image from "next/image";

import { SITE_NAME } from "@/lib/config";

export function SiteLogo() {
  return (
    <span className="flex items-center gap-2 font-semibold">
      <Image src="/logo.png" alt="" width={28} height={28} className="rounded-md" />
      <span className="font-title">{SITE_NAME}</span>
    </span>
  );
}
