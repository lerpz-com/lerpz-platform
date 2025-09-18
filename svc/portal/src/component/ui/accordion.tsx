import type {
  AccordionItemContentProps,
  AccordionItemTriggerProps,
  AccordionItemProps as ArkAccordionItemProps
} from "@ark-ui/solid"
import { Accordion, AccordionItemIndicator } from "@ark-ui/solid"
import { ChevronDownIcon } from "lucide-solid"
import { ComponentProps, type ParentProps, splitProps } from "solid-js"
import { cn } from "~/lib/cn"

export const AccordionRoot = Accordion.Root

type AccordionItemProps = ComponentProps<"div"> &
  ArkAccordionItemProps & {
    class?: string
  }

export const AccordionItem = (props: AccordionItemProps) => {
  const [local, rest] = splitProps(props as AccordionItemProps, ["class"])

  return <Accordion.Item class={cn("border-b", local.class)} {...rest} />
}

type AccordionTriggerProps = ParentProps<
  ComponentProps<"button"> &
    AccordionItemTriggerProps & {
      class?: string
    }
>

export const AccordionTrigger = (props: AccordionTriggerProps) => {
  const [local, rest] = splitProps(props as AccordionTriggerProps, [
    "class",
    "children"
  ])

  return (
    <Accordion.ItemTrigger
      class={cn(
        "flex flex-1 items-center justify-between w-full py-4 \
        text-sm font-medium transition-shadow hover:underline \
        focus-visible:outline-none focus-visible:ring-[1.5px] \
        focus-visible:ring-ring",
        local.class
      )}
      {...rest}
    >
      {local.children}
      <AccordionItemIndicator class="[&[data-state=open]>svg]:rotate-180">
        <ChevronDownIcon class="transition-transform duration-200" />
      </AccordionItemIndicator>
    </Accordion.ItemTrigger>
  )
}

type AccordionContentProps = ParentProps<
  ComponentProps<"div"> &
    AccordionItemContentProps & {
      class?: string
    }
>

export const AccordionContent = (props: AccordionContentProps) => {
  const [local, rest] = splitProps(props as AccordionContentProps, [
    "class",
    "children"
  ])

  return (
    <Accordion.ItemContent
      class={cn(
        "overflow-hidden text-sm data-[state=closed]:animate-accordion-up \
        data-[state=open]:animate-accordion-down",
        local.class
      )}
      {...rest}
    >
      <div class="pb-4 pt-0">{local.children}</div>
    </Accordion.ItemContent>
  )
}
