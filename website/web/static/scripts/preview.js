/** @type {HTMLImageElement} */
const image = document.getElementById("wplace-preview");

/** @type {HTMLCanvasElement} */
const canvas = document.getElementById("wplace-preview-canvas");
const ctx = canvas.getContext("2d");
canvas.width = 300;
canvas.height = 300;

function run(metadata) {
    const tileCoords = { x: metadata.image.tile[0], y: metadata.image.tile[1] };
    const relCoords = { x: metadata.image.coords[0], y: metadata.image.coords[1] };
    const tileUrl = `https://backend.wplace.live/files/s0/tiles/${tileCoords.x}/${tileCoords.y}.png`;
    const proxied = (url) => `https://corsproxy.io/?url=${url}`;

    // Getting the submitted image size
    const localImage = new Image();
    const targetSize = { width: 0, height: 0 };
    localImage.onload = function() {
        targetSize.width = localImage.naturalWidth;
        targetSize.height = localImage.naturalHeight;

        // Showing the wplace image
        const wplaceImage = new Image();
        wplaceImage.crossOrigin = "anonymous";
        wplaceImage.onload = function () {
            canvas.width = targetSize.width;
            canvas.height = targetSize.height;
            ctx.fill()
            ctx.drawImage(wplaceImage, relCoords.x, relCoords.y, targetSize.width, targetSize.height, 0, 0, targetSize.width, targetSize.height);
            image.src = canvas.toDataURL();
        };
        wplaceImage.src = proxied(tileUrl);
    };
    localImage.src = `${artUrl}/${metadata.image.png}`;
}
const metadata = fetch(`${artUrl}/metadata.json`, { method: "GET" }).then(e => e.json().then(run));
