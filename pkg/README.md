# 🖼️ wasm-image-converter

> Convert and process images in the browser using blazing-fast Rust + WebAssembly. Built for modern frontend frameworks with full TypeScript support.

## 🚀 Features

- ✅ Convert images between formats (`.png`, `.jpg`, `.webp`, `.bmp`, `.gif`, `.tiff`)
- ✅ Resize, optimize or manipulate images (WIP)
- ✅ Written in Rust, compiled to WebAssembly (WASM)
- ✅ TypeScript bindings for smooth integration
- ✅ Runs 100% in-browser, no backend needed
- ✅ Lightweight, fast and secure

## 📦 Installation

```bash
npm install wasm-image-converter
# or
yarn add wasm-image-converter

🛠️ Usage (TypeScript)

import init, { convert_image } from "wasm-image-converter";

// First, initialize WASM module
await init();

// Then, use convert_image
const inputArrayBuffer: ArrayBuffer = await file.arrayBuffer();
const output = convert_image(inputArrayBuffer, "webp");

// You get a Uint8Array containing the converted image
const blob = new Blob([output], { type: "image/webp" });
const url = URL.createObjectURL(blob);

✍️ API

convert_image(input: ArrayBuffer, format: string): Uint8Array

Param	Type	Description
input	ArrayBuffer	Original image binary
format	string	Target format: "png", "jpg", "webp", etc.
returns	Uint8Array	Converted image data

💡 More advanced image processing (resize, crop, compress) coming soon.

🧱 Tech Stack
	•	🦀 Rust
	•	🕸️ WebAssembly
	•	🧙 wasm-bindgen
	•	📦 wasm-pack
	•	🔷 TypeScript + modern frontend compatibility

🧪 Development

Build the WASM module

wasm-pack build --target web

Link locally (for testing)

npm link ./pkg

📁 File Structure

wasm-image-converter/
├── src/
│   └── lib.rs              # Rust logic for image conversion
├── pkg/                    # WASM build output
├── tests/                  # Integration tests
├── README.md
├── package.json
└── index.ts                # TypeScript bindings and usage

⚠️ Limitations
	•	Browser memory limits apply for large images.
	•	Only basic format conversion is supported for now.

🤝 Contributing

Pull requests welcome! Let’s build the fastest in-browser image converter out there.

📜 License

MIT © 2025 Selamettin Aysik & Contributors

