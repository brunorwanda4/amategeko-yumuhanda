import { docs } from "@/.source/server";
import { loader } from "fumadocs-core/source";
import type * as PageTree from "fumadocs-core/page-tree";

export const source = loader({
  baseUrl: "/docs",
  source: docs.toFumadocsSource(),
});

export function buildDocsTree(): PageTree.Root {
  const root = source.pageTree;
  const itemMap = new Map<string, PageTree.Item>();

  function collect(nodes: PageTree.Node[]) {
    for (const node of nodes) {
      if (node.type === "page") {
        itemMap.set(node.url, node);
      } else if (node.type === "folder") {
        collect(node.children);
      }
    }
  }

  collect(root.children);

  const getItem = (url: string, overrideName?: string): PageTree.Node | undefined => {
    const item = itemMap.get(url);
    if (!item) return undefined;
    if (overrideName) {
      return { ...item, name: overrideName };
    }
    return item;
  };

  return {
    ...root,
    children: [
      {
        type: "folder",
        name: "Getting Started",
        defaultOpen: true,
        children: [
          getItem("/docs", "Introduction"),
          getItem("/docs/install", "Installation"),
          getItem("/docs/web-version", "Web Version"),
        ].filter(Boolean) as PageTree.Node[],
      },
      {
        type: "folder",
        name: "Using the App",
        defaultOpen: true,
        children: [
          getItem("/docs/quiz-modes"),
          getItem("/docs/screens"),
          getItem("/docs/settings"),
          getItem("/docs/shortcuts"),
        ].filter(Boolean) as PageTree.Node[],
      },
      {
        type: "folder",
        name: "Development",
        defaultOpen: false,
        children: [getItem("/docs/development")].filter(Boolean) as PageTree.Node[],
      },
    ],
  };
}

export const docsTree: PageTree.Root = buildDocsTree();
