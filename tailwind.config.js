/** @type {import('tailwindcss').Config} */
const token = (name) => `rgb(var(--${name}) / <alpha-value>)`;

module.exports = {
    darkMode: "class",
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {
            colors: {
                bg: token("bg"),
                surface: token("surface"),
                raised: token("raised"),
                line: token("line"),
                "line-strong": token("line-strong"),
                fg: token("fg"),
                "fg-muted": token("fg-muted"),
                "fg-subtle": token("fg-subtle"),
                mc: token("mc"),
                gold: token("gold"),
                "on-gold": token("on-gold"),
                danger: token("danger"),
            },
            fontFamily: {
                sans: ['"Schibsted Grotesk"', "ui-sans-serif", "system-ui", "sans-serif"],
                pixel: ['"Pixelify Sans"', "ui-monospace", "monospace"],
                mono: ['"JetBrains Mono"', "ui-monospace", "monospace"],
            },
            maxWidth: {
                page: "1240px",
            },
        },
    },
    plugins: [],
}
