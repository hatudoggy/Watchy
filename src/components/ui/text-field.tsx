import { CloseButton, TextInput as MantineTextInput } from "@mantine/core";
import { ComponentPropsWithRef, ReactNode } from "react";
import { cn } from "@/utils/utils";
import { InputVariants, Sizes } from "./types";

export interface TextFieldProps extends Omit<
  ComponentPropsWithRef<"input">,
  "size"
> {
  variant?: InputVariants;
  className?: string;
  size?: Sizes;
  leftSection?: ReactNode;
  clearable?: boolean;
  onClear?: () => void;
}

export default function TextField({
  variant,
  className,
  leftSection,
  clearable,
  onClear,
  ...props
}: TextFieldProps) {
  return (
    <MantineTextInput
      variant={variant}
      className={cn(className)}
      leftSection={leftSection}
      rightSection={
        clearable &&
        props.value && (
          <CloseButton
            size="sm"
            onMouseDown={(event) => event.preventDefault()}
            onClick={() => {
              onClear?.();
              if (!onClear) {
                const event = {
                  target: { value: "" },
                  currentTarget: { value: "" },
                } as React.ChangeEvent<HTMLInputElement>;
                props.onChange?.(event);
              }
            }}
            aria-label="Clear value"
          />
        )
      }
      {...props}
    />
  );
}
