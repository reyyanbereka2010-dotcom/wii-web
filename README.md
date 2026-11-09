# Wii Web

A local browser frontend for an experimental WebAssembly port of Dolphin, the GameCube and Wii emulator. The Rust application serves the emulator page and provides the cross-origin isolation headers needed by the browser core.

**Status: experimental.** This project is not affiliated with or endorsed by Nintendo or the Dolphin Emulator Project. It does not include any game images. Use game files you are authorized to use.

## Requirements

- Rust stable (install with [rustup](https://rustup.rs/))
- Desktop Chrome or another Chromium browser with WebGPU and SharedArrayBuffer support
- A compatible game image, such as `.iso`, `.wbfs`, `.rvz`, or `.gcm`

## Quick start

Clone this repository with its Dolphin core submodule, then start the Rust server:

```sh
git clone --recurse-submodules <repository-url> wii-web
cd wii-web
cargo run
```

Open <http://127.0.0.1:3000> in a compatible browser and load your game image through the emulator interface. The game image is opened by the browser and is not uploaded to a server.

If the project was cloned without submodules, initialize the core with:

```sh
git submodule update --init --recursive
```

The Rust server is intentionally bound to `127.0.0.1`. It is a local development server, not a production internet-facing service.

## Core and compatibility

The core is [wasm-dolphin](https://github.com/dougchansan/wasm-dolphin), pinned as a Git submodule at commit `7e38409ace3dda709c178312ff63fd92a3653cc7`. It uses Dolphin compiled to WebAssembly, including an experimental PowerPC-to-WebAssembly JIT. The prebuilt core is included in the submodule; running the Rust frontend does not require Node.js or an Emscripten build.

Wii support is partial. The core project reports no Wii Remote input yet, and performance and rendering vary by game and GPU. Browser support depends on WebGPU and cross-origin isolation. Consult the [core's status and setup notes](https://github.com/dougchansan/wasm-dolphin#requirements) before troubleshooting a game that does not boot or render correctly.

## License

This combined project is distributed under GPL-2.0-or-later. See [LICENSE](LICENSE) and the core submodule's license and attribution notices. The Dolphin-derived WebAssembly core is GPL-licensed; redistributing a modified or combined build carries corresponding source and license obligations. No Nintendo game images, firmware, or other proprietary game data are included.
