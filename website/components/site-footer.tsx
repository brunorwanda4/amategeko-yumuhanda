import Image from "next/image";
import Link from "next/link";

import { AUTHOR_NAME, AUTHOR_URL, ISSUES_URL, RELEASES_URL, REPO_URL } from "@/lib/config";

const productLinks = [["Download", "/download"], ["Docs", "/docs"], ["About", "/about"]] as const;
const projectLinks = [["GitHub", REPO_URL], ["Releases", RELEASES_URL], ["Report an issue", ISSUES_URL]] as const;

export function SiteFooter() {
  return (
    <footer className="border-t bg-background">
      <div className="mx-auto grid max-w-6xl gap-10 px-5 py-12 sm:px-8 md:grid-cols-[1.5fr_1fr_1fr]">
        <div className="flex max-w-sm flex-col gap-4">
          <Image src="/logo.png" alt="Amategeko y'Umuhanda logo" width={40} height={40} className="rounded-lg" />
          <p className="text-sm text-muted-foreground">Free study app for the Rwanda driving theory test.</p>
        </div>
        <FooterColumn title="Product" links={productLinks} />
        <FooterColumn title="Project" links={projectLinks} external />
      </div>
      <div className="border-t">
        <div className="mx-auto flex max-w-6xl flex-col gap-3 px-5 py-5 text-sm text-muted-foreground sm:flex-row sm:items-center sm:justify-between sm:px-8">
          <a href={AUTHOR_URL} rel="noopener noreferrer" target="_blank">Built by {AUTHOR_NAME}</a>
          <span>Study tool, not the official exam.</span>
          <span>© {new Date().getFullYear()}</span>
        </div>
      </div>
    </footer>
  );
}

function FooterColumn({ title, links, external = false }: {
  title: string;
  links: ReadonlyArray<readonly [string, string]>;
  external?: boolean;
}) {
  return (
    <nav aria-label={`${title} links`} className="flex flex-col gap-3 text-sm">
      <h2 className="font-semibold text-foreground">{title}</h2>
      {links.map(([label, href]) => external ? (
        <a key={label} href={href} rel="noopener noreferrer" target="_blank" className="text-muted-foreground hover:text-foreground">{label}</a>
      ) : (
        <Link key={label} href={href} className="text-muted-foreground hover:text-foreground">{label}</Link>
      ))}
    </nav>
  );
}
