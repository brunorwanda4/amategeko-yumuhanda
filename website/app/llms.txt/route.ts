import { generateLLMIndexText } from "@/lib/get-llm-text";

export const dynamic = "force-static";
export const revalidate = false;

export async function GET() {
  const content = generateLLMIndexText();

  return new Response(content, {
    headers: {
      "Content-Type": "text/plain; charset=utf-8",
    },
  });
}
