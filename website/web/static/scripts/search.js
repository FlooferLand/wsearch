/** @type {HTMLOListElement} */
const searchResults = document.getElementById("search-results");

/** @type {HTMLSpanElement[]} */
const searchTerms = document.querySelectorAll("span.search-term");

/**
 * @typedef {Object} SearchJson
 * @property {string[][]} posts
 * @property {Object.<string, number>} names
 * @property {Object.<string, number[]>} tags
*/

/** @param {SearchJson} json */
function updateResults(json) {
    searchResults.innerHTML = `<p>Loading..</p>`;

    /** @type {string} */
    const query = (new URLSearchParams(window.location.search)?.get("q") ?? "").toLowerCase();
    for (let term of searchTerms) {
        term.textContent = query;
    }

    // -- Searching
    /** @type {Set.<number>} */
    const found = new Set();

    // Searching through names
    for (let name in json.names) {
        const id = json.names[name];
        if (name.includes(query)) {
            found.add(id);
        }
    }

    // Searching through tags
    if (json.tags[query] !== undefined) {
        const tagIds = json.tags[query];
        for (let id of tagIds) {
            found.add(id);
        }
    }

    console.log(found);
    // -- Constructing HTML
    let resultsHtml = "";
    for (let i of found) {
        const post = json.posts[i];
        resultsHtml += `<li><a href="/art/${post[0]}">${post[1]}</a></li>`;
    }
    if (found.size == 0) {
        resultsHtml += `<p>No results found.</p>`;
    }
    searchResults.innerHTML = resultsHtml;
}

fetch("/static/data/search.json", { method: "GET" }).then(response => {
    response.json().then(json => updateResults(json));
});
