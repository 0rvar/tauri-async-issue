#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(async_closure)]

fn main() {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("failed to build Tauri application");

    spawn_async();

    app.run(|_app_handle, event| match event {
        _ => {}
    });
}

fn spawn_async() {
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new().route(
            "/{token}",
            actix_web::web::get().to(async || "Hello, world"),
        )
    })
    .bind("127.0.0.1:5005")
    .expect("Failed to run http server")
    .run();
    tauri::async_runtime::spawn(server);
}
