// index.js

import init, { convert_image } from "./pkg/wasm_image_converter.js";

async function convertImage() {
  await init(); // WASM başlat

  const fileInput = document.getElementById("fileInput");
  const formatSelect = document.getElementById("formatSelect");
  const outputImg = document.getElementById("outputImg");

  const file = fileInput.files[0];
  const format = formatSelect.value;

  if (!file) {
    alert("Bir dosya seç amk");
    return;
  }

  const buffer = await file.arrayBuffer();
  const converted = convert_image(new Uint8Array(buffer), format);

  const blob = new Blob([converted], { type: `image/${format}` });
  const url = URL.createObjectURL(blob);
  outputImg.src = url;
}

// 💥 Fonksiyonu global scope’a ver
window.convertImage = convertImage;