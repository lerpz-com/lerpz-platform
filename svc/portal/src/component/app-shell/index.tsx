import { ParentProps } from "solid-js";
import { useAuth } from "~/component/auth-context";
import { Card, CardDescription, CardHeader, CardTitle } from "../ui/card";

type AppShellKind = "chat" | "visual" | "dashboard";

type AppShellProps = ParentProps<{
  kind: AppShellKind;
}>;

export function AppShell({ children, kind }: AppShellProps) {
  return (
    <div class="flex h-full w-full">
      <aside class="w-56 border-r">
        <SidebarHeader />
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
  const { session } = useAuth();

  return (
    <Card class="m-4 p-4">
      <CardHeader>
        <CardTitle class="text-center">Lerpz</CardTitle>
        <CardDescription class="text-center">
          {session()?.user?.email}
        </CardDescription>
      </CardHeader>
    </Card>
  );
}
