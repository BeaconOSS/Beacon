import { computed } from "vue";

export type ThemePref = "system" | "light" | "dark" | "oled";
export type ListLayout = "grid" | "rows";

export interface BeaconSettings {
	theme: ThemePref;
	listLayout: ListLayout;
	advancedRendering: boolean;
	externalLinksNewTab: boolean;
	filtersSidebarRight: boolean;
	contentSidebarLeft: boolean;
}

export const DEFAULT_SETTINGS: BeaconSettings = {
	theme: "dark",
	listLayout: "rows",
	advancedRendering: true,
	externalLinksNewTab: false,
	filtersSidebarRight: false,
	contentSidebarLeft: false,
};

export function useSettings() {
	const settings = useCookie<BeaconSettings>("beacon-settings", {
		default: () => ({ ...DEFAULT_SETTINGS }),
		maxAge: 60 * 60 * 24 * 365,
		sameSite: "lax",
		path: "/",
	});

	const systemDark = useState("beacon-system-dark", () => true);

	const resolvedThemeClass = computed(() => {
		switch (settings.value.theme) {
			case "light":
				return "";
			case "dark":
				return "dark";
			case "oled":
				return "oled";
			default:
				return systemDark.value ? "dark" : "";
		}
	});

	return { settings, systemDark, resolvedThemeClass };
}
