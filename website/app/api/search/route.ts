import { source } from "@/lib/source";
import { createFromSource } from "fumadocs-core/search/server";

// Force static generation for static export compatibility
export const revalidate = false;
export const dynamic = "force-static";

export const { staticGET: GET } = createFromSource(source);
