import { ParentProps } from "solid-js"

export default function Wrapper(props: ParentProps) {
  return <main class="wrapper">{props.children}</main>
}
