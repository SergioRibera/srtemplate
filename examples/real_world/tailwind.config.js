/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "jit",
    content: {
        files: ["src/**/*.rs", "index.html"],
    },
    darkMode: "media", // 'media' or 'class'
    theme: {
        fontFamily: {
            "alfa-slab": ["Alfa Slab One", "sans-serif"],
            "fira-sans": ["Fira Sans", "sans-serif"],
            "work-sans": ["Work Sans", "sans-serif"],
        },
        extend: {},
    },
    variants: {
        extend: {},
    },
    plugins: [],
};
