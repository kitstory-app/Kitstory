use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    tauri::Builder::default()
        // Plugins
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        //
        .setup(|app| {
            let menu = MenuBuilder::new(app)
                /* This is where the quick access stuff in this area usually resides, but will be dynamically added via JS */
                .item(
                    &MenuItemBuilder::new("About Kitstory")
                        .id("spawn_about")
                        .build(app)?,
                )
                .item(
                    &MenuItemBuilder::new("Open/hide window")
                        .id("respawn")
                        .build(app)?,
                )
                .separator()
                .item(
                    &MenuItemBuilder::new("Login to Kitstory Cloud")
                        .id("ks_cloud_auth")
                        .build(app)?,
                )
                .separator()
                .item(
                    &MenuItemBuilder::new("Exit Kitstory")
                        .id("exit_app")
                        .build(app)?,
                )
                .build()?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "exit_app" => {
                        // TODO: Might need to do some checks such as unsaved file changes or some shit
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
