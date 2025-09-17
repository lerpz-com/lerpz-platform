import { ColorModeProvider, ColorModeScript } from "@kobalte/core"
import { MetaProvider, Title } from "@solidjs/meta"
import { Router } from "@solidjs/router"
import { FileRoutes } from "@solidjs/start/router"
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query"
import { SolidQueryDevtools } from "@tanstack/solid-query-devtools"
import { Suspense } from "solid-js"
import "./app.css"

const queryClient = new QueryClient()

export default function App() {
  return (
    <Router
      root={(props) => (
        <MetaProvider>
          <QueryClientProvider client={queryClient}>
            <Title>Lerpz portal</Title>
            <Suspense>
              <ColorModeScript />
              <ColorModeProvider>{props.children}</ColorModeProvider>
            </Suspense>
            <SolidQueryDevtools initialIsOpen={false} />
          </QueryClientProvider>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  )
}
