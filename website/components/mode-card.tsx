import type { LucideIcon } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { cn } from "@/lib/utils";

type ModeCardProps = {
  name: string;
  description: string;
  badge: string;
  badgeClassName: string;
  Icon: LucideIcon;
  bars: number;
};

export function ModeCard({ name, description, badge, badgeClassName, Icon, bars }: ModeCardProps) {
  return (
    <Card className="h-full gap-0">
      <CardHeader>
        <div className="flex items-center justify-between gap-4">
          <span className="grid size-11 place-items-center rounded-xl bg-muted">
            <Icon aria-hidden="true" className="size-5" />
          </span>
          <Badge className={badgeClassName} variant="outline">{badge}</Badge>
        </div>
        <CardTitle className="pt-5 text-xl">{name}</CardTitle>
      </CardHeader>
      <CardContent className="mt-2 text-sm leading-6 text-muted-foreground">{description}</CardContent>
      <CardFooter className="mt-5 gap-1.5" aria-hidden="true">
        {[0, 1, 2].map((index) => (
          <span className={cn("h-1.5 flex-1 rounded-full bg-muted", index < bars && "bg-foreground")} key={index} />
        ))}
      </CardFooter>
    </Card>
  );
}
