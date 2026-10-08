import { generateLLMFullText, getSortedDocsPages } from "@/lib/get-llm-text";

export const dynamic = "force-static";
export const revalidate = false;

export async function GET() {
  const content = await generateLLMFullText();
  const pages = getSortedDocsPages();
  const pageCount = pages.length;
  const charCount = content.length;
  const kbSize = (charCount / 1024).toFixed(2);
  const tokenCount = Math.round(charCount / 4);

  console.log(
    `[llms-full.txt] Pages included: ${pageCount}, Size: ${kbSize} KB, Approx tokens: ${tokenCount}`,
  );
  if (tokenCount > 100000) {
    console.warn(`[llms-full.txt] WARNING: Token count (${tokenCount}) exceeds 100,000 tokens!`);
  }

  return new Response(content, {
    headers: {
      "Content-Type": "text/plain; charset=utf-8",
    },
  });
}
