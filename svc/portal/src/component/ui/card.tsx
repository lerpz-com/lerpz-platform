import { cva, VariantProps } from "class-variance-authority";
import type { ComponentProps } from "solid-js";
import { splitProps } from "solid-js";
import { cn } from "~/lib/cn";

export const cardVariants = cva("rounded-lg border shadow p-6", {
	variants: {
		variant: {
			default: "bg-card text-card-foreground",
			outline: "bg-background text-foreground",
		},
	},
	defaultVariants: {
		variant: "default",
	},
});

type CardProps = ComponentProps<"div"> &
	VariantProps<typeof cardVariants> & {
		class?: string;
	};

export const Card = (props: CardProps) => {
	const [local, rest] = splitProps(props, ["variant", "class"]);

	return (
		<div
			class={cn(cardVariants({ variant: local.variant }), local.class)}
			{...rest}
		/>
	);
};

type CardHeaderProps = ComponentProps<"div"> & {
	class?: string;
};

export const CardHeader = (props: CardHeaderProps) => {
	const [local, rest] = splitProps(props, ["class"]);

	return <div class={cn("flex flex-col space-y-1.5", local.class)} {...rest} />;
};

type CardTitleProps = ComponentProps<"h1"> & {
	class?: string;
};

export const CardTitle = (props: CardTitleProps) => {
	const [local, rest] = splitProps(props, ["class"]);

	return (
		<h1
			class={cn("text-lg font-semibold leading-none", local.class)}
			{...rest}
		/>
	);
};

export type CardDescriptionProps = ComponentProps<"h3"> & {
	class?: string;
};

export const CardDescription = (props: CardDescriptionProps) => {
	const [local, rest] = splitProps(props, ["class"]);

	return (
		<h3
			class={cn("text-md font-medium text-muted-foreground", local.class)}
			{...rest}
		/>
	);
};

export type CardContentProps = ComponentProps<"div"> & {
	class?: string;
};

export const CardContent = (props: CardContentProps) => {
	const [local, rest] = splitProps(props, ["class"]);

	return <div class={cn("pt-2", local.class)} {...rest} />;
};

export type CardFooterProps = ComponentProps<"div"> & {
	class?: string;
};

export const CardFooter = (props: CardFooterProps) => {
	const [local, rest] = splitProps(props, ["class"]);

	return <div class={cn("pt-2", local.class)} {...rest} />;
};
