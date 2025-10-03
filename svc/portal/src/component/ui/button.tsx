import type { VariantProps } from "class-variance-authority"
import { cva } from "class-variance-authority"
import { ComponentProps, splitProps } from "solid-js"
import { cn } from "~/lib/cn"

export const buttonVariants = cva(
  "inline-flex items-center justify-center rounded-md text-sm font-medium \
  transition-[color,background-color,box-shadow] focus-visible:outline-none \
  focus-visible:ring-[1.5px] focus-visible:ring-ring \
  disabled:pointer-events-none disabled:opacity-50",
  {
    variants: {
      variant: {
        default:
          "bg-primary text-primary-foreground shadow hover:bg-primary/80",
        destructive:
          "bg-destructive text-destructive-foreground shadow hover:bg-destructive/80",
        outline:
          "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
        secondary:
          "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80",
        ghost: "hover:bg-accent hover:text-accent-foreground",
        link: "text-primary underline-offset-4 hover:underline"
      },
      size: {
        sm: "h-8 px-4 text-sm",
        md: "h-10 px-6 text-md",
        lg: "h-12 px-8 text-lg"
      }
    },
    defaultVariants: {
      variant: "default",
      size: "md"
    }
  }
)

type ButtonProps = ComponentProps<"button"> &
  VariantProps<typeof buttonVariants> & {
    class?: string
  }

export const Button = (props: ButtonProps) => {
  const [local, rest] = splitProps(props as ButtonProps, [
    "class",
    "variant",
    "size"
  ])

  return (
    <button
      class={cn(
        buttonVariants({
          size: local.size,
          variant: local.variant
        }),
        local.class
      )}
      {...rest}
    />
  )
}
