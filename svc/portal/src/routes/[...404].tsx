import { A } from "@solidjs/router";
import { HttpStatusCode } from "@solidjs/start";
import { Show } from "solid-js";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "~/component/ui/card.tsx";

interface Problem<D = unknown> {
  type: string;
  title: string;
  detail: string;
  instance: string | null | undefined;
  extension: D | null | undefined;
  log_id: string | null | undefined;
}

export default function NotFound() {
  const problem: Problem = {
    type: "/docs/errors/not-found",
    title: "Not Found",
    detail: "The requested resource was not found.",
    instance: null,
    extension: {},
    log_id: "8cd9d8ce-5dbc-4a1a-ac58-5ad918edb67d",
  };

  return (
    <main class="w-full h-full flex justify-center items-center">
      <HttpStatusCode code={404} />
      <Card class="max-w-2xl m-4">
        <CardHeader>
          <CardTitle class="text-center">404 - Not Found</CardTitle>
          <CardDescription class="text-center">
            The page you were looking for was not found.
          </CardDescription>
          <Show when={problem.type !== "about:blank"}>
            <A class="text-center underline" href={problem.type}>
              Read more here
            </A>
          </Show>
          <p class="text-center text-muted-foreground">{problem.log_id}</p>
        </CardHeader>
        <CardContent>
          <Card variant="outline">
            <CardContent>
              <pre class="whitespace-pre-wrap">
                {JSON.stringify(problem, null, 2)}
              </pre>
            </CardContent>
          </Card>
        </CardContent>
      </Card>
    </main>
  );
}
