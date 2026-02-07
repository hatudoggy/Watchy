export interface QueryMeta extends Record<string, unknown> {
  errorMessage?: string;
  successMessage?: string;

  log?: boolean;

  context?: Record<string, any>;
}

declare module "@tanstack/react-query" {
  interface Register {
    queryMeta: QueryMeta;
    mutationMeta: QueryMeta;
  }
}
