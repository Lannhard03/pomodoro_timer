use std::time::Duration;
use std::time::Instant;
use std::sync::{Mutex,Arc};
use std::thread;
use eframe::{egui::{TextBuffer, Visuals, Color32, Frame, Rect, Pos2, Vec2, Sense, RichText, TextFormat, FontFamily, FontId, Label}};
use egui::style::{Widgets, WidgetVisuals};
use egui::{text_edit, Button, Ui, TextStyle, Widget};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TimerApp {
    #[serde(skip)]
    timer_data: Option<Arc<Mutex<TimerData>>>,
    color_scheme: AppColorScheme,
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

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppColorScheme{
   fill_color: Color32,
   timer_paused: Color32,
   timer_active: Color32,
   ligth_bg_color: Color32,
   dark_bg_color: Color32,
   ligth_bg_stroke: Color32,
   ligth_fg_stroke: Color32,
   dark_bg_stroke: Color32,
   dark_fg_stroke: Color32,
}
impl Default for AppColorScheme{
    fn default() -> Self {
        AppColorScheme {
            fill_color: Color32::from_rgb(88,31,24),
            timer_active: Color32::from_rgb(33, 44, 91),
            timer_paused: Color32::from_rgb(16,22,45),
            ligth_bg_color: Color32::from_rgb(217, 93, 57),
            dark_bg_color: Color32::from_rgb(241, 136, 5),
            ligth_fg_stroke: Color32::from_rgb(249, 224, 217),
            ligth_bg_stroke: Color32::from_rgb(240, 162, 2),
            dark_fg_stroke: Color32::from_rgb(249, 224, 217),
            dark_bg_stroke: Color32::from_rgb(253, 186, 53),
        } 
    }
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
            total_time: Duration::from_secs(5),
        }
    }
    
}

impl TimerData{
    fn as_minutes(dur: &Duration) -> String {
        let secs = dur.as_secs();
        let minutes = if secs/60 >= 10 {
            format!("{}", secs/60)
        } else if secs/60 > 0 {
            format!(" {}", secs/60)
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
            //YES, you don't even need to "run" timer in different thread
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
            color_scheme: AppColorScheme::default(),
        }
    }
}

