import * as http from "http";
import { createCanvas } from "canvas";

function createImage(text: string): Buffer {
  const side = 200;
  const canvas = createCanvas(side, side);
  const ctx = canvas.getContext("2d");

  ctx.font = "40px Verdana";
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillText(text, side / 2, side / 2);

  return canvas.toBuffer();
}

// GET /:number -> image with number
// GET / -> idk

const server = http.createServer(function (req, res) {
  const url = req.url ?? "/";
  const match = url.match(/^\/(\d+)/);

  if (match) {
    const number = match[1];
    const buffer = createImage(number);
    res.writeHead(200, { "Content-Type": "image/png" });
    res.end(buffer);
  } else {
    res.end("bebra");
  }
});

server.listen(8080);
