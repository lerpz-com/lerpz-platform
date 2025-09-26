import { cva, VariantProps } from "class-variance-authority"
import type { ComponentProps, ParentComponent } from "solid-js"
import { splitProps } from "solid-js"
import { cn } from "~/lib/cn"

export const cardVariants = cva("rounded-lg border shadow p-6", {
  variants: {
    variant: {
      default: "bg-card text-card-foreground",
      outline: "bg-background text-card-foreground"
    }
  },
  defaultVariants: {
    variant: "default"
  }
})

type CardProps = ComponentProps<"div"> &
  VariantProps<typeof cardVariants> & {
    class?: string
  }

export const Card = (props: CardProps) => {
  const [local, rest] = splitProps(props, ["variant", "class"])

  return (
    <div
      class={cn(cardVariants({ variant: local.variant }), local.class)}
      {...rest}
    />
  )
}

export const CardHeader = (props: ComponentProps<"div">) => {
  const [local, rest] = splitProps(props, ["class"])

  return <div class={cn("flex flex-col space-y-1.5", local.class)} {...rest} />
}

export const CardTitle: ParentComponent<ComponentProps<"h1">> = (props) => {
  const [local, rest] = splitProps(props, ["class"])

  return (
    <h1
      class={cn("text-xl font-semibold leading-none", local.class)}
      {...rest}
    />
  )
}

export const CardDescription: ParentComponent<ComponentProps<"h3">> = (
  props
) => {
  const [local, rest] = splitProps(props, ["class"])

  return (
    <h3 class={cn("text-md text-muted-foreground", local.class)} {...rest} />
  )
}

export const CardContent = (props: ComponentProps<"div">) => {
  const [local, rest] = splitProps(props, ["class"])

  return <div class={cn("pt-2", local.class)} {...rest} />
}

export const CardFooter = (props: ComponentProps<"div">) => {
  const [local, rest] = splitProps(props, ["class"])

  return <div class={cn("pt-2", local.class)} {...rest} />
}
