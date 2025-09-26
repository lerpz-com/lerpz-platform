import { ParentProps } from "solid-js"

export default function Wrapper(props: ParentProps) {
  return (
    <main class="flex justify-center">
      <div class="max-w-[90rem] w-full">{props.children}</div>
    </main>
  )
}
