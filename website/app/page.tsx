import HomePage, { metadata } from "@/components/home-page";
import { HomeLayout } from "fumadocs-ui/layouts/home";
import { baseOptions } from "@/app/layout.config";
import { SiteFooter } from "@/components/site-footer";

export { metadata };

export default function Page() {
  return (
    <HomeLayout {...baseOptions}>
      <HomePage />
      <SiteFooter />
    </HomeLayout>
  );
}
