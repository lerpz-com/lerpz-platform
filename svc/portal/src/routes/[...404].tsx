import { HttpStatusCode } from "@solidjs/start"

export function NotFound() {
  return (
    <>
      <HttpStatusCode code={404} />
      <div>404 - Not Found</div>
    </>
  )
}
