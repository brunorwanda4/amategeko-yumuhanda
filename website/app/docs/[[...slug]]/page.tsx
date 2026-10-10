import {
  DocsPage,
  DocsBody,
  DocsTitle,
  DocsDescription,
  ViewOptionsPopover,
} from "fumadocs-ui/layouts/docs/page";
import { source } from "@/lib/source";
import { notFound } from "next/navigation";
import defaultMdxComponents from "fumadocs-ui/mdx";
import { useMDXComponents } from "@/mdx-components";
import { REPO_URL } from "@/lib/config";
import { CopyMarkdownButton } from "@/components/copy-markdown-button";

interface Props {
  params: Promise<{ slug?: string[] }>;
}

export default async function Page({ params }: Props) {
  const { slug } = await params;
  const page = source.getPage(slug);
  if (!page) notFound();

  const MDX = page.data.body;
  const githubUrl = `${REPO_URL}/blob/main/website/content/docs/${page.path}`;
  const mdxComponents = useMDXComponents(defaultMdxComponents);

  const rawMarkdown =
    typeof page.data.getText === "function" ? await page.data.getText("processed") : "";

  return (
    <DocsPage toc={page.data.toc} full={page.data.full}>
      <header className="mb-8">
        <DocsTitle className="text-3xl sm:text-4xl font-semibold tracking-[-0.03em] text-foreground">
          {page.data.title}
        </DocsTitle>
        {page.data.description && (
          <DocsDescription className="mt-2 text-base text-muted-foreground leading-relaxed">
            {page.data.description}
          </DocsDescription>
        )}
        <div className="flex flex-wrap items-center gap-2 mt-4 not-prose">
          <CopyMarkdownButton markdown={rawMarkdown} />
          <ViewOptionsPopover githubUrl={githubUrl} />
        </div>
      </header>
      <DocsBody>
        <MDX components={mdxComponents} />
      </DocsBody>
    </DocsPage>
  );
}

export async function generateStaticParams() {
  return source.generateParams();
}

export async function generateMetadata({ params }: Props) {
  const { slug } = await params;
  const page = source.getPage(slug);
  if (!page) notFound();
  return {
    title: page.data.title,
    description: page.data.description,
  };
}
