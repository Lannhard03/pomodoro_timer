#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::sync::{Arc, Mutex};
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> eframe::Result<()> {
    use std::thread;

    use egui::{Vec2, Pos2};
    use pomodoro_timer::PomoTimer;

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let native_options = eframe::NativeOptions{
        resizable: false,
        transparent: true,
        initial_window_size: Option::from(Vec2::new(350 as f32, 300 as f32)),
        initial_window_pos: Option::from(Pos2::new(10 as f32, 10 as f32)), 
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| {
            let timer_data = Arc::new(Mutex::new(pomodoro_timer::TimerData::default()));                 
            let timer_data_clone = timer_data.clone();
            
            thread::spawn(move ||{
               let mut timer = PomoTimer::new(timer_data); 
               timer.init();
            });
            Box::new(pomodoro_timer::TimerApp::new(cc, timer_data_clone))
        })
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(pomodoro_timer::TimerApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
