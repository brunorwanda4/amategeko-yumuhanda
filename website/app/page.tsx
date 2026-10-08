import HomePage, { metadata } from "@/components/home-page";
import { HomeShell } from "@/components/home-shell";

export { metadata };

export default function Page() {
  return (
    <HomeShell>
      <HomePage />
    </HomeShell>
  );
}
