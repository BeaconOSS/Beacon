import {
	BarChart3,
	Bold,
	CircleAlert,
	CircleCheck,
	Clock,
	Code,
	EyeOff,
	FileText,
	Globe,
	Heading,
	Image as ImageIcon,
	Images,
	Info,
	Italic,
	Link2,
	Link as LinkIcon,
	List,
	ListOrdered,
	Lock,
	Package,
	Quote,
	Scale,
	Settings,
	Strikethrough,
	Tags,
	TriangleAlert,
	Users,
	Video,
} from "@lucide/vue";

import { VERSION_CHANNELS } from "../versions";

import type { ChecklistLevel, LevelStyle, LicenseOption, MarkdownAction, NavItem, SharePreset, StatusBanner, VisibilityOption } from "./types";
import type { ProjectStatus } from "~/scripts/constants";

import { PROJECT_STATUS, PROJECT_VISIBILITY, VERSION_CHANNEL } from "~/scripts/constants";

export const NAV_ITEMS: NavItem[] = [
	{ id: "general", label: "General", icon: Settings },
	{ id: "tags", label: "Tags", icon: Tags },
	{ id: "description", label: "Description", icon: FileText },
	{ id: "versions", label: "Versions", icon: Package },
	{ id: "license", label: "License", icon: Scale },
	{ id: "gallery", label: "Gallery", icon: Images },
	{ id: "links", label: "Links", icon: Link2 },
	{ id: "members", label: "Members", icon: Users },
	{ id: "analytics", label: "Analytics", icon: BarChart3 },
];

export const LEVEL_STYLES: Record<ChecklistLevel, LevelStyle> = {
	required: {
		icon: CircleAlert,
		label: "Required",
		tone: "text-destructive",
		pill: "bg-destructive/10 text-destructive",
		accent: "border-l-destructive",
	},
	warning: {
		icon: TriangleAlert,
		label: "Warning",
		tone: "text-amber-500",
		pill: "bg-amber-500/10 text-amber-500",
		accent: "border-l-amber-500",
	},
	suggestion: {
		icon: Info,
		label: "Suggestion",
		tone: "text-muted-foreground",
		pill: "bg-muted text-muted-foreground",
		accent: "border-l-border",
	},
};

export const STATUS_BANNERS: Record<ProjectStatus, StatusBanner> = {
	[PROJECT_STATUS.DRAFT]: {
		label: "Draft",
		description: "Only you can see this project. Complete the checklist and submit it for review when you're ready to go live.",
		icon: FileText,
		pill: "bg-muted text-muted-foreground",
		card: "border-border bg-muted/30",
		iconTone: "text-muted-foreground",
		showNotes: false,
	},
	[PROJECT_STATUS.IN_REVIEW]: {
		label: "In review",
		description: "A moderator is reviewing your project. Editing is locked while it's in review - withdraw the submission if you need to make changes before they respond.",
		icon: Clock,
		pill: "bg-amber-500/15 text-amber-500",
		card: "border-amber-500/30 bg-amber-500/5",
		iconTone: "text-amber-500",
		showNotes: false,
	},
	[PROJECT_STATUS.CHANGES_REQUESTED]: {
		label: "Changes requested",
		description: "A moderator asked for changes before this project can go live. Address their notes, then resubmit.",
		icon: TriangleAlert,
		pill: "bg-amber-500/15 text-amber-500",
		card: "border-amber-500/30 bg-amber-500/5",
		iconTone: "text-amber-500",
		showNotes: true,
	},
	[PROJECT_STATUS.APPROVED]: {
		label: "Approved & live",
		description: "Your project is live on Beacon. Editing key details or uploading a new version will send it back for review.",
		icon: CircleCheck,
		pill: "bg-primary/15 text-primary",
		card: "border-primary/30 bg-primary/5",
		iconTone: "text-primary",
		showNotes: false,
	},
	[PROJECT_STATUS.REJECTED]: {
		label: "Rejected",
		description: "A moderator rejected this project. Review their notes, make the necessary changes, and resubmit.",
		icon: CircleAlert,
		pill: "bg-destructive/15 text-destructive",
		card: "border-destructive/30 bg-destructive/5",
		iconTone: "text-destructive",
		showNotes: true,
	},
};

