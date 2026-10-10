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
import { REPO_URL, AUTHOR_NAME, AUTHOR_URL } from "@/lib/config";

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

  return (
    <DocsPage toc={page.data.toc} full={page.data.full}>
      <ViewOptionsPopover githubUrl={githubUrl} />
      <header className="mb-8 border-b border-border pb-6">
        <DocsTitle className="text-3xl sm:text-4xl font-semibold tracking-[-0.04em] text-foreground">
          {page.data.title}
        </DocsTitle>
        <DocsDescription className="mt-2 text-base text-muted-foreground">
          {page.data.description}
        </DocsDescription>
      </header>
      <DocsBody>
        <MDX components={mdxComponents} />
        <footer className="mt-12 flex flex-wrap items-center gap-x-2 gap-y-1 border-t border-border pt-6 text-sm text-muted-foreground">
          <span>
            Built by{" "}
            <a
              href={AUTHOR_URL}
              rel="noopener noreferrer"
              target="_blank"
              className="text-foreground underline underline-offset-2 hover:opacity-80"
            >
              {AUTHOR_NAME}
            </a>
          </span>
          <span aria-hidden="true">·</span>
          <span>Study tool, not the official exam.</span>
        </footer>
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
