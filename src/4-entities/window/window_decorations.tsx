import { useWindowDecorations } from "./window_decorations.hook";
import style from "./windowtitle.module.css";
import IconMinimize from "@fluentui/svg-icons/icons/subtract_16_regular.svg";
import IconMaximize from "@fluentui/svg-icons/icons/maximize_16_regular.svg";
import IconUnmaximize from "@fluentui/svg-icons/icons/square_multiple_16_regular.svg";
import IconClose from "@fluentui/svg-icons/icons/dismiss_16_regular.svg";
import { Show } from "solid-js";

export function WindowDecorations() {
	const decorations = useWindowDecorations();

	return (
		<div data-tauri-drag-region={true} class={style.titlebar}>
			<div
				class={style["titlebar-button"]}
				id="titlebar-minimize"
				on:click={decorations.minimize}
			>
				<IconMinimize />
			</div>
			<div
				class={style["titlebar-button"]}
				id="titlebar-maximize"
				on:click={decorations.toggleMaximize}
			>
				<Show when={decorations.isMaximized()} fallback={<IconMaximize />}>
					<IconUnmaximize />
				</Show>
			</div>
			<div
				class={style["titlebar-button"]}
				id="titlebar-close"
				on:click={decorations.close}
			>
				<IconClose />
			</div>
		</div>
	);
}
