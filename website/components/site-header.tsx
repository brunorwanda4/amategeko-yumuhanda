"use client";

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
import { REPO_URL } from "@/lib/config";

const navigation = [
  ["Open app", "/app"],
  ["Docs", "/docs"],
  ["Download", "/download"],
  ["About", "/about"],
] as const;

export function SiteHeader() {
  const { resolvedTheme, setTheme } = useTheme();

  return (
    <header className="sticky top-0 z-50 border-b bg-background/90 backdrop-blur-xl supports-[backdrop-filter]:bg-background/75">
      <div className="mx-auto flex h-16 max-w-6xl items-center gap-2 px-5 sm:px-8">
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
        <Button aria-label="Open the GitHub repository" asChild size="icon" variant="ghost">
          <a href={REPO_URL} rel="noopener noreferrer" target="_blank">
            <FaGithub aria-hidden="true" data-icon="inline-start" />
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
