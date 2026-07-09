// GhostDeck — a thin shell: the webview loads the gateway's owner-mode
// dashboard (same-origin, so the page talks to its own API with the owner
// key the user enters once; the key persists in the webview's localStorage).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running GhostDeck");
}
