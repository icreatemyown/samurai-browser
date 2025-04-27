import { BrowserTabs } from "../4-entities/browser_tabs/browser_tabs.tsx";

type Props = {
	url: string;
};

export function BrowserHeader(_props: Props) {
	return (
		<div>
			<BrowserTabs />
		</div>
	);
}
