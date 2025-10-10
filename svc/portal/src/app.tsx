<<<<<<< Updated upstream
import { ColorModeProvider, ColorModeScript } from "@kobalte/core";
import { MetaProvider, Title } from "@solidjs/meta";
import { RouteDefinition, Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { SolidQueryDevtools } from "@tanstack/solid-query-devtools";
import { Suspense } from "solid-js";
import { AuthProvider } from "~/component/auth-context";
import { querySession } from "~/lib/auth/server";
import "./app.css";
=======
import { ColorModeProvider, ColorModeScript } from "@kobalte/core"
import { MetaProvider, Title } from "@solidjs/meta"
import { Router } from "@solidjs/router"
import { FileRoutes } from "@solidjs/start/router"
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query"
import { SolidQueryDevtools } from "@tanstack/solid-query-devtools"
import { Suspense } from "solid-js"
import { AuthProvider } from "~/component/auth-context"
import Wrapper from "~/component/wrapper"
import "./app.css"
>>>>>>> Stashed changes

const queryClient = new QueryClient();

<<<<<<< Updated upstream
export const route: RouteDefinition = {
	preload: () => querySession(),
};

export default function App() {
	return (
		<Router
			root={(props) => (
				<MetaProvider>
					<AuthProvider>
						<QueryClientProvider client={queryClient}>
							<Title>Lerpz portal</Title>
							<Suspense>
								<ColorModeScript />
								<ColorModeProvider>{props.children}</ColorModeProvider>
							</Suspense>
							<SolidQueryDevtools initialIsOpen={false} />
						</QueryClientProvider>
					</AuthProvider>
				</MetaProvider>
			)}
		>
			<FileRoutes />
		</Router>
	);
=======
export default function App() {
  return (
    <Router
      root={({ children }) => (
        <MetaProvider>
          <AuthProvider>
            <QueryClientProvider client={queryClient}>
              <Title>Lerpz portal</Title>
              <Suspense>
                <ColorModeScript />
                <ColorModeProvider>
                  <Wrapper>{children}</Wrapper>
                </ColorModeProvider>
              </Suspense>
              <SolidQueryDevtools initialIsOpen={false} />
            </QueryClientProvider>
          </AuthProvider>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  )
>>>>>>> Stashed changes
}
