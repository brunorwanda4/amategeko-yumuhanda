"use client";

import React from "react";
import { useDocsSearch } from "fumadocs-core/search/client";
import { staticClient } from "fumadocs-core/search/client/orama-static";
import {
  SearchDialog,
  SearchDialogClose,
  SearchDialogContent,
  SearchDialogFooter,
  SearchDialogHeader,
  SearchDialogIcon,
  SearchDialogInput,
  SearchDialogList,
  SearchDialogOverlay,
  type SharedProps,
} from "fumadocs-ui/components/dialog/search";

export default function StaticSearchDialog(props: SharedProps) {
  const { search, setSearch, query } = useDocsSearch({
    client: staticClient(),
  });

  return (
    <SearchDialog search={search} onSearchChange={setSearch} isLoading={query.isLoading} {...props}>
      <SearchDialogOverlay className="z-[100] bg-black/60 backdrop-blur-sm" />
      <SearchDialogContent className="z-[100] border-border bg-popover shadow-2xl">
        <SearchDialogHeader className="border-b border-border px-4 py-3">
          <SearchDialogIcon className="size-5 text-muted-foreground" />
          <SearchDialogInput className="text-base text-foreground placeholder:text-muted-foreground focus:outline-none focus-visible:outline-none focus:ring-0" />
          <SearchDialogClose />
        </SearchDialogHeader>
        <SearchDialogList items={query.data !== "empty" ? query.data : null} />
        <SearchDialogFooter />
      </SearchDialogContent>
    </SearchDialog>
  );
}