impl TimerApp {
    /// Called once before the first frame.
    fn setup_fonts(cc: &eframe::CreationContext<'_>) {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert("Fira Code Mono".to_owned(), egui::FontData::from_static(include_bytes!("../assets/FiraCodeNerdFontMono-Regular.ttf")));
        fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, "Fira Code Mono".to_owned());
        cc.egui_ctx.set_fonts(fonts);
    }

    fn setup_style(cc: &eframe::CreationContext<'_>) {
        use FontFamily::{Proportional, Monospace};
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::new(25.0, Proportional)),
            (TextStyle::Body, FontId::new(16.0, Proportional)),
            (TextStyle::Monospace, FontId::new(12.0, Monospace)),
            (TextStyle::Button, FontId::new(12.0, Proportional)),
            (TextStyle::Small, FontId::new(8.0, Proportional)),
            (TextStyle::Name("Timer".into()), FontId::new(24.0, Monospace)),
        ].into();

        cc.egui_ctx.set_style(style);
    } 

    fn setup_visuals(cc: &eframe::CreationContext<'_>, color_scheme:& AppColorScheme) {

        cc.egui_ctx.set_visuals(Visuals {
            panel_fill: color_scheme.fill_color,
            window_fill: color_scheme.fill_color,
            extreme_bg_color: color_scheme.timer_paused,
            widgets: TimerApp::standard_widget_visuals(color_scheme),
            ..Default::default()
        });
    }
    pub fn standard_widget_visuals(color_scheme:& AppColorScheme) -> Widgets {
        Widgets {
                inactive: WidgetVisuals {
                    bg_fill: color_scheme.ligth_bg_color,
                    weak_bg_fill: color_scheme.ligth_bg_color,
                    bg_stroke: egui::Stroke::new(1.0, color_scheme.ligth_bg_stroke),
                    fg_stroke: egui::Stroke::new(1.0, color_scheme.ligth_fg_stroke),
                    rounding: egui::Rounding::same(3.0),
                    expansion: 0.0
                },
                noninteractive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(217,93,57),
                    weak_bg_fill: Color32::from_rgb(217,93,57),
                    bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(240,162,2)),
                    fg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(249, 224, 217)),
                    rounding: egui::Rounding::same(3.0),
                    expansion: 0.0
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgb(241, 136, 5),
                    weak_bg_fill: Color32::from_rgb(241, 136, 5),
                    bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(253, 186, 53)),
                    fg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(249, 224, 217)),
                    rounding: egui::Rounding::same(3.0),
                    expansion: 0.5
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgb(241, 136, 5),
                    weak_bg_fill: Color32::from_rgb(241, 136, 5),
                    bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(253, 186, 53)),
                    fg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(249, 224, 217)),
                    rounding: egui::Rounding::same(3.0),
                    expansion: -0.5
                },
                open: WidgetVisuals {
                    bg_fill: Color32::from_rgb(241, 136, 5),
                    weak_bg_fill: Color32::from_rgb(241, 136, 5),
                    bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(253, 186, 53)),
                    fg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(249, 224, 217)),
                    rounding: egui::Rounding::same(3.0),
                    expansion: -0.5
                },
            }
    }

    pub fn new(cc: &eframe::CreationContext<'_>, timer_data: Arc<Mutex<TimerData>>) -> Self {
        let color_scheme = AppColorScheme::default();   
        cc.egui_ctx.set_pixels_per_point(2.5);
        TimerApp::setup_fonts(cc);
        TimerApp::setup_style(cc);
        TimerApp::setup_visuals(cc, &color_scheme);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut app: TimerApp = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            app.timer_data = Some(timer_data);
            return app 
        }
        
        let mut app = TimerApp::default();
        app.timer_data = Some(timer_data);
        return app
    }

    fn draw_timer_text_element<'a>(ui: &'a mut Ui, open_data: &mut std::sync::MutexGuard<'_, TimerData>, color_scheme:& AppColorScheme ) {
        match open_data.timer_state {
            TimerState::Started(_) => {ui.visuals_mut().extreme_bg_color = color_scheme.timer_active}
            _ => {ui.visuals_mut().extreme_bg_color = color_scheme.timer_paused}


        }

        let mut display_string = match open_data.timer_state {
            TimerState::Started(_)|TimerState::Paused(_) => {
                format!("{}", TimerData::as_minutes(&open_data.current_left))
            }
            TimerState::Done => {
                format!("{}", TimerData::as_minutes(&open_data.total_time))
            }
        };
        
        

        let response = ui.add_sized([100.0, 30.0], egui::TextEdit::singleline(&mut display_string)
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
    }

    fn draw_skip_button_element<'a>(ui: &'a mut Ui, open_data: &mut std::sync::MutexGuard<'_, TimerData> ) {
        if ui.add_sized([10.0, 30.0], Button::new(">")).clicked() {
            match open_data.timer_state {
                TimerState::Done => (),
                TimerState::Paused(_)|TimerState::Started(_) => {open_data.timer_state = TimerState::Done}
            }
        }

    } 

    fn draw_pause_button_element<'a>(ui: &'a mut Ui, open_data: &mut std::sync::MutexGuard<'_, TimerData> ) {
         let button_string = match open_data.timer_state {
                TimerState::Paused(_) => "Restart timer",
                TimerState::Done => "Start Timer",
                TimerState::Started(_) => "Pause"
            };
        if ui.add_sized([80.0, 10.0], egui::Button::new(button_string)).clicked() {
            match open_data.timer_state {
                TimerState::Done => {open_data.timer_state = TimerState::Started(Instant::now())}
                TimerState::Started(started_time) => {open_data.timer_state = TimerState::Paused(started_time.elapsed())} 
                TimerState::Paused(paused_time) => {open_data.timer_state = TimerState::Started(Instant::now()-paused_time)}
                _ => {open_data.timer_state = TimerState::Done}
            }
        }

    } 
    fn draw_set_time_buttons_element<'a>(ui: &'a mut Ui, open_data: &mut std::sync::MutexGuard<'_, TimerData> ){
       ui.horizontal(|ui|{ 
            let changing_time_allowed = open_data.timer_state == TimerState::Done;
            if ui.add_enabled(changing_time_allowed, egui::Button::new("Work")).clicked() {
                open_data.total_time = Duration::from_secs(25*60); 
            }
            if ui.add_enabled(changing_time_allowed, egui::Button::new("Short")).clicked() {
                open_data.total_time = Duration::from_secs(5*60); 
            }
            if ui.add_enabled(changing_time_allowed, egui::Button::new("Long")).clicked() {
                open_data.total_time = Duration::from_secs(15*60); 
            }
        });
    }

    pub fn draw_from_state<'a>(ui: &'a mut Ui, open_data: &mut std::sync::MutexGuard<'_, TimerData>, color_scheme:& AppColorScheme ) -> &'a mut Ui {
        ui.horizontal(|ui| {
            TimerApp::draw_timer_text_element(ui, open_data, color_scheme);
            TimerApp::draw_skip_button_element(ui, open_data);
        });
        TimerApp::draw_pause_button_element(ui, open_data); 
        TimerApp::draw_set_time_buttons_element(ui, open_data);
        return ui
    }
}


impl eframe::App for TimerApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn persist_native_window(&self) -> bool { false }
    fn persist_egui_memory(&self) -> bool { false }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut open_data = self.timer_data.as_ref().unwrap().lock().unwrap();
        let color_scheme = &self.color_scheme;
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
            let _ui = TimerApp::draw_from_state(ui, &mut open_data, color_scheme);
            });
        ctx.request_repaint_after(Duration::from_secs(1));
    }


}
