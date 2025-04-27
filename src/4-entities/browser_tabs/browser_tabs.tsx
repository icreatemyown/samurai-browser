import { For } from "solid-js";
import { useAction } from "../actions/action.provider";
import { useBrowserTabs } from "./browser_tabs.hook";
import { BrowserTab } from "./browser_tab.tsx";

export function BrowserTabs() {
	const { tabs, addTab } = useBrowserTabs();
	const action = useAction();
	action.on("tabs:add", addTab);

	return (
		<div class="flex -space-x-2">
			<For each={tabs()}>{(tab) => <BrowserTab tab={tab} />}</For>
		</div>
	);
}
