import type {
  AccordionContextProps as ArkAccordionContextProps,
  AccordionItemContentProps as ArkAccordionItemContentProps,
  AccordionItemProps as ArkAccordionItemProps,
  AccordionItemTriggerProps as ArkAccordionItemTriggerProps,
} from "@ark-ui/solid";
import {
  Accordion as ArkAccordion,
  AccordionItemIndicator as ArkAccordionItemIndicator,
} from "@ark-ui/solid";
import { ChevronDownIcon } from "lucide-solid";
import type { ComponentProps, ParentProps, VoidProps } from "solid-js";
import { splitProps } from "solid-js";
import { cn } from "~/lib/cn.ts";

export const AccordionRoot = ArkAccordion.Root;

type AccordionItemProps = VoidProps<
  ArkAccordionItemProps & {
    class?: string;
  }
>;

export const AccordionItem = (props: AccordionItemProps) => {
  const [local, rest] = splitProps(props as AccordionItemProps, ["class"]);

  return (
    <ArkAccordion.Item
      class={cn("border-b [&[data-disabled]>button]:text-muted", local.class)}
      {...rest}
    />
  );
};

type AccordionTriggerProps = ParentProps<
  ArkAccordionItemTriggerProps & {
    class?: string;
  }
>;

export const AccordionTrigger = (props: AccordionTriggerProps) => {
  const [local, rest] = splitProps(props as AccordionTriggerProps, [
    "class",
    "children",
  ]);

  return (
    <ArkAccordion.ItemTrigger
      class={cn(
        "flex flex-1 items-center justify-between \
        w-full py-4 text-md font-semibold \
        focus-visible:outline-none \
        focus-visible:ring-[1.5px] \
        focus-visible:ring-ring",
        local.class,
      )}
      {...rest}
    >
      {local.children}
      <ArkAccordionItemIndicator class="[&[data-state=open]>svg]:rotate-180 [&[data-disabled]>svg]:text-muted">
        <ChevronDownIcon class="transition-transform duration-200" />
      </ArkAccordionItemIndicator>
    </ArkAccordion.ItemTrigger>
  );
};

type AccordionContentProps = ParentProps<
  ComponentProps<"div"> &
    ArkAccordionItemContentProps & {
      class?: string;
    }
>;

export const AccordionContent = (props: AccordionContentProps) => {
  const [local, rest] = splitProps(props as AccordionContentProps, [
    "class",
    "children",
  ]);

  return (
    <ArkAccordion.ItemContent
      class={cn(
        "overflow-hidden text-sm \
        data-[state=closed]:animate-accordion-up \
        data-[state=open]:animate-accordion-down",
        local.class,
      )}
      {...rest}
    >
      <div class="pb-4 pt-0">{local.children}</div>
    </ArkAccordion.ItemContent>
  );
};

type AccordionContextProps = ParentProps<
  ArkAccordionContextProps & {
    class?: string;
  }
>;

export const AccordionContext = (props: AccordionContextProps) => {
  const [local, rest] = splitProps(props as AccordionContextProps, [
    "children",
  ]);

  return (
    <ArkAccordion.Context {...rest}>{local.children}</ArkAccordion.Context>
  );
};
