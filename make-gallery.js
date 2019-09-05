#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const base = path.join(__dirname, "docs/gallery/scenes");
const dest = path.join(__dirname, "docs/gallery");

const htmlTop = `
<!doctype html>
<head>
<title>VeoLuz Gallery</title>
<meta charset=utf8>
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel=stylesheet href=styles.css>
</head>
<body>
<div class="title">
  Made with VeoLuz
</div>
<div class="description">
  <p>
    Tap an image to remix in VeoLuz. All images on this page are <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>, in attribution credit "Jared Forsyth".
  </p>
</div>
<div class="images">
`;

const htmlBottom = `
</div>
<script src="delay.js"></script>
</body>
`;

const images = fs
  .readdirSync(base)
  .map(name => {
    const full = path.join(base, name);
    if (!fs.existsSync(path.join(full, "image.png"))) {
      return;
    }
    const meta = require(path.join(full, "meta.json"));
    const config = require(path.join(full, "config.json"));
    const url = fs.readFileSync(path.join(full, "url.txt"), "utf8");
    const src = `scenes/${name}/image.png`;
    return { full, meta, config, url, src };
  })
  .filter(Boolean)
  .sort((a, b) =>
    a.meta.starred === b.meta.starred
      ? b.meta.created - a.meta.created
      : b.meta.starred - a.meta.starred
  );

const htmlBody = images
  .map(({ full, meta, config, url, src }, i) => {
    const link = "../app#" + url;
    if (i >= 6) {
      return `
    <a href="${link}">
      <div class="image loading" data-src="${src}">Loading</div>
      </a>
      `;
    }
    return `
    <a href="${link}">
    <div class="image" data-meta='${JSON.stringify(meta)}'>
      <img src="${src}">
    </div>
    </a>
  `;
  })
  .join("\n");

fs.writeFileSync(
  path.join(dest, "index.html"),
  htmlTop + htmlBody + htmlBottom
);
