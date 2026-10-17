import type { Metadata } from "next";

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { ISSUES_URL, SITE_NAME } from "@/lib/config";
import { getCreditGroups } from "@/lib/facts";

const description = `Credits and open-source licenses for ${SITE_NAME}.`;

export const metadata: Metadata = {
  title: "Credits and licenses",
  description,
  openGraph: {
    title: `Credits and licenses | ${SITE_NAME}`,
    description,
    url: "/credits",
  },
};

export default function CreditsPage() {
  const groups = getCreditGroups();

  return (
    <main className="mx-auto flex max-w-6xl flex-col gap-12 px-6 pt-12 pb-24 sm:px-8 sm:pt-20 sm:pb-28">
      <header className="flex max-w-3xl flex-col gap-4">
        <p className="text-sm font-medium text-muted-foreground">Project / Credits</p>
        <h1 className="text-4xl font-semibold tracking-[-0.045em] sm:text-5xl">
          Credits and licenses
        </h1>
        <p className="text-base leading-relaxed text-muted-foreground sm:text-lg">
          I&apos;m grateful to everyone who builds and shares the tools that made this project
          possible.
        </p>
      </header>

      {groups.map((group) => (
        <section key={group.title} className="flex flex-col gap-4" aria-labelledby={group.title}>
          <h2 id={group.title} className="text-2xl font-semibold tracking-tight">
            {group.title}
          </h2>
          <div className="rounded-xl border bg-card">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Name</TableHead>
                  <TableHead>Used for</TableHead>
                  <TableHead>Author or organization</TableHead>
                  <TableHead>License</TableHead>
                  <TableHead>Link</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {group.rows.map((row) => (
                  <TableRow key={`${group.title}-${row.name}`}>
                    <TableCell className="font-medium">{row.name}</TableCell>
                    <TableCell>{row.purpose}</TableCell>
                    <TableCell>{row.author}</TableCell>
                    <TableCell>{row.license}</TableCell>
                    <TableCell>
                      <a
                        href={row.href}
                        rel="noopener noreferrer"
                        target="_blank"
                        className="font-medium underline underline-offset-4"
                      >
                        {row.linkLabel}
                      </a>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
        </section>
      ))}

      <section className="flex flex-col gap-3 border-t pt-8">
        <h2 className="text-xl font-semibold">License texts</h2>
        <p className="text-muted-foreground">
          Read the bundled{" "}
          <a href="/THIRD_PARTY_LICENSES.txt" className="font-medium underline underline-offset-4">
            third-party open-source licenses
          </a>
          .
        </p>
        <p className="text-muted-foreground">
          If I forgot you, tell me in the{" "}
          <a
            href={ISSUES_URL}
            rel="noopener noreferrer"
            target="_blank"
            className="font-medium underline underline-offset-4"
          >
            repository issues
          </a>
          .
        </p>
      </section>
    </main>
  );
}
