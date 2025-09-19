import { ParentProps } from "solid-js"

export default function Wrapper(props: ParentProps) {
  return <main class="p-4">{props.children}</main>
}
