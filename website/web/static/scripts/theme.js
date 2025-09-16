/** @type {HTMLSelectElement} */
const themePicker = document.getElementById("theme-picker");

/** @type {HTMLParagraphElement} */
const themeNotice = document.getElementById("theme-notice");

const cachedTheme = localStorage.getItem("theme");
if (cachedTheme) {
    document.documentElement.setAttribute("color-scheme", cachedTheme);
    themePicker.value = cachedTheme;
}

themePicker.addEventListener("change", e => {
    const theme = e.target.value;
    document.documentElement.setAttribute("color-scheme", theme);
    localStorage.setItem("theme", theme);
    themeNotice.textContent = (theme == "light") ? "Light theme is experimental" : "";
})
