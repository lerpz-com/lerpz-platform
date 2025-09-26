import { A } from "@solidjs/router"
import { HttpStatusCode } from "@solidjs/start"
import { Show } from "solid-js"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from "~/component/ui/card"

interface Problem<D = unknown> {
  type: string
  title: string
  detail: string
  instance: string | null
  extension: D
  log_id: string | null
}

export default function NotFound() {
  const problem: Problem = {
    type: "/docs/errors/not-found",
    title: "Not Found",
    detail: "The requested resource was not found.",
    instance: null,
    extension: {},
    log_id: null
  }

  return (
    <>
      <HttpStatusCode code={404} />
      <Card class="max-w-2xl">
        <CardHeader>
          <CardTitle class="text-center">404 - Not Found</CardTitle>
          <CardDescription class="text-center">
            The page you were looking for was not found.
          </CardDescription>
          <Show when={problem.type != "about:blank"}>
            <A class="text-center" href={problem.type}>
              Read more here.
            </A>
          </Show>
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
    </>
  )
}
