import {
  Checkbox as ArkCheckbox,
  CheckboxControlProps as ArkCheckboxControlProps
} from "@ark-ui/solid/checkbox"
import type { ComponentProps, VoidProps } from "solid-js"
import { splitProps } from "solid-js"
import { cn } from "~/lib/cn"

export const CheckboxLabel = ArkCheckbox.Label
export const Checkbox = ArkCheckbox.Root

type CheckboxControlProps = VoidProps<
  ComponentProps<"div"> & ArkCheckboxControlProps & { class?: string }
>

export const CheckboxControl = (props: CheckboxControlProps) => {
  const [local, rest] = splitProps(props as CheckboxControlProps, [
    "class",
    "children"
  ])

  return (
    <>
      <ArkCheckbox.HiddenInput
        class="[&:focus-visible+div]:outline-none
        [&:focus-visible+div]:ring-[1.5px] [&:focus-visible+div]:ring-ring 
        [&:focus-visible+div]:ring-offset-2
        [&:focus-visible+div]:ring-offset-background"
      />
      <ArkCheckbox.Control
        class={cn(
          "h-4 w-4 shrink-0 rounded-sm border border-primary shadow \
          transition-shadow focus-visible:outline-none \
          focus-visible:ring-[1.5px] focus-visible:ring-ring \
          data-[disabled]:cursor-not-allowed data-[state=checked]:bg-primary \
          data-[state=checked]:text-primary-foreground data-[disabled]:opacity-50",
          local.class
        )}
        {...rest}
      >
        <ArkCheckbox.Indicator class="flex items-center justify-center text-current">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            class="h-4 w-4"
          >
            <path
              fill="none"
              stroke="currentColor"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="m5 12l5 5L20 7"
            />
            <title>Checkbox</title>
          </svg>
        </ArkCheckbox.Indicator>
      </ArkCheckbox.Control>
    </>
  )
}
