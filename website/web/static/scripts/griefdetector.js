/** @type {HTMLCanvasElement} */
const wplaceCanvas = document.getElementById("wplace-canvas");

/** @type {HTMLCanvasElement} */
const targetCanvas = document.getElementById("target-canvas");

const wplaceCtx = wplaceCanvas.getContext("2d");
const targetCtx = targetCanvas.getContext("2d");

function run(metadata) {
    console.log(metadata);
    const tileCoords = { x: metadata.image.tile[0], y: metadata.image.tile[1] };
    const relCoords = { x: metadata.image.coords[0], y: metadata.image.coords[1] };
    const pixelUrl = `https://backend.wplace.live/s0/pixel/${tileCoords.x}/${tileCoords.y}?x=${relCoords.x}&y=${relCoords.y}`;
    const tileUrl = `https://backend.wplace.live/files/s0/tiles/${tileCoords.x}/${tileCoords.y}.png`;
    const proxied = (url) => `https://corsproxy.io/?url=${url}`;

    // Getting the submitted image size
    const targetImage = new Image();
    const targetSize = { width: 0, height: 0 };
    targetImage.onload = function () {
        targetSize.width = targetImage.naturalWidth;
        targetSize.height = targetImage.naturalHeight;
        targetCanvas.width = targetImage.naturalWidth;
        targetCanvas.height = targetImage.naturalHeight;
        targetCtx.drawImage(targetImage, 0, 0);
        targetCtx.getImageData()

        // Showing the wplace image        
        const wplaceImage = new Image();
        wplaceImage.loading = "eager";
        wplaceImage.onload = function () {
            wplaceCanvas.width = targetSize.width;
            wplaceCanvas.height = targetSize.height;
            wplaceCtx.drawImage(wplaceImage, relCoords.x, relCoords.y, targetSize.width, targetSize.height, 0, 0, targetSize.width, targetSize.height);
            wplaceCtx.getImageData()
        };
        console.log(tileUrl);
        wplaceImage.src = proxied(tileUrl);
    };
    targetImage.src = `${artUrl}/${metadata.image.png}`;
}
const metadata = fetch(`${artUrl}/metadata.json`, { method: "GET" }).then(e => e.json().then(run));
