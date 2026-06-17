// @ts-check
import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt({
	rules: {
		"vue/max-attributes-per-line": "off",
		"vue/singleline-html-element-content-newline": "off",
		"vue/html-self-closing": "off",
		"import/consistent-type-specifier-style": ["error", "prefer-top-level"],
		"import/newline-after-import": "error",
		"import/order": [
			"error",
			{
				groups: ["builtin", "external", "internal", ["parent", "sibling", "index"], "type"],
				"newlines-between": "always",
				alphabetize: { order: "asc", caseInsensitive: true },
			},
		],
	},
}).append({
	name: "beacon/ui-generated",
	files: ["app/components/ui/**"],
	rules: {
		"vue/require-default-prop": "off",
	},
});
