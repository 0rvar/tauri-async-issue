# Tauri async issue

This repo showcases a minimal reproduction of an issue with Tauri.

Run the app with `yarn tauri dev`, and press ctrl+c.
The app will not exit.

However, if the `spawn_async()` line is commented out, the app will exit on `ctrl+c`.
