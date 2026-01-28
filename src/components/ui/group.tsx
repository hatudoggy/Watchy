import {
  Group as MantineGroup,
  GroupProps as MantineGroupProps,
} from "@mantine/core";
import { cn } from "@/utils/utils";

export interface GroupProps extends MantineGroupProps {}

export default function Group({ className, ...props }: GroupProps) {
  return <MantineGroup className={cn(className)} {...props} />;
}
