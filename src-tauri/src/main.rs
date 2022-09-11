#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::*;

mod commands;

#[allow(unused_must_use)]
fn main() {

    tauri::Builder::default()
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();
      main_window.with_webview(|webview| {
        #[cfg(target_os = "linux")]
        {
          use webkit2gtk::traits::WebViewExt;
          webview.inner().set_zoom_level(4.);
        };

        #[cfg(windows)]
        unsafe {
          webview.controller().SetZoomFactor(4.).unwrap();
        };

        #[cfg(target_os = "macos")]
        unsafe {
          let () = msg_send![webview.inner(), setPageZoom: 4.];
          let () = msg_send![webview.controller(), removeAllUserScripts];
          let bg_color: cocoa::base::id = msg_send![class!(NSColor), colorWithDeviceRed:0.5 green:0.2 blue:0.4 alpha:1.];
          let () = msg_send![webview.ns_window(), setBackgroundColor: bg_color];
        };
      });
      main_window.set_fullscreen(false);
      main_window.set_resizable(false);
      main_window.set_size(Size::Logical(LogicalSize {
                    width: 1000.0,
                    height: 600.0,
      }));
      let message = main_window.is_fullscreen().unwrap();
      println!("Message: {}", message);
      Ok(())
  })
  .invoke_handler(tauri::generate_handler![
    commands::authorization::authorize,
    ])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}