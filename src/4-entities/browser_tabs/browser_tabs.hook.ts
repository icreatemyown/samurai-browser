import { useMutation, useQuery } from "@tanstack/solid-query";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { onMount } from "solid-js";
import { useAutoClean } from "../../5-shared/lib/cleanup/cleanup.ts";

export type TabAddr = {
	kind: "search" | "url";
	src: string;
};
export type Tab = {
	addr: TabAddr;
	label: string;
};

export function useBrowserTabs() {
	const autoclean = useAutoClean();
	const queryTabs = useQuery(() => ({
		queryKey: ["tabs"],
		async queryFn() {
			const tabs = await invoke<Tab[]>("tabs_get");
			console.log(tabs);
			return tabs;
		},
		initialData: [],
	}));

	onMount(async () => {
		const unlisten = await listen<null>("tabs:list:changed", () =>
			queryTabs.refetch(),
		);
		autoclean(unlisten);
	});

	const mutationAddTab = useMutation(() => ({
		mutationKey: ["tabs", "new"],
		async mutationFn() {
			const tab = await invoke<Tab>("tabs_add", {
				url: Math.random() > 0.5 ? "https://yandex.ru" : "wtf",
			});
			return tab;
		},
	}));
	return {
		tabs: () => queryTabs.data ?? [],
		addTab: mutationAddTab.mutate,
	};
}
