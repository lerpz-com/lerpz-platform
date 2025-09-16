import { ColorModeProvider, ColorModeScript } from "@kobalte/core"
import { MetaProvider, Title } from "@solidjs/meta"
import { Router } from "@solidjs/router"
import { FileRoutes } from "@solidjs/start/router"
import { Suspense } from "solid-js"
import "./app.css"

export default function App() {
  return (
    <Router
      root={(props) => (
        <MetaProvider>
          <Title>Lerpz portal</Title>
          <Suspense>
            <ColorModeScript />
            <ColorModeProvider>{props.children}</ColorModeProvider>
          </Suspense>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  )
}