export const NAV_STATUS_DOT: Record<ProjectStatus, string> = {
	[PROJECT_STATUS.DRAFT]: "bg-muted-foreground/40",
	[PROJECT_STATUS.IN_REVIEW]: "bg-amber-500",
	[PROJECT_STATUS.CHANGES_REQUESTED]: "bg-amber-500",
	[PROJECT_STATUS.APPROVED]: "bg-primary",
	[PROJECT_STATUS.REJECTED]: "bg-destructive",
};

export const VISIBILITY_OPTIONS: VisibilityOption[] = [
	{
		value: PROJECT_VISIBILITY.PUBLIC,
		label: "Public",
		description: "Anyone can find and view it.",
		icon: Globe,
	},
	{
		value: PROJECT_VISIBILITY.UNLISTED,
		label: "Unlisted",
		description: "Only people with the link can view it.",
		icon: Link2,
	},
	{
		value: PROJECT_VISIBILITY.PRIVATE,
		label: "Private",
		description: "Only members can view it.",
		icon: Lock,
	},
];

export const SHARE_PRESETS: SharePreset[] = [
	{ value: 80, label: "Max (80%)" },
	{ value: 60, label: "60%" },
	{ value: 40, label: "40%" },
	{ value: 20, label: "20%" },
	{ value: 0, label: "Donate all (0%)" },
];

export const LICENSE_OPTIONS: LicenseOption[] = [
	{ value: "All Rights Reserved", label: "All Rights Reserved" },
	{ value: "MIT", label: "MIT" },
	{ value: "Apache-2.0", label: "Apache License 2.0" },
	{ value: "GPL-3.0", label: "GNU GPL v3.0" },
	{ value: "LGPL-3.0", label: "GNU LGPL v3.0" },
	{ value: "MPL-2.0", label: "Mozilla Public License 2.0" },
	{ value: "BSD-3-Clause", label: "BSD 3-Clause" },
	{ value: "CC0-1.0", label: "Creative Commons Zero (Public Domain)" },
	{ value: "CC-BY-4.0", label: "Creative Commons Attribution 4.0" },
	{
		value: "CC-BY-SA-4.0",
		label: "Creative Commons Attribution-ShareAlike 4.0",
	},
];

export const CHANNEL_STYLES: Record<string, string> = {
	[VERSION_CHANNEL.RELEASE]: "bg-primary/15 text-primary",
	[VERSION_CHANNEL.BETA]: "bg-amber-500/15 text-amber-500",
	[VERSION_CHANNEL.ALPHA]: "bg-violet-500/15 text-violet-400",
};

export function channelLabel(value: string): string {
	return VERSION_CHANNELS.find((c) => c.value === value)?.label ?? value.charAt(0).toUpperCase() + value.slice(1);
}

export const MARKDOWN_ACTIONS: MarkdownAction[] = [
	{
		icon: Heading,
		label: "Heading",
		before: "## ",
		placeholder: "Heading",
		block: true,
	},
	{
		icon: Bold,
		label: "Bold",
		before: "**",
		after: "**",
		placeholder: "bold text",
	},
	{
		icon: Italic,
		label: "Italic",
		before: "_",
		after: "_",
		placeholder: "italic text",
	},
	{
		icon: Strikethrough,
		label: "Strikethrough",
		before: "~~",
		after: "~~",
		placeholder: "struck text",
	},
	{ icon: Code, label: "Code", before: "`", after: "`", placeholder: "code" },
	{
		icon: LinkIcon,
		label: "Link",
		before: "[",
		after: "](https://)",
		placeholder: "link text",
	},
	{
		icon: ImageIcon,
		label: "Image",
		before: "![",
		after: "](https://)",
		placeholder: "alt text",
	},
	{
		icon: Video,
		label: "Video / embed",
		before: "[",
		after: "](https://)",
		placeholder: "video link",
	},
	{
		icon: List,
		label: "Bullet list",
		before: "- ",
		placeholder: "List item",
		block: true,
	},
	{
		icon: ListOrdered,
		label: "Numbered list",
		before: "1. ",
		placeholder: "List item",
		block: true,
	},
	{
		icon: Quote,
		label: "Quote",
		before: "> ",
		placeholder: "Quote",
		block: true,
	},
	{
		icon: EyeOff,
		label: "Spoiler",
		before: "<details><summary>Spoiler</summary>\n\n",
		after: "\n\n</details>",
		placeholder: "hidden content",
	},
];

export const NOTE_MARKDOWN_ACTIONS: MarkdownAction[] = MARKDOWN_ACTIONS.filter((a) => ["Bold", "Italic", "Code", "Link", "Bullet list"].includes(a.label));
