import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export function HomeFeatures() {
  return (
    <div className="grid gap-4 md:grid-cols-2">
      <Card>
        <CardHeader>
          <Badge className="w-fit border-success bg-success-soft text-success-foreground" variant="outline">
            Free
          </Badge>
          <CardTitle className="pt-2 text-5xl tracking-tight">0 RWF</CardTitle>
        </CardHeader>
        <CardContent className="text-sm leading-6 text-muted-foreground">
          No subscriptions, no ads, and no locked features. Everything is open from the first
          launch.
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <Badge className="w-fit border-success bg-success-soft text-success-foreground" variant="outline">
            Offline
          </Badge>
          <CardTitle className="pt-2 text-5xl tracking-tight">0 MB data</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-sm leading-6 text-muted-foreground">
            Study without mobile data or Wi-Fi. Questions are stored on your device.
          </p>
          <div className="mt-5 flex items-center gap-2 rounded-lg border border-dashed px-3 py-2 font-mono text-xs text-muted-foreground">
            <span className="size-1.5 rounded-full bg-warning" aria-hidden="true" />
            No connection. Still working.
          </div>
        </CardContent>
      </Card>

      <Card className="overflow-hidden md:col-span-2">
        <CardContent className="grid gap-10 pt-1 md:grid-cols-[1fr_1.05fr] md:items-center">
          <div>
            <h3 className="text-2xl font-semibold tracking-tight">English and Kinyarwanda</h3>
            <p className="mt-3 text-sm leading-6 text-muted-foreground">
              Switch to the language that helps you learn, even in the middle of a quiz. Your
              progress stays the same.
            </p>
            <div className="mt-5 flex flex-wrap gap-2">
              <Badge variant="outline">English</Badge>
              <Badge variant="outline">Kinyarwanda</Badge>
              <Badge variant="outline">Instant switch</Badge>
            </div>
          </div>

          <div className="flex justify-center gap-4 md:-mb-16 md:-mt-2" aria-label="App result and quiz previews">
            <ResultPreview />
            <QuestionPreview />
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

function ResultPreview() {
  return (
    <div className="w-48 rounded-[1.75rem] border bg-background p-4 shadow-xl shadow-foreground/5">
      <p className="text-xs text-muted-foreground">Result</p>
      <p className="mt-2 text-4xl font-semibold tracking-tight">17/20</p>
      <p className="mt-1 text-xs text-muted-foreground">Passed</p>
      <div className="my-3 h-1.5 overflow-hidden rounded-full bg-muted">
        <div className="h-full w-4/5 bg-success" />
      </div>
      <PreviewLine color="bg-success" label="Road signs" />
      <PreviewLine color="bg-success" label="Right of way" />
      <PreviewLine color="bg-error" label="Parking" />
    </div>
  );
}

function QuestionPreview() {
  return (
    <div className="mt-8 w-48 rounded-[1.75rem] border bg-background p-4 shadow-xl shadow-foreground/5">
      <p className="text-xs text-muted-foreground">Question 5 of 20</p>
      <div className="mt-3 h-14 rounded-lg border" />
      <div className="mt-2 rounded-lg border border-success bg-success-soft px-3 py-2 text-xs text-success-foreground">
        Stop completely
      </div>
      <div className="mt-2 rounded-lg border px-3 py-2 text-xs">Slow down</div>
      <div className="mt-2 rounded-lg border px-3 py-2 text-xs">Continue if clear</div>
    </div>
  );
}

function PreviewLine({ color, label }: { color: string; label: string }) {
  return (
    <div className="mb-2 flex h-8 items-center gap-2 rounded-lg border px-2 text-xs">
      <span className={`size-1.5 rounded-full ${color}`} aria-hidden="true" />
      {label}
    </div>
  );
}
