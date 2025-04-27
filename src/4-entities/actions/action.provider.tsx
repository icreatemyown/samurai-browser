import { unbindKeyCombo } from "@rwh/keystrokes";
import { bindKeyCombo } from "@rwh/keystrokes";
import { useAutoClean } from "../../5-shared/lib/cleanup/cleanup";

const SHORTCUT_EVENTS = [
	{
		action: "tabs:add",
		title: "Add new tab",
	},
	{
		action: "tabs:close",
		title: "Close current tab",
	},
] as const;
export type ShortcutAction = (typeof SHORTCUT_EVENTS)[number]["action"];

const DEFAULT_SHORTCUT_BINDS: Record<ShortcutAction, string[]> = {
	"tabs:add": ["@ControlLeft + @KeyT"],
	"tabs:close": ["@ControlLeft + @KeyW"],
};

export function useAction() {
	const autoclean = useAutoClean();

	return {
		on(action: ShortcutAction, listener: () => void) {
			const binds = DEFAULT_SHORTCUT_BINDS[action];
			const handler = { onPressed: listener };
			bindKeyCombo(binds, handler);
			autoclean(() => unbindKeyCombo(binds, handler));
		},
	};
}
