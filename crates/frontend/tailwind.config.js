/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "*.html",
        "./src/app.rs",
        "./src/pages/**/*.rs",
        "./src/components/**/*.rs",
        "./src/hoc/**/*.rs",
    ],
    theme: {
        extend: {
            keyframes: {
                slideInFromBottom: {
                    from: {
                        transform: "translateY(100%)",
                        opacity: 0,
                    },
                    to: {
                        transform: "translateY(0)",
                        opacity: 1,
                    },
                },
                slideOutFromBottom: {
                    from: {
                        transform: "translateY(0)",
                        opacity: 1,
                    },
                    to: {
                        transform: "translateX(100%)",
                        opacity: 0,
                    },
                },
            },
            animation: {
                "slide-in-bottom": "slideInFromBottom 0.2s ease-out",
                "slide-out-bottom": "slideOutFromBottom 0.2s ease-in",
            },
        },
    },
    plugins: [require("daisyui")],
    daisyui: {
        themes: ["light", "dark", "cupcake", "dim"],
    },
};
