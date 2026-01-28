import { ComponentPropsWithRef } from "react";
import { cn } from "@/utils/utils";

export interface ContainerProps extends ComponentPropsWithRef<"main"> {
  className?: string;
}

export default function Container({
  children,
  className,
  ...props
}: ContainerProps) {
  return (
    <main className={cn("h-screen dark:bg-black", className)} {...props}>
      {children}
    </main>
  );
}
