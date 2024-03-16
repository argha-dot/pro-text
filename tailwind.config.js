/** @type {import('tailwindcss').Config} */
    module.exports = {
      content: {
        relative: true,
        files: ["*.html", "./src/**/*.rs"],
      },
      theme: {
        extend: {
          colors: {
            "primary": "#191919",
            "secondary": "#202020"
          }
        },
      },
      plugins: [],
    }
    