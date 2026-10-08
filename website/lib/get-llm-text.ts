import { source } from "./source";
import { REPO_URL, RELEASES_URL, SITE_URL } from "./config";

const PAGE_ORDER = [
  "index",
  "install",
  "quiz-modes",
  "screens",
  "settings",
  "shortcuts",
  "development",
];

export function cleanMarkdown(content: string): string {
  let text = content;

  // Remove MDX import and export statements
  text = text.replace(/^import\s+.*$/gm, "");
  text = text.replace(/^export\s+.*$/gm, "");

  // Convert Callout components to "Warning: " or "Note: "
  text = text.replace(
    /<Callout\s+[^>]*type=["'](?:warn|warning|caution)["'][^>]*>([\s\S]*?)<\/Callout>/gi,
    (_match, p1) => `Warning: ${p1.trim()}`,
  );
  text = text.replace(
    /<Callout[^>]*>([\s\S]*?)<\/Callout>/gi,
    (_match, p1) => `Note: ${p1.trim()}`,
  );
  text = text.replace(/<Callout\s+[^>]*>/gi, "Note: ");
  text = text.replace(/<\/Callout>/gi, "");

  // Convert Tab components to headers
  text = text.replace(/<Tab\s+[^>]*title=["']([^"']+)["'][^>]*>/gi, "\n### $1\n");
  text = text.replace(/<Tab\s+[^>]*value=["']([^"']+)["'][^>]*>/gi, "\n### $1\n");

  // Remove remaining JSX / HTML tags while preserving inner text
  text = text.replace(/<\/?[A-Za-z][A-Za-z0-9.]*(?:\s+[^>]*)?\/?>/g, "");

  // Replace Markdown images ![alt](url) with alt text
  text = text.replace(/!\[([^\]]*)\]\([^)]*\)/g, "$1");

  // Normalize excessive blank lines
  text = text.replace(/\n{3,}/g, "\n\n");

  return text.trim();
}

export function getSortedDocsPages() {
  const pages = source.getPages();
  return pages.sort((a, b) => {
    const slugA = a.slugs.length === 0 ? "index" : a.slugs.join("/");
    const slugB = b.slugs.length === 0 ? "index" : b.slugs.join("/");
    const indexA = PAGE_ORDER.indexOf(slugA);
    const indexB = PAGE_ORDER.indexOf(slugB);
    return (indexA === -1 ? 999 : indexA) - (indexB === -1 ? 999 : indexB);
  });
}

export async function generateLLMFullText(): Promise<string> {
  const pages = getSortedDocsPages();

  const header = [
    "# Amategeko y'Umuhanda",
    "> Offline study app for the Rwanda driving theory test, Windows and Android, English and Kinyarwanda, free",
    `Site: ${SITE_URL}`,
    `Download: ${SITE_URL}/download`,
    `Source: ${REPO_URL}`,
  ].join("\n");

  const pageTexts: string[] = [];

  for (const page of pages) {
    let rawText = "";
    if (typeof page.data.getText === "function") {
      rawText = await page.data.getText("processed");
    } else if ("structuredData" in page.data) {
      rawText = (page.data as { content?: string }).content || "";
    }

    const cleaned = cleanMarkdown(rawText);
    const pageUrl = `${SITE_URL}${page.url}`;
    const description = page.data.description ? `${page.data.description}\n\n` : "";

    const pageBlock = `# ${page.data.title}\nURL: ${pageUrl}\n\n${description}${cleaned}`;
    pageTexts.push(pageBlock);
  }

  return `${header}\n\n---\n\n${pageTexts.join("\n\n---\n\n")}\n`;
}

export function generateLLMIndexText(): string {
  const pages = getSortedDocsPages();

  const header = [
    "# Amategeko y'Umuhanda",
    "",
    "> Offline study app for the Rwanda driving theory test, Windows and Android, English and Kinyarwanda, free",
    "",
    "## Docs",
    "",
  ].join("\n");

  const docsList = pages
    .map((page) => `- [${page.data.title}](${SITE_URL}${page.url}): ${page.data.description || ""}`)
    .join("\n");

  const optional = [
    "",
    "## Optional",
    "",
    `- [Download](${SITE_URL}/download): Download the app for Windows and Android`,
    `- [Releases](${RELEASES_URL}): Latest releases and APK downloads`,
    `- [Source code](${REPO_URL}): Open source repository on GitHub`,
    `- [Blog](${SITE_URL}/blog): Articles and updates`,
  ].join("\n");

  return `${header}${docsList}\n${optional}\n`;
}
