use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;
use eframe::{egui::{TextBuffer, Visuals, Color32, Frame, Rect, Pos2, Vec2, Sense, RichText, TextFormat, FontFamily, FontId, Label}};
use egui::Margin;
use egui::Rounding;
use egui::Stroke;
use egui::style::Spacing;
use egui::style::{Widgets, WidgetVisuals};
use egui::{text_edit, Button, Ui, TextStyle, Widget};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TimerApp {
    settings: Setting,
    #[serde(skip)]
    timer_data: TimerData,
    color_scheme: AppColorScheme,
    #[serde(skip)]
    current_screen: Screen
}


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TimerData{
    #[serde(skip)]
    timer_state: TimerState,
    #[serde(skip)]
    work_time: WorkTimes, 
}

#[derive(PartialEq, Eq)]
pub enum TimerState{
    Started (Instant),
    Paused (Duration),
    Done,
}

#[derive(PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum WorkTimes {
    Work, 
    Short, 
    Long,
}
#[derive(PartialEq, Eq)]
pub enum Screen {
    TimerScreen,
    SettingsScreen,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Setting{
    work_times_times: HashMap<WorkTimes, Duration>,  //needs to store time data aswell?
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
impl Default for Setting{
    fn default() -> Self {
        Setting {
            work_times_times: HashMap::from([(WorkTimes::Work, Duration::from_secs(25*60)),
                                             (WorkTimes::Short, Duration::from_secs(5*60)),
                                             (WorkTimes::Long, Duration::from_secs(15*60))]),
        }
    }
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


impl Default for TimerData{
    fn default() -> Self{
        Self {
            timer_state: TimerState::Done, 
            work_time: WorkTimes::Work,
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
    fn get_work_time(& self, work_time_setting: & HashMap<WorkTimes, Duration>) -> Duration {
        work_time_setting.get(& self.work_time).unwrap_or(& Duration::from_secs(0)).clone()
    }
}

impl Default for TimerApp {
    fn default() -> Self {
        Self {
            settings: Setting::default(),
            timer_data: TimerData::default(),
            color_scheme: AppColorScheme::default(),
            current_screen: Screen::TimerScreen,
        }
    }
}

impl TimerApp {
    /// Called once before the first frame.
    fn setup_fonts(cc: &eframe::CreationContext<'_>) {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert("Roboto".to_owned(), egui::FontData::from_static(include_bytes!("../assets/Roboto-Regular.ttf")));
        fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, "Roboto".to_owned());
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
            (TextStyle::Name("Small Text".into()), FontId::new(8.0, Proportional)),
        ].into();

        style.spacing = Spacing {
            menu_margin: Margin {left: -40.0, right: -40.0, top: 0.0, bottom: 0.0},
            button_padding: Vec2::new(0.0, 0.0),
            ..Default::default()
        };
        cc.egui_ctx.set_style(style);
    } 

    fn setup_visuals(cc: &eframe::CreationContext<'_>, color_scheme:& AppColorScheme) {
        cc.egui_ctx.set_visuals(Visuals {
            panel_fill: color_scheme.fill_color,
            window_fill: color_scheme.fill_color,
            selection: egui::style::Selection {bg_fill: color_scheme.ligth_bg_color, stroke: Stroke::new(1.0, color_scheme.ligth_fg_stroke)},
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
                    bg_fill: color_scheme.ligth_bg_color,
                    weak_bg_fill: color_scheme.ligth_bg_color,
                    bg_stroke: egui::Stroke::new(1.0, color_scheme.ligth_bg_stroke),
                    fg_stroke: egui::Stroke::new(1.0, color_scheme.ligth_fg_stroke),
                    rounding: egui::Rounding::same(3.0),
                    expansion: 0.0
                },
                hovered: WidgetVisuals {
                    bg_fill: color_scheme.dark_bg_color,
                    weak_bg_fill: color_scheme.dark_bg_color, 
                    bg_stroke: egui::Stroke::new(1.0, color_scheme.dark_bg_stroke),
                    fg_stroke: egui::Stroke::new(1.0, color_scheme.dark_fg_stroke),
                    rounding: egui::Rounding::same(3.0),
                    expansion: 0.5
                },
                active: WidgetVisuals {
                    bg_fill: color_scheme.dark_bg_color,
                    weak_bg_fill: color_scheme.dark_bg_color,
                    bg_stroke: egui::Stroke::new(1.0, color_scheme.dark_bg_stroke),
                    fg_stroke: egui::Stroke::new(1.0, color_scheme.dark_fg_stroke),
                    rounding: egui::Rounding::same(3.0),
                    expansion: -0.5
                },
                open: WidgetVisuals {
                    bg_fill: color_scheme.ligth_bg_color,
                    weak_bg_fill: color_scheme.ligth_bg_color,
                    bg_stroke: egui::Stroke::new(1.0, color_scheme.ligth_bg_stroke),
                    fg_stroke: egui::Stroke::new(1.0, color_scheme.ligth_fg_stroke),
                    rounding: egui::Rounding::same(3.0),
                    expansion: -0.5
                },
            }
    }

    pub fn new(cc: &eframe::CreationContext<'_>, timer_data: TimerData) -> Self {
        let color_scheme = AppColorScheme::default();   
        cc.egui_ctx.set_pixels_per_point(2.5);
        TimerApp::setup_fonts(cc);
        TimerApp::setup_style(cc);
        TimerApp::setup_visuals(cc, &color_scheme);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut app: TimerApp = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            app.timer_data = timer_data;
            return app 
        }
        
        let mut app = TimerApp::default();
        app.timer_data = timer_data;
        return app
    }

    fn draw_timer_text_element<'a>(&mut self, ui: &'a mut Ui) {
        let timer_bg_color = match self.timer_data.timer_state {
            TimerState::Started(_) => {self.color_scheme.timer_active}
            _ => {self.color_scheme.timer_paused}


        };
        let display_string = match self.timer_data.timer_state {
            TimerState::Started(time_stamp) => {
                format!("{}", TimerData::as_minutes(&(self.timer_data.get_work_time(&self.settings.work_times_times)-time_stamp.elapsed())))
            }
            TimerState::Done => {
                format!("{}", TimerData::as_minutes(&self.timer_data.get_work_time(&self.settings.work_times_times)))
            }
            TimerState::Paused(paused_time) => {
                format!("{}", TimerData::as_minutes(&(self.timer_data.get_work_time(&self.settings.work_times_times)-paused_time)))
            }
        };
        
        egui::Frame::none().fill(timer_bg_color)
                           .inner_margin(Margin::same(0.0))
                           .outer_margin(Margin::same(0.0))
                           .rounding(Rounding::same(5.0))
                           .stroke(egui::Stroke::new(2.0, self.color_scheme.ligth_bg_stroke))
                           .show(ui, |ui| {
            ui.add_sized([100.0, 30.0], egui::Label::new(RichText::new(display_string).text_style(TextStyle::Name("Timer".into()))));
        });
    } 
    

    fn draw_skip_button_element<'a>(&mut self, ui: &'a mut Ui) {
        if ui.add_sized([10.0, 30.0], Button::new(">")).clicked() {
            match self.timer_data.timer_state {
                TimerState::Done => (),
                TimerState::Paused(_)|TimerState::Started(_) => {self.timer_data.timer_state = TimerState::Done}
            }
        }

    } 

