import { createSignal } from "solid-js";
import { BrowserHeader } from "../3-widgets/browser_header.tsx";
import { WindowLayout } from "../4-entities/window/window.layout";
import { TanstackQueryClientProvider } from "../5-shared/lib/query-client/tanstack.provider";

export type Tab = { kind: "search" | "ip" | "url"; src: string };

export function App() {
	const [pageUrl] = createSignal("https://google.com");

	return (
		<TanstackQueryClientProvider>
			<WindowLayout>
				<BrowserHeader url={pageUrl()} />
			</WindowLayout>
		</TanstackQueryClientProvider>
	);
}
