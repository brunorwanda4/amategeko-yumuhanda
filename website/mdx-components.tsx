import type { ComponentProps, ReactNode } from "react";
import type { MDXComponents } from "mdx/types";
import defaultComponents from "fumadocs-ui/mdx";
import Link from "next/link";
import {
  CheckCircle2,
  AlertTriangle,
  AlertCircle,
  Info,
  ExternalLink,
} from "lucide-react";
import { Tabs as FumadocsTabs, Tab as FumadocsTab } from "fumadocs-ui/components/tabs";
import { Steps as FumadocsSteps, Step as FumadocsStep } from "fumadocs-ui/components/steps";
import { cn } from "@/lib/utils";

// Stats component: 3-column grid (1 on mobile), 32px numbers
export function Stats({
  children,
  className,
  ...props
}: ComponentProps<"div">) {
  return (
    <div
      className={cn("my-8 grid grid-cols-1 gap-4 sm:grid-cols-3", className)}
      {...props}
    >
      {children}
    </div>
  );
}

export function Stat({
  value,
  label,
  className,
  ...props
}: {
  value: ReactNode;
  label: ReactNode;
  className?: string;
} & ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "flex flex-col rounded-2xl border border-border bg-card p-5 transition-colors",
        className,
      )}
      {...props}
    >
      <span className="font-mono text-[32px] font-semibold leading-none tracking-tight text-foreground">
        {value}
      </span>
      <span className="mt-2 text-sm text-muted-foreground">{label}</span>
    </div>
  );
}

// Facts key-value table in a rounded bordered box, with pill chips for platforms
interface FactItem {
  label: string;
  value?: ReactNode;
  platforms?: string[];
}

export function Facts({
  items,
  children,
  className,
  ...props
}: {
  items?: FactItem[];
  children?: ReactNode;
  className?: string;
} & ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "my-8 divide-y divide-border overflow-hidden rounded-2xl border border-border bg-card",
        className,
      )}
      {...props}
    >
      {items
        ? items.map((item, idx) => (
            <div
              key={idx}
              className="flex flex-col justify-between gap-2 px-5 py-3.5 text-sm sm:flex-row sm:items-center"
            >
              <span className="font-medium text-foreground">{item.label}</span>
              <div className="flex flex-wrap items-center gap-2 text-muted-foreground">
                {item.platforms?.map((p) => (
                  <span
                    key={p}
                    className="inline-flex items-center rounded-full border border-border bg-accent px-2.5 py-0.5 text-xs font-medium text-foreground"
                  >
                    {p}
                  </span>
                ))}
                {item.value && <span>{item.value}</span>}
              </div>
            </div>
          ))
        : children}
    </div>
  );
}

export function FactRow({
  label,
  platforms,
  children,
  className,
}: {
  label: ReactNode;
  platforms?: string[];
  children?: ReactNode;
  className?: string;
}) {
  return (
    <div
      className={cn(
        "flex flex-col justify-between gap-2 px-5 py-3.5 text-sm sm:flex-row sm:items-center",
        className,
      )}
    >
      <span className="font-medium text-foreground">{label}</span>
      <div className="flex flex-wrap items-center gap-2 text-muted-foreground">
        {platforms?.map((p) => (
          <span
            key={p}
            className="inline-flex items-center rounded-full border border-border bg-accent px-2.5 py-0.5 text-xs font-medium text-foreground"
          >
            {p}
          </span>
        ))}
        {children}
      </div>
    </div>
  );
}

