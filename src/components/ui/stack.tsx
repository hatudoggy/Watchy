import {
  Stack as MantineStack,
  StackProps as MantineStackProps,
} from "@mantine/core";
import { cn } from "@/utils/utils";

export interface StackProps extends MantineStackProps {}

export default function Stack({ className, ...props }: StackProps) {
  return <MantineStack className={cn(className)} {...props} />;
}
