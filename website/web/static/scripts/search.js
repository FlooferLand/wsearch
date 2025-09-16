/** @type {HTMLOListElement} */
const searchResults = document.getElementById("search-results");

/** @type {HTMLSpanElement[]} */
const searchTerms = document.querySelectorAll("span.search-term");

/** @param {{ [string]: string }} */
function updateResults(json) {
    /** @type {string} */
    const param = new URLSearchParams(window.location.search).get("q");
    for (let term of searchTerms) {
        term.textContent = param;
    }

    // Updating
    let resultsHtml = "";
    let found = []
    for (let key of Object.keys(json)) {
        if (!key.includes(param)) continue;
        let elem = json[key];
        if (!found.includes(elem[0])) {
            resultsHtml += `<li><a href="/art/${elem[0]}">${elem[1]}</a></li>`;
            found += elem[0];
        }
    }
    if (found.length == 0) {
        resultsHtml += `<li><p>No results found.</p></li>`;
    }
    searchResults.innerHTML = resultsHtml;
}

fetch("/static/data/search.json", { method: "GET" }).then(response => {
    response.json().then(json => updateResults(json));
});