// Callout variants using semantic colors (success, warning, error, info), each with an icon
export function Callout({
  type = "info",
  icon,
  title,
  children,
  className,
  ...props
}: {
  type?: "info" | "warning" | "error" | "success" | "warn" | "tip";
  icon?: ReactNode;
  title?: ReactNode;
  children?: ReactNode;
  className?: string;
} & ComponentProps<"div">) {
  const normalizedType =
    type === "warn" ? "warning" : type === "tip" ? "info" : type;

  const typeStyles = {
    info: "bg-[#dbeafe] text-[#1d4ed8] dark:bg-[#0c2340] dark:text-[#93c5fd] border-[#1d4ed8]/20 dark:border-[#93c5fd]/20",
    warning:
      "bg-[#fef3c7] text-[#b45309] dark:bg-[#2a1c03] dark:text-[#fbbf24] border-[#b45309]/20 dark:border-[#fbbf24]/20",
    error:
      "bg-[#fee2e2] text-[#dc2626] dark:bg-[#2d0a0a] dark:text-[#f87171] border-[#dc2626]/20 dark:border-[#f87171]/20",
    success:
      "bg-[#dcfce7] text-[#16a34a] dark:bg-[#052e16] dark:text-[#4ade80] border-[#16a34a]/20 dark:border-[#4ade80]/20",
  }[normalizedType] || "bg-card text-foreground border-border";

  const defaultIcon = {
    info: <Info className="size-5 shrink-0" aria-hidden="true" />,
    warning: <AlertTriangle className="size-5 shrink-0" aria-hidden="true" />,
    error: <AlertCircle className="size-5 shrink-0" aria-hidden="true" />,
    success: <CheckCircle2 className="size-5 shrink-0" aria-hidden="true" />,
  }[normalizedType];

  return (
    <div
      className={cn(
        "my-6 flex items-start gap-3.5 rounded-2xl border p-4 text-sm leading-relaxed",
        typeStyles,
        className,
      )}
      {...props}
    >
      <span className="mt-0.5">{icon ?? defaultIcon}</span>
      <div className="flex-1 min-w-0">
        {title && (
          <p className="mb-1 font-semibold text-inherit">{title}</p>
        )}
        <div className="[&>p]:m-0 [&>p+p]:mt-2 text-inherit opacity-95">
          {children}
        </div>
      </div>
    </div>
  );
}

// Feature cards: icon tile (40px), title, muted description, border darkens on hover
export function Cards({
  children,
  className,
  ...props
}: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "my-8 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3",
        className,
      )}
      {...props}
    >
      {children}
    </div>
  );
}

export function Card({
  icon,
  title,
  description,
  href,
  children,
  className,
  ...props
}: {
  icon?: ReactNode;
  title?: ReactNode;
  description?: ReactNode;
  href?: string;
  children?: ReactNode;
  className?: string;
} & ComponentProps<"div">) {
  const content = (
    <div
      data-card="true"
      className={cn(
        "group flex flex-col rounded-2xl border border-border bg-card p-6 transition-colors hover:border-foreground/40",
        href && "cursor-pointer",
        className,
      )}
      {...props}
    >
      {icon && (
        <div className="mb-4 flex size-10 shrink-0 items-center justify-center rounded-xl border border-border bg-background text-foreground transition-colors group-hover:border-foreground/30">
          {icon}
        </div>
      )}
      {title && (
        <h3 className="mb-1 text-base font-semibold tracking-[-0.02em] text-foreground">
          {title}
        </h3>
      )}
      {description && (
        <p className="m-0 text-sm leading-relaxed text-muted-foreground">
          {description}
        </p>
      )}
      {children && (
        <div className="mt-2 text-sm leading-relaxed text-muted-foreground">
          {children}
        </div>
      )}
    </div>
  );

  if (href) {
    return (
      <Link href={href} className="no-underline">
        {content}
      </Link>
    );
  }

  return content;
}

// Mode level pills: green, amber, red
export function Mode({
  level,
  children,
  className,
  ...props
}: {
  level?: "easy" | "medium" | "hard" | string;
  children?: ReactNode;
  className?: string;
} & ComponentProps<"span">) {
  const normalized = (level || "easy").toLowerCase();
  const styles = {
    easy: "bg-[#dcfce7] text-[#16a34a] dark:bg-[#052e16] dark:text-[#4ade80] border-[#16a34a]/25 dark:border-[#4ade80]/25",
    medium:
      "bg-[#fef3c7] text-[#b45309] dark:bg-[#2a1c03] dark:text-[#fbbf24] border-[#b45309]/25 dark:border-[#fbbf24]/25",
    hard: "bg-[#fee2e2] text-[#dc2626] dark:bg-[#2d0a0a] dark:text-[#f87171] border-[#dc2626]/25 dark:border-[#f87171]/25",
  }[normalized as "easy" | "medium" | "hard"] ||
    "bg-accent text-foreground border-border";

  const label =
    children || normalized.charAt(0).toUpperCase() + normalized.slice(1);

  return (
    <span
      className={cn(
        "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-medium",
        styles,
        className,
      )}
      {...props}
    >
      {label}
    </span>
  );
}

// Steps and Tabs (extended from fumadocs-ui)
export const Steps = FumadocsSteps;
export const Step = FumadocsStep;
export const Tabs = FumadocsTabs;
export const Tab = FumadocsTab;

export function useMDXComponents(
  components?: MDXComponents,
): MDXComponents {
  return {
    ...defaultComponents,
    Stats,
    Stat,
    Facts,
    FactRow,
    Callout,
    Cards,
    Card,
    Mode,
    Steps,
    Step,
    Tabs,
    Tab,
    ...components,
  };
}

export default useMDXComponents;
