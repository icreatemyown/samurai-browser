import { onCleanup } from "solid-js";

export function useAutoClean() {
	const fns: Array<() => unknown> = [];
	let alreadyCleaned = false;
	onCleanup(() => {
		fns.forEach((fn) => fn());
		alreadyCleaned = true;
	});

	return (fn: () => unknown) => {
		if (alreadyCleaned) {
			fn();
		} else {
			fns.push(fn);
		}
	};
}
