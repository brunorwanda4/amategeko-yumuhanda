"use client";

import { useState } from "react";
import { Check, Copy } from "lucide-react";
import { buttonVariants } from "fumadocs-ui/components/ui/button";
import { cn } from "@/lib/utils";

export function CopyMarkdownButton({
  markdown,
  className,
}: {
  markdown: string;
  className?: string;
}) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(markdown);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error("Failed to copy markdown", err);
    }
  };

  return (
    <button
      type="button"
      onClick={handleCopy}
      className={cn(
        buttonVariants({
          variant: "secondary",
          size: "sm",
        }),
        "gap-2 text-xs font-medium cursor-pointer transition-colors",
        className
      )}
    >
      {copied ? (
        <Check className="size-3.5 text-fd-primary" />
      ) : (
        <Copy className="size-3.5 text-fd-muted-foreground" />
      )}
      <span>{copied ? "Copied Markdown" : "Copy Markdown"}</span>
    </button>
  );
}
