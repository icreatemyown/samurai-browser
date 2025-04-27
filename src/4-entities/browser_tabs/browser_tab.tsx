import type { Tab } from "./browser_tabs.hook";
import { invoke } from "@tauri-apps/api/core";
export function BrowserTab(props: { tab: Tab }) {
	function setActive() {
		invoke("command_tabs_set_active", { id: props.tab.id });
	}
	return (
		<div
			class="px-4 py-2 rounded-t bg-linear-to-bl to-gray-300 from-gray-50 ring ring-gray-200 cursor-pointer"
			on:click={setActive}
		>
			Page: ({props.tab.url})
		</div>
	);
}
