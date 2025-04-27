import type { ParentProps } from "solid-js";
import { WindowDecorations } from "./window_decorations";

export function WindowLayout(props: ParentProps) {
	return (
		<>
			<WindowDecorations />
			{props.children}
		</>
	);
}
