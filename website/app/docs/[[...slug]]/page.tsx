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

  return (
    <DocsPage toc={page.data.toc} full={page.data.full}>
      <ViewOptionsPopover githubUrl={githubUrl} />
      <DocsTitle>{page.data.title}</DocsTitle>
      <DocsDescription>{page.data.description}</DocsDescription>
      <DocsBody>
        <MDX components={{ ...defaultMdxComponents }} />
        <footer className="mt-12 border-t pt-6 text-sm text-muted-foreground flex flex-wrap gap-x-4 gap-y-1">
          <span>
            Built by{" "}
            <a
              href={AUTHOR_URL}
              rel="noopener noreferrer"
              target="_blank"
              className="underline underline-offset-2 hover:text-foreground"
            >
              {AUTHOR_NAME}
            </a>
          </span>
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
