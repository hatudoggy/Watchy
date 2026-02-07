import { MutationCache, QueryCache, QueryClient } from "@tanstack/react-query";

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: Infinity,
      networkMode: "always",
      refetchOnWindowFocus: false,
      retry: false,
    },
  },
  queryCache: new QueryCache({
    onError: (err, query) => {
      const meta = query.meta;
      if (meta?.errorMessage) {
      }
      if (meta?.log === true)
        console.error(
          `Query Error [${query.queryKey}]:`,
          err,
          meta.errorMessage,
        );
    },
    onSuccess: (_data, query) => {
      const meta = query.meta;
      if (meta?.successMessage) {
      }
      if (meta?.log === true)
        console.log(`Query Success [${query.queryKey}]:`, meta.successMessage);
    },
  }),
  mutationCache: new MutationCache({
    onError: (err, _vars, _ctx, mut) => {
      const meta = mut.meta;
      if (meta?.errorMessage) {
      }
      if (meta?.log === true)
        console.error(
          `Mutation Error [${mut.options.mutationKey}]:`,
          err,
          meta.errorMessage,
        );
    },
    onSuccess: (_data, _vars, _ctx, mut) => {
      const meta = mut.meta;
      if (meta?.successMessage) {
      }
      if (meta?.log === true)
        console.log(
          `Mutation Success  [${mut.options.mutationKey}]:`,
          meta.successMessage,
        );
    },
  }),
});
