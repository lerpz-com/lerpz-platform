import { ParentProps } from "solid-js"

export default function Wrapper(props: ParentProps) {
  return <main class="max-w-[90rem]">{props.children}</main>
}
