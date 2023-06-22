use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;
use eframe::egui::RichText;
use egui::{Button, Ui, TextStyle};
use crate::AppColorScheme;
use crate::visuals::TimerAppVisuals;
use crate::custom_widgets::TimerDisplay;
use std::fs::File;
use std::io::BufReader;
use rodio;
use rodio::source::Source;
use std::thread;
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TimerApp {
    settings: Setting,
    #[serde(skip)]
    timer_data: TimerData,
    color_scheme: AppColorScheme,
    #[serde(skip)]
    current_screen: Screen,
    #[serde(skip)]
    timer_visuals: TimerAppVisuals,
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
#[derive(PartialEq, Eq, Clone)]
pub enum Screen {
    TimerScreen,
    SettingsScreen{editable_settings: Vec<String>},
}

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Setting{
    work_times_times: HashMap<WorkTimes, Duration>,  
    alert_sound_path: String,
}


impl Default for Setting{
    fn default() -> Self {
        Setting {
            work_times_times: HashMap::from([(WorkTimes::Work, Duration::from_secs(25*60)),
                                             (WorkTimes::Short, Duration::from_secs(5*60)),
                                             (WorkTimes::Long, Duration::from_secs(15*60))]),
            alert_sound_path: "assets/alert_sound.wav".into(),
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
    fn dur_as_minutes(dur: &Duration) -> String {
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
    fn minutes_as_dur(minute: &String) -> Option<Duration> {
        let minute = minute.trim().clone();
        let display_digits = minute.split(':');
        let mut is_numeric = true;
        let converted_digits: Vec<u64> = display_digits.map(|digit_display| {
            digit_display.parse::<u64>().unwrap_or_else(|_x| {is_numeric = false; 0})
        }).collect();
        if is_numeric {
            match converted_digits.len() {
                1 => {return Some(Duration::from_secs(converted_digits[0]));}
                2 => {return Some(Duration::from_secs(60*converted_digits[0]+converted_digits[1]));}
                _ => {return None}
            }
        } else {
            return None
        }
 
    }
    fn get_work_time(work_time: &WorkTimes, work_time_setting: & HashMap<WorkTimes, Duration>) -> Duration {
        work_time_setting.get(work_time).unwrap_or(& Duration::from_secs(0)).clone()
    }
    fn calculate_timer_text(&self, settings: & Setting) -> String {
        match self.timer_state {
            TimerState::Started(time_stamp) => {
                format!("{}", TimerData::dur_as_minutes(&(TimerData::get_work_time(&self.work_time, &settings.work_times_times)
                                                          .checked_sub(time_stamp.elapsed()).unwrap_or(Duration::from_secs(0)))))
            }
            TimerState::Done => {
                format!("{}", TimerData::dur_as_minutes(&TimerData::get_work_time(&self.work_time, &settings.work_times_times)))
            }
            TimerState::Paused(paused_time) => {
                format!("{}", TimerData::dur_as_minutes(&(TimerData::get_work_time(&self.work_time, &settings.work_times_times)-paused_time)))
            }
        }
    }
    fn load_editable_settings(settings: &Setting) -> Vec<String> {
        let worktimes_map = &settings.work_times_times;
        let mut editable_strings: Vec<String> = Vec::new();
        editable_strings.push(TimerData::dur_as_minutes(worktimes_map.get(&WorkTimes::Work).unwrap()));
        editable_strings.push(TimerData::dur_as_minutes(worktimes_map.get(&WorkTimes::Long).unwrap()));
        editable_strings.push(TimerData::dur_as_minutes(worktimes_map.get(&WorkTimes::Short).unwrap()));
        editable_strings
    }
}

impl Default for TimerApp {
    fn default() -> Self {
        Self {
            settings: Setting::default(),
            timer_data: TimerData::default(),
            color_scheme: AppColorScheme::default(),
            current_screen: Screen::TimerScreen,
            timer_visuals: TimerAppVisuals::default(),
        }
    }
}

impl TimerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, timer_data: TimerData) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut app = if let Some(storage) = cc.storage {
            let mut app: TimerApp = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            app.timer_data = timer_data;
            app
        } else {
            let mut app = TimerApp::default();
            app.timer_data = timer_data;
            app
        };
        app.timer_visuals.setup_app_visuals(cc); 
        app
    }
    

    fn draw_timer_text_element<'a>(&mut self, ui: &'a mut Ui) {
        let timer_bg_color = match self.timer_data.timer_state {
            TimerState::Started(_) => {self.color_scheme.timer_active}
            _ => {self.color_scheme.timer_paused}
        };
        let display_string = self.timer_data.calculate_timer_text(&self.settings);
        ui.add(TimerDisplay::new(timer_bg_color, self.color_scheme.ligth_bg_stroke, display_string));
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
    pub fn draw_settings_screen<'a>(&mut self, ui: &'a mut Ui){
        let editable_settings = match &mut self.current_screen {
            Screen::SettingsScreen { editable_settings } => Some(editable_settings),
            Screen::TimerScreen => None,
        }.unwrap(); //We already know screen is settingsScreen, but borrow checker demands we have
                    //a match statement here.
        ui.add(egui::TextEdit::singleline(&mut editable_settings[0]));
        ui.add(egui::TextEdit::singleline(&mut editable_settings[1]));
        ui.add(egui::TextEdit::singleline(&mut editable_settings[2]));
    }
    pub fn validate_work_time_setting(settings: &mut Setting, new_val: &String, work_time_setting: &WorkTimes) {
        let as_dur = TimerData::minutes_as_dur(new_val);
        match as_dur {
            Some(dur) => *settings.work_times_times.get_mut(&work_time_setting).unwrap() = dur,
            None => (),
        };
    }
    pub fn play_alert(audio_path: &String) {
        let path_clone = audio_path.clone();
        thread::spawn( ||{
            let (_stream, stream_handle) = match rodio::OutputStream::try_default() {
                Ok((output_stream, stream_handle)) => Some((output_stream, stream_handle)),
                Err(_) => {return},
            }.unwrap();
            //not hanlding errors?? Cringe!
            let file = BufReader::new(match File::open(path_clone) {
                Ok(file) => Some(file),
                Err(_) => {return},
            }.unwrap());
            let source = match rodio::Decoder::new(file) {
                Ok(source) => Some(source),
                Err(_) => {return},
            }.unwrap();
            stream_handle.play_raw(source.convert_samples());
            thread::sleep(Duration::from_secs(5));
        });
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
        match self.timer_data.timer_state {
            TimerState::Done => (),
            TimerState::Paused(_) => (),
            TimerState::Started(time_stamp) => {
                let time = TimerData::get_work_time(&self.timer_data.work_time, &self.settings.work_times_times).checked_sub(time_stamp.elapsed());
                if time == None {
                    self.timer_data.timer_state = TimerState::Done;
                    println!("heje");
                    TimerApp::play_alert(&self.settings.alert_sound_path);
                    match self.timer_data.work_time {
                        WorkTimes::Work => {self.timer_data.work_time = WorkTimes::Short},
                        _ => {self.timer_data.work_time = WorkTimes::Work},
                    }
                }
            }

        }



         // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                //Size of button is bugged/weird, make a custom menu_button?
                #[cfg(not(target_arch = "wasm32"))]
                ui.menu_button("X", |ui| {
                    if ui.add_sized([20.0, 10.0], egui::Button::new(RichText::new("Confirm?").text_style(TextStyle::Name("Small Text".into())))).clicked() {
                        _frame.close();
                 }
                });

                if ui.add(egui::SelectableLabel::new(self.current_screen != Screen::TimerScreen, "Settings")).clicked() {
                    match &self.current_screen {
                        Screen::SettingsScreen{editable_settings} => {
                            TimerApp::validate_work_time_setting(&mut self.settings, &editable_settings[0], &WorkTimes::Work);
                            TimerApp::validate_work_time_setting(&mut self.settings, &editable_settings[1], &WorkTimes::Long);
                            TimerApp::validate_work_time_setting(&mut self.settings, &editable_settings[2], &WorkTimes::Short);
                            self.current_screen = Screen::TimerScreen
                        }
                        Screen::TimerScreen => {
                            self.current_screen = Screen::SettingsScreen{editable_settings: TimerData::load_editable_settings(& self.settings)}
                        }
                    }
                }
            });
        });
        let cur_screen = self.current_screen.clone();
        egui::CentralPanel::default().show(ctx, |ui| {
            match cur_screen {
                Screen::TimerScreen => self.draw_timer_screen(ui),
                Screen::SettingsScreen{editable_settings} => {
                    //We cant use editable_settings in function call directly due to the borrow
                    //checker 
                    self.draw_settings_screen(ui);
                }
            }
        });

        #[cfg(not(target_arch = "wasm32"))]
        if _frame.info().window_info.focused {
            ctx.request_repaint_after(Duration::from_secs(1));
        } else {
            ctx.request_repaint_after(Duration::from_secs(5));
        }
        #[cfg(target_arch = "wasm32")]
        ctx.request_repaint_after(Duration::from_secs(1));

    }


}
