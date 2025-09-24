import {
  Checkbox as ArkCheckbox,
  CheckboxControlProps as ArkCheckboxControlProps,
  CheckboxRootProps as ArkCheckboxRootProps,
  CheckboxLabelProps as ArkCheckboxLabelProps
} from "@ark-ui/solid/checkbox"
import type { ParentProps, VoidProps } from "solid-js"
import { splitProps } from "solid-js"
import { cn } from "~/lib/cn"

export { useCheckbox } from "@ark-ui/solid/checkbox"

export const CheckboxContext = ArkCheckbox.Context
export const CheckboxGroup = ArkCheckbox.Group
export const CheckboxGroupProvider = ArkCheckbox.GroupProvider

export const CheckboxProvider = ArkCheckbox.RootProvider

type CheckboxProps = ParentProps<ArkCheckboxRootProps & { class?: string }>

export const Checkbox = (props: CheckboxProps) => {
  const [local, rest] = splitProps(props as CheckboxProps, [
    "class",
    "children"
  ])

  return (
    <ArkCheckbox.Root
      class={cn("flex items-center gap-2", local.class)}
      {...rest}
    >
      {local.children}
    </ArkCheckbox.Root>
  )
}

type CheckboxControlProps = VoidProps<
  ArkCheckboxControlProps & { class?: string }
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
        [&:focus-visible+div]:ring-[1.5px]
        [&:focus-visible+div]:ring-ring 
        [&:focus-visible+div]:ring-offset-2
        [&:focus-visible+div]:ring-offset-background"
      />
      <ArkCheckbox.Control
        class={cn(
          "h-4 w-4 shrink-0 rounded-sm border border-primary shadow \
          transition-shadow focus-visible:outline-none \
          focus-visible:ring-[1.5px] focus-visible:ring-ring \
          data-[disabled]:cursor-not-allowed \
          data-[disabled]:opacity-50 \
          data-[state=checked]:bg-primary \
          data-[state=checked]:text-primary-foreground ",
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

type CheckboxLabelProps = ParentProps<
  ArkCheckboxLabelProps & { class?: string }
>

export const CheckboxLabel = (props: CheckboxLabelProps) => {
  const [local, rest] = splitProps(props as CheckboxLabelProps, [
    "class",
    "children"
  ])

  return (
    <ArkCheckbox.Label
      class={cn("select-none data-[disabled]:opacity-50", local.class)}
      {...rest}
    >
      {local.children}
    </ArkCheckbox.Label>
  )
}
