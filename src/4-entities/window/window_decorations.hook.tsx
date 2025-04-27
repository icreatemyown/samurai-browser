import { getCurrentWindow } from "@tauri-apps/api/window";
import { onCleanup } from "solid-js";
import { useQuery, useQueryClient } from "@tanstack/solid-query";

export function useWindowDecorations() {
	const appWindow = getCurrentWindow();
	const queryClient = useQueryClient();
	const isMaximized = useQuery(() => ({
		queryKey: ["decorations", "isMaximized"],
		queryFn: appWindow.isMaximized,
	}));
	const isMinimized = useQuery(() => ({
		queryKey: ["decorations", "isMinimized"],
		queryFn: appWindow.isMaximized,
	}));

	const cleanupFns: (() => void)[] = [];
	appWindow
		.onResized(() =>
			queryClient.invalidateQueries({ queryKey: ["decorations"] }),
		)
		.then((unlisten) => cleanupFns.push(unlisten));

	onCleanup(() => cleanupFns.forEach((fn) => fn()));

	return {
		isMaximized: () => isMaximized.data,
		isMinimized: () => isMinimized.data,
		toggleMaximize: () => appWindow.toggleMaximize(),
		minimize: () => appWindow.minimize(),
		close: () => appWindow.close(),
	};
}
