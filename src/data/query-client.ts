import { MutationCache, QueryCache, QueryClient } from "@tanstack/react-query";

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      refetchOnWindowFocus: false,
      staleTime: 30000,
    },
  },
  queryCache: new QueryCache({
    onError: (err, query) => {
      if (query.state.data === undefined) {
        console.error(`Query Error [${query.queryKey}]:`, err);
      }
    },
  }),
  mutationCache: new MutationCache({
    onError: (err, _vars, _ctx, _mut) => {
      console.error(`Mutation Error:`, err);
    },
  }),
});
