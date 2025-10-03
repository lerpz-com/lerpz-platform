import { ParentProps } from "solid-js"

export function AppShell({ children }: ParentProps) {
  return (
    <div class="flex h-full w-full">
      <aside class="w-56 border-r-2">
        <div>Lerpz</div>
      </aside>
      <div class="grow">
        <header class="h-16 w-full border-b-2">
          <div>Search...</div>
        </header>
        <main class="max-w-[90rem] w-full m-auto p-6">{children}</main>
      </div>
    </div>
  )
}