    fn draw_pause_button_element<'a>(&mut self, ui: &'a mut Ui) {
         let button_string = match self.timer_data.timer_state {
                TimerState::Paused(_) => "Restart timer",
                TimerState::Done => "Start Timer",
                TimerState::Started(_) => "Pause"
            };
        if ui.add_sized([80.0, 10.0], egui::Button::new(button_string)).clicked() {
            match self.timer_data.timer_state {
                TimerState::Done => {self.timer_data.timer_state = TimerState::Started(Instant::now())}
                TimerState::Started(started_time) => {self.timer_data.timer_state = TimerState::Paused(started_time.elapsed())} 
                TimerState::Paused(paused_time) => {self.timer_data.timer_state = TimerState::Started(Instant::now()-paused_time)}
                _ => {self.timer_data.timer_state = TimerState::Done}
            }
        }

    } 
    fn draw_set_time_buttons_element<'a>(&mut self, ui: &'a mut Ui){
       ui.vertical(|ui|{ 
            let changing_time_allowed = match self.timer_data.timer_state {
                TimerState::Started(_) => false,
                _ => true

            };
            if ui.add_enabled(changing_time_allowed, egui::RadioButton::new(self.timer_data.work_time == WorkTimes::Work, "Work")).clicked() {
                self.timer_data.work_time = WorkTimes::Work; 
                self.timer_data.timer_state = TimerState::Done;
            }
            if ui.add_enabled(changing_time_allowed, egui::RadioButton::new(self.timer_data.work_time == WorkTimes::Short, "Short")).clicked() {
                self.timer_data.work_time = WorkTimes::Short; 
                self.timer_data.timer_state = TimerState::Done;
            }
            if ui.add_enabled(changing_time_allowed, egui::RadioButton::new(self.timer_data.work_time == WorkTimes::Long, "Long")).clicked() {
                self.timer_data.work_time = WorkTimes::Long;
                self.timer_data.timer_state = TimerState::Done;
            }
        });
    }

    pub fn draw_timer_screen<'a>(&mut self, ui: &'a mut Ui){
        ui.horizontal(|ui| {
            self.draw_timer_text_element(ui);
            self.draw_skip_button_element(ui);
        });
        self.draw_pause_button_element(ui); 
        self.draw_set_time_buttons_element(ui);
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
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                //Size of button is bugged/weird, make a custom menu_button?
                ui.menu_button("X", |ui| {
                    if ui.add_sized([20.0, 10.0], egui::Button::new(RichText::new("Confirm?").text_style(TextStyle::Name("Small Text".into())))).clicked() {
                        _frame.close();
                 }
                });

                if ui.add(egui::SelectableLabel::new(self.current_screen == Screen::SettingsScreen, "Settings")).clicked() {
                    self.current_screen = match self.current_screen {Screen::SettingsScreen => Screen::TimerScreen, Screen::TimerScreen => Screen::SettingsScreen}
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_screen {
                Screen::TimerScreen => self.draw_timer_screen(ui),
                Screen::SettingsScreen => ()
            }
        });

        if _frame.info().window_info.focused {
            ctx.request_repaint_after(Duration::from_secs(1));
        } else {
            ctx.request_repaint_after(Duration::from_secs(5));
        }
    }


}
