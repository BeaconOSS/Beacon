import { Globe, Package, Palette, Shirt } from "@lucide/vue";

import { VERSION_CHANNELS } from "../versions";

import type { TypeStyle } from "./types";

export const TYPE_STYLES: Record<string, TypeStyle> = {
	addon: {
		icon: Package,
		gradient: "from-amber-400/30 to-amber-600/10 text-amber-300",
	},
	world: {
		icon: Globe,
		gradient: "from-emerald-400/30 to-emerald-600/10 text-emerald-300",
	},
	resource_pack: {
		icon: Palette,
		gradient: "from-violet-400/30 to-violet-600/10 text-violet-300",
	},
	skin_pack: {
		icon: Shirt,
		gradient: "from-pink-400/30 to-pink-600/10 text-pink-300",
	},
};

export function channelLabel(value: string): string {
	return VERSION_CHANNELS.find((c) => c.value === value)?.label ?? value;
}
