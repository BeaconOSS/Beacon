// @ts-check
import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt({
	rules: {
		"vue/max-attributes-per-line": "off",
		"vue/singleline-html-element-content-newline": "off",
	},
}).append({
	name: "beacon/ui-generated",
	files: ["app/components/ui/**"],
	rules: {
		"vue/require-default-prop": "off",
	},
});
