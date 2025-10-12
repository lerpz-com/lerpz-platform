import { useLocation } from "@solidjs/router";
import { Match, type ParentProps, Switch } from "solid-js";
import { useAuth } from "~/component/auth-context.tsx";
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from "~/component/ui/card.tsx";
import ChatSidebar from "./chat.tsx";
import DashboardSidebar from "./dashboard.tsx";
import VisualSidebar from "./visual.tsx";

type AppShellProps = ParentProps;

export function AppShell({ children }: AppShellProps) {
  const loc = useLocation();

  return (
    <div class="flex h-full w-full">
      <aside class="w-56 border-r">
        <SidebarHeader />
        <Switch>
          <Match when={loc.pathname.startsWith("/chat")}>
            <ChatSidebar />
          </Match>
          <Match when={loc.pathname.startsWith("/dashboard")}>
            <DashboardSidebar />
          </Match>
          <Match when={loc.pathname.startsWith("/visuals")}>
            <VisualSidebar />
          </Match>
        </Switch>
      </aside>
      <div class="grow">
        <header class="h-16 w-full border-b">
          <div>Search...</div>
        </header>
        <main class="max-w-[90rem] w-full m-auto p-6">{children}</main>
      </div>
    </div>
  );
}

function SidebarHeader() {
  // const { session } = useAuth();

  return (
    <Card class="m-4 p-4">
      <CardHeader>
        <CardTitle class="text-center">Lerpz</CardTitle>
        <CardDescription class="text-center">
          {/*{session()?.user?.email}*/}
        </CardDescription>
      </CardHeader>
    </Card>
  );
}
