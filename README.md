# GhostDeck

A small desktop client for the ghosts you own in the GaiaWM world.

It is deliberately a **thin shell**: the window loads the gateway's
owner-mode dashboard (`https://svc.ingmmo.com/my/`) — same-origin, so the
page talks to its own API. You enter your **owner key** once; it persists in
the app's local storage and is sent only to the gateway. UI improvements ship
server-side; the app never needs an update for them.

What you get: your roster (and only yours — the key scopes everything
server-side), vitals, soul & goal imprinting, memories, a live map of the
whole world, their event feed, and a chat box — your ghosts remember what
you tell them.

## Local minds (your Ollama)

The mind card's **🖥 this device** mode runs a ghost's think-loop *inside
the app* against a local OpenAI-compatible endpoint (Ollama's
`http://localhost:11434/v1` by default): organs go through the gateway,
LLM calls go straight to your machine, and no model, key, or prompt ever
touches the server. The shell grants the owner page exactly one power for
this — `tauri-plugin-http` scoped to `localhost`/`127.0.0.1` only
(`src-tauri/capabilities/remote-mind.json`) — so the page cannot use the
app to reach anything else. The mind ticks while the deck is open; use the
☁ gateway mode (custodial engine) if you want it thinking while you're
away. In a plain browser the same UI works but needs
`OLLAMA_ORIGINS=https://svc.ingmmo.com` and a Chromium-based browser.

## Build (Docker — no host toolchain needed)

```bash
cd ghostdeck
docker build -f Dockerfile.build --target bundles -o out .              # Linux
docker build -f Dockerfile.build --target win-bundles -o out-windows .  # Windows
docker build -f Dockerfile.build --target mac-bundles -o out-macos .    # macOS
```

`out/` then holds `bundle/deb/*.deb`, `bundle/rpm/*.rpm`,
`bundle/appimage/*.AppImage` and the bare `bin/ghostdeck` binary
(x86_64 Linux). `out-windows/` holds the NSIS installer
(`nsis/GhostDeck_*_x64-setup.exe` — fetches the WebView2 runtime at
install time if missing) and the bare `bin/ghostdeck.exe`; the Windows
build cross-compiles with cargo-xwin against the MS CRT/SDK and is
**unsigned** (SmartScreen will warn — set `bundle > windows >
sign_command` when a cert exists). Cargo registry and target dirs are
BuildKit cache mounts, so rebuilds are incremental.

`out-macos/` holds `GhostDeck_*_universal.app.tar.gz` — a universal
(x86_64 + Apple Silicon) `.app`, cross-compiled with osxcross against
the MacOSX 13.3 SDK and **ad-hoc signed** with rcodesign (arm64 macOS
refuses unsigned code outright). tauri-cli won't emit macOS bundle
types from a Linux host, so the `.app` is assembled manually from
`src-tauri/macos/Info.plist` (keep its version in sync with
`Cargo.toml`). No DMG (`hdiutil` is macOS-only) and no notarization:
downloaded copies are quarantined by Gatekeeper — first launch needs
right-click → Open (or a Developer-ID sign + notarize pass on a real
Mac when distribution warrants it).

Pinned to the Tauri **2.11** line (the latest — Tauri v3 does not exist
yet; bump `Cargo.toml` + `TAURI_CLI_VERSION` in `Dockerfile.build`
together when it ships).

## Build (bare metal)

Prereqs: Rust ≥ 1.77 and the platform webview deps
(<https://tauri.app/start/prerequisites/>). Then:

```bash
cd ghostdeck/src-tauri
cargo install tauri-cli --version '^2'   # once
cargo tauri build                        # installers in target/release/bundle/
# or, just the binary:
cargo build --release && ./target/release/ghostdeck
```

Linux needs `libwebkit2gtk-4.1-dev libgtk-3-dev librsvg2-dev`; macOS and
Windows build with their stock toolchains.

## Pointing at a world

The app opens a small **gateway chooser** (`dist/index.html`): the shared
cloud world (`svc.ingmmo.com`), a localhost gateway (default
`http://localhost:8090/my/` — the toril-sim dashboard port), or any custom
URL. The choice is remembered and auto-reconnects on the next launch
(with a 4-second "stay here" escape hatch). Bare hosts are normalised to
`…/my/`, so `localhost:8090` works as-is.
