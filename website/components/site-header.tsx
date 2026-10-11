"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { SunMoonIcon } from "lucide-react";
import { FullSearchTrigger } from "fumadocs-ui/layouts/shared/slots/search-trigger";
import { useTheme } from "fumadocs-ui/provider/base";
import { FaGithub } from "react-icons/fa";

import { Button } from "@/components/ui/button";
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
} from "@/components/ui/navigation-menu";
import { Separator } from "@/components/ui/separator";
import { SiteLogo } from "@/components/site-logo";
import { REPO_API_URL, REPO_URL } from "@/lib/config";

const navigation = [
  ["Open app", "/app"],
  ["Docs", "/docs"],
  ["Download", "/download"],
  ["About", "/about"],
] as const;

const CACHE_KEY = "amategeko-github-stars";
const TEN_MINUTES = 10 * 60 * 1000;

function formatStars(count: number): string {
  if (count >= 1000) {
    return `${(count / 1000).toFixed(1).replace(/\.0$/, "")}k`;
  }
  return count.toString();
}

export function SiteHeader() {
  const { resolvedTheme, setTheme } = useTheme();
  const [stars, setStars] = useState<number | null>(null);

  useEffect(() => {
    let cancelled = false;

    async function loadStars() {
      try {
        const cached = sessionStorage.getItem(CACHE_KEY);
        if (cached) {
          const parsed = JSON.parse(cached) as { savedAt: number; stars: number };
          if (Date.now() - parsed.savedAt < TEN_MINUTES) {
            if (!cancelled) setStars(parsed.stars);
            return;
          }
        }

        const res = await fetch(REPO_API_URL, {
          headers: { Accept: "application/vnd.github+json" },
        });
        if (!res.ok) return;
        const data = (await res.json()) as { stargazers_count?: number };
        if (typeof data.stargazers_count === "number") {
          sessionStorage.setItem(
            CACHE_KEY,
            JSON.stringify({ savedAt: Date.now(), stars: data.stargazers_count }),
          );
          if (!cancelled) setStars(data.stargazers_count);
        }
      } catch {
        // Silently fall back to default icon only if network or rate limit fails
      }
    }

    void loadStars();
    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <header className="sticky top-0 z-50 border-b bg-background/90 backdrop-blur-xl supports-[backdrop-filter]:bg-background/75">
      <div className="mx-auto flex h-16 max-w-6xl items-center gap-2 px-6 sm:px-8">
        <SiteLogo />

        <NavigationMenu className="ml-4 hidden md:flex" viewport={false}>
          <NavigationMenuList>
            {navigation.map(([label, href]) => (
              <NavigationMenuItem key={href}>
                <NavigationMenuLink asChild>
                  <Link href={href}>{label}</Link>
                </NavigationMenuLink>
              </NavigationMenuItem>
            ))}
          </NavigationMenuList>
        </NavigationMenu>

        <div className="ml-auto hidden items-center gap-2 md:flex">
          <FullSearchTrigger className="h-9 w-48 rounded-full bg-muted/40 px-3" />
        </div>

        <Button
          aria-label="Toggle color theme"
          onClick={() => setTheme(resolvedTheme === "dark" ? "light" : "dark")}
          size="icon"
          variant="ghost"
        >
          <SunMoonIcon aria-hidden="true" data-icon="inline-start" />
        </Button>
        <Button
          aria-label={
            stars !== null
              ? `Open the GitHub repository (${stars} stars)`
              : "Open the GitHub repository"
          }
          asChild
          size={stars !== null ? "sm" : "icon"}
          variant="ghost"
          className={stars !== null ? "h-9 gap-1.5 px-2.5 text-xs font-semibold" : undefined}
        >
          <a href={REPO_URL} rel="noopener noreferrer" target="_blank">
            <FaGithub aria-hidden="true" data-icon="inline-start" />
            {stars !== null && <span>{formatStars(stars)}</span>}
          </a>
        </Button>
        <Separator className="mx-1 hidden h-5 md:block" orientation="vertical" />
        <Button asChild className="rounded-full" size="sm">
          <Link href="/download">Download</Link>
        </Button>
      </div>
    </header>
  );
}
