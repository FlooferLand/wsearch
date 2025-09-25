/** @type {HTMLDivElement} */
const limitationNotice = document.getElementById("wsearch-tile-limit-notice");

/** @type {HTMLImageElement} */
const liveImage = document.getElementById("wplace-live-preview");

/** @type {HTMLImageElement} */
const clippedImage = document.getElementById("wplace-clipped-preview");

/** @type {CanvasRenderingContext2D} */
const liveCtx = document.getElementById("wplace-live-preview-canvas").getContext("2d");

/** @type {CanvasRenderingContext2D} */
const clippedCtx = document.getElementById("wplace-clipped-preview-canvas").getContext("2d");

liveCtx.canvas.width = 300;
liveCtx.canvas.height = 300;
clippedCtx.canvas.width = 300;
clippedCtx.canvas.height = 300;

const proxied = (url) => `https://corsproxy.io/?url=${url}`;

const targetSize = { width: 0, height: 0 };
let tileCoords = { x: 0, y: 0 };
let relCoords = { x: 0, y: 0 };
let tileUrl = "";
const localImage = new Image();

function fetchRemoteImage() {
    // Showing the wplace image
    const wplaceImage = new Image();
    wplaceImage.crossOrigin = "anonymous";
    wplaceImage.onload = function () {
        liveCtx.canvas.width = targetSize.width;
        liveCtx.canvas.height = targetSize.height;
        clippedCtx.canvas.width = targetSize.width;
        clippedCtx.canvas.height = targetSize.height;

        // Live
        liveCtx.drawImage(wplaceImage, relCoords.x, relCoords.y, targetSize.width, targetSize.height, 0, 0, targetSize.width, targetSize.height);
        liveImage.src = liveCtx.canvas.toDataURL();

        // Clipped
        clippedCtx.drawImage(localImage, 0, 0);
        clippedCtx.globalCompositeOperation = "source-in";
        clippedCtx.drawImage(wplaceImage, relCoords.x, relCoords.y, targetSize.width, targetSize.height, 0, 0, targetSize.width, targetSize.height);
        clippedCtx.globalCompositeOperation = "source-over";
        clippedImage.src = clippedCtx.canvas.toDataURL();

        // Limitation check
        /*limitationNotice.style.display = (liveImage.width != localImage.width || liveImage.height != localImage.height)
            ? "block"
            : "none";*/
    };
    wplaceImage.src = proxied(tileUrl);
}

function run(metadata) {
    // Getting the submitted image size
    localImage.onload = function() {
        targetSize.width = localImage.naturalWidth;
        targetSize.height = localImage.naturalHeight;

        tileCoords = { x: metadata.image.tile[0], y: metadata.image.tile[1] };
        relCoords = { x: metadata.image.coords[0], y: metadata.image.coords[1] };
        tileUrl = `https://backend.wplace.live/files/s0/tiles/${tileCoords.x}/${tileCoords.y}.png`;
        fetchRemoteImage();
        setInterval(function() {
            if (document.hasFocus()) {
                fetchRemoteImage();
            }
        }, 5000);
    };
    localImage.src = `${artUrl}/${metadata.image.png}`;
}
const metadata = fetch(`${artUrl}/metadata.json`, { method: "GET" }).then(e => e.json().then(run));
