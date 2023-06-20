use std::time::{self, Duration};
use std::time::Instant;
use std::sync::{Mutex,Arc, MutexGuard};
use std::thread;
use eframe::{egui::{TextBuffer, Visuals, Color32, Frame, Rect, Pos2, Vec2, Sense, RichText, TextFormat, FontFamily, FontId, Label}};
use egui::{text_edit, Button, Ui, TextStyle};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TimerApp {
    #[serde(skip)]
    //can you make this field read-only? only a refrence perhaps?
    timer_data: Option<Arc<Mutex<TimerData>>>,
}

pub struct PomoTimer{
    time_data: Arc<Mutex<TimerData>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TimerData{
    current_left: Duration,
    #[serde(skip)]
    timer_state: TimerState,
    total_time: Duration,
}

#[derive(PartialEq, Eq)]
pub enum TimerState{
    Started (Instant),
    Paused (Duration),
    Done,
}

impl Default for PomoTimer{
    fn default() -> Self {
        Self {
            time_data: Arc::new(Mutex::new(TimerData::default())),
        }
    }
}
impl Default for TimerData{
    fn default() -> Self{
        Self {
            current_left: Duration::from_secs(0),
            timer_state: TimerState::Done, 
            total_time: Duration::from_secs(25*60),
        }
    }
    
}

impl TimerData{
    fn as_minutes(dur: &Duration) -> String {
        let secs = dur.as_secs();
        let minutes = if secs/60 >= 10 {
            format!(" {}", secs/60)
        } else if secs/60 > 0 {
            format!("{}", secs/60)
        } else {
            String::from("00")
        };

        let secs = if  secs%60 >= 10 {
            format!("{}", secs%60)
        } else if secs%60 > 0{
            format!("0{}", secs%60)
        } else {
            String::from("00")
        };

       format!("{}:{}", minutes, secs)
    }
}

            
impl PomoTimer{
    pub fn new(time_data: Arc<Mutex<TimerData>>)-> Self {
        PomoTimer {time_data}
    }
    pub fn init(&mut self) {
       self.run(); 
    }
    fn run(&mut self) {
        loop {
            //is this cringe?
            thread::sleep(Duration::from_millis(250));
            self.update(); 
        }
    }
    fn update(&mut self)  {
       let mut data = self.time_data.lock().unwrap();
       match data.timer_state {
            TimerState::Started(start_time) =>{
                data.current_left = data.total_time - start_time.elapsed();
                if data.current_left.as_secs() <= 0 {
                    data.timer_state = TimerState::Done;
                }
            }
            _ => ()
        }
    }
 
}


impl Default for TimerApp {
    fn default() -> Self {
        Self {
            timer_data: None,
        }
    }
}

impl TimerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, timer_data: Arc<Mutex<TimerData>>) -> Self {
        
        cc.egui_ctx.set_pixels_per_point(2.5);
        let mut style = (*cc.egui_ctx.style()).clone();
        use FontFamily::{Proportional, Monospace};
        style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(12.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
        (TextStyle::Name("Timer".into()), FontId::new(24.0, Proportional)),
        ].into();
        cc.egui_ctx.set_style(style);
        

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut app: TimerApp = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            app.timer_data = Some(timer_data);
            //should this be clone?
            return app 
        }
        
        let mut app = TimerApp::default();
        app.timer_data = Some(timer_data);
        return app
    }

    pub fn draw_from_state<'a>(ui: &'a mut Ui, open_data: &mut std::sync::MutexGuard<'_, TimerData> ) -> &'a mut Ui {
        ui.horizontal(|ui| {

            let display_string = &mut format!("{}", TimerData::as_minutes(&open_data.current_left));

            let response = ui.add_sized([100.0, 30.0], egui::TextEdit::singleline(display_string)
                                        .font(TextStyle::Name("Timer".into()))
                                        .interactive(open_data.timer_state == TimerState::Done));
            if response.changed() {
                if let Ok(time_f32) = display_string.parse::<f32>() {
                    if let Ok(duration) = Duration::try_from_secs_f32(time_f32) {
                        open_data.total_time = duration; //doesn't work since we redraw every
                                                         //frame, maybe don't redraw when
                                                         //TimerState == Done?
                    }
                }
            }
            ui.add_sized([10.0, 30.0], Button::new(">"));

        });
            if ui.add(egui::Button::new("Start timer")).clicked() {
                match open_data.timer_state {
                    TimerState::Done => {open_data.timer_state = TimerState::Started(Instant::now())}
                    TimerState::Started(started_time) => {open_data.timer_state = TimerState::Paused(started_time.elapsed())} 
                    TimerState::Paused(paused_time) => {open_data.timer_state = TimerState::Started(Instant::now()-paused_time)}
                }
            }
            ui.horizontal(|ui|{ ui.button("Work"); ui.end_row(); ui.button("Short"); ui.end_row(); ui.button("Long"); });

        return ui
    }
}


impl eframe::App for TimerApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn persist_native_window(&self) -> bool {
        false
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut open_data = self.timer_data.as_ref().unwrap().lock().unwrap();
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let ui = TimerApp::draw_from_state(ui, &mut open_data);
            });
        ctx.request_repaint_after(Duration::from_secs(1));
    }


}
