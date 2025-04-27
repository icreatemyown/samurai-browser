import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import type { ParentProps } from "solid-js";

export function TanstackQueryClientProvider(props: ParentProps) {
	const queryClient = new QueryClient({
		defaultOptions: { queries: { staleTime: 0, refetchOnWindowFocus: true } },
	});
	return (
		<QueryClientProvider client={queryClient}>
			{props.children}
		</QueryClientProvider>
	);
}
