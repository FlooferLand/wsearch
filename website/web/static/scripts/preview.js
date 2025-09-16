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
        };
        wplaceImage.src = proxied(tileUrl);
    };
    localImage.src = `${artUrl}/${metadata.image.png}`;
}
const metadata = fetch(`${artUrl}/metadata.json`, { method: "GET" }).then(e => e.json().then(run));
