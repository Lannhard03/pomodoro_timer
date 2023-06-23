use crate::custom_widgets::TimerDisplay;
use crate::timer::{TimerData, TimerState, WorkTimes};
use crate::visuals::TimerAppVisuals;
use crate::AppColorScheme;
use eframe::egui::RichText;
use egui::{Button, TextStyle, Ui};
use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

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

#[derive(PartialEq, Eq, Clone)]
pub enum Screen {
    TimerScreen,
    SettingsScreen { editable_settings: Vec<String> },
}

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Setting {
    work_times_settings: HashMap<WorkTimes, Duration>,
    alert_sound_path: String,
}

impl Default for Setting {
    fn default() -> Self {
        Setting {
            work_times_settings: HashMap::from([
                (WorkTimes::Work, Duration::from_secs(25 * 60)),
                (WorkTimes::Short, Duration::from_secs(5 * 60)),
                (WorkTimes::Long, Duration::from_secs(15 * 60)),
            ]),
            alert_sound_path: "assets/alert_sound.wav".into(),
        }
    }
}

impl Setting {
    pub fn work_time_settings(&self) -> &HashMap<WorkTimes, Duration> {
        &self.work_times_settings
    }
    pub fn alert_sound_setting(&self) -> &String {
        &self.alert_sound_path
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
        let timer_bg_color = match self.timer_data.timer_state() {
            TimerState::Started(_) => self.color_scheme.timer_active,
            _ => self.color_scheme.timer_paused,
        };
        let display_string = self.timer_data.calculate_timer_text(&self.settings);
        ui.add(TimerDisplay::new(
            timer_bg_color,
            self.color_scheme.ligth_bg_stroke,
            display_string,
        ));
    }

    fn draw_skip_button_element<'a>(&mut self, ui: &'a mut Ui) {
        if ui.add_sized([10.0, 30.0], Button::new(">")).clicked() {
            match self.timer_data.timer_state() {
                TimerState::Done => (),
                TimerState::Paused(_) | TimerState::Started(_) => {
                    *self.timer_data.timer_state_mut() = TimerState::Done
                }
            }
        }
    }

    fn draw_pause_button_element<'a>(&mut self, ui: &'a mut Ui) {
        let button_string = match self.timer_data.timer_state() {
            TimerState::Paused(_) => "Restart timer",
            TimerState::Done => "Start Timer",
            TimerState::Started(_) => "Pause",
        };
        if ui
            .add_sized([80.0, 10.0], egui::Button::new(button_string))
            .clicked()
        {
            match self.timer_data.timer_state() {
                TimerState::Done => {
                    *self.timer_data.timer_state_mut() = TimerState::Started(Instant::now())
                }
                TimerState::Started(started_time) => {
                    *self.timer_data.timer_state_mut() = TimerState::Paused(started_time.elapsed())
                }
                TimerState::Paused(paused_time) => {
                    *self.timer_data.timer_state_mut() =
                        TimerState::Started(Instant::now() - *paused_time)
                }
            }
        }
    }
    fn draw_set_time_buttons_element<'a>(&mut self, ui: &'a mut Ui) {
        ui.vertical(|ui| {
            let changing_time_allowed = match self.timer_data.timer_state() {
                TimerState::Started(_) => false,
                _ => true,
            };
            if ui
                .add_enabled(
                    changing_time_allowed,
                    egui::RadioButton::new(
                        *self.timer_data.work_time_mut() == WorkTimes::Work,
                        "Work",
                    ),
                )
                .clicked()
            {
                *self.timer_data.work_time_mut() = WorkTimes::Work;
                *self.timer_data.timer_state_mut() = TimerState::Done;
            }
            if ui
                .add_enabled(
                    changing_time_allowed,
                    egui::RadioButton::new(
                        *self.timer_data.work_time() == WorkTimes::Short,
                        "Short",
                    ),
                )
                .clicked()
            {
                *self.timer_data.work_time_mut() = WorkTimes::Short;
                *self.timer_data.timer_state_mut() = TimerState::Done;
            }
            if ui
                .add_enabled(
                    changing_time_allowed,
                    egui::RadioButton::new(*self.timer_data.work_time() == WorkTimes::Long, "Long"),
                )
                .clicked()
            {
                *self.timer_data.work_time_mut() = WorkTimes::Long;
                *self.timer_data.timer_state_mut() = TimerState::Done;
            }
        });
    }

    pub fn draw_timer_screen<'a>(&mut self, ui: &'a mut Ui) {
        ui.horizontal(|ui| {
            self.draw_timer_text_element(ui);
            self.draw_skip_button_element(ui);
        });
        self.draw_pause_button_element(ui);
        self.draw_set_time_buttons_element(ui);
    }
    pub fn draw_settings_screen<'a>(&mut self, ui: &'a mut Ui) {
        let editable_settings = match &mut self.current_screen {
            Screen::SettingsScreen { editable_settings } => Some(editable_settings),
            Screen::TimerScreen => None,
        }
        .unwrap(); //We already know screen is settingsScreen, but borrow checker demands we have
                   //a match statement here.
        ui.add(egui::TextEdit::singleline(&mut editable_settings[0]));
        ui.add(egui::TextEdit::singleline(&mut editable_settings[1]));
        ui.add(egui::TextEdit::singleline(&mut editable_settings[2]));
    }
    pub fn validate_work_time_setting(
        settings: &mut Setting,
        new_val: &String,
        work_time_setting: &WorkTimes,
    ) {
        let as_dur = TimerData::minutes_as_dur(new_val);
        match as_dur {
            Some(dur) => {
                *settings
                    .work_times_settings
                    .get_mut(&work_time_setting)
                    .unwrap() = dur
            }
            None => (),
        };
    }
}

impl eframe::App for TimerApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn persist_native_window(&self) -> bool {
        false
    }
    fn persist_egui_memory(&self) -> bool {
        false
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //here we should somehow show the error in some popup dialog!
        self.timer_data.update(&self.settings);

        // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                //Size of button is bugged/weird, make a custom menu_button?
                #[cfg(not(target_arch = "wasm32"))]
                ui.menu_button("X", |ui| {
                    if ui
                        .add_sized(
                            [20.0, 10.0],
                            egui::Button::new(
                                RichText::new("Confirm?")
                                    .text_style(TextStyle::Name("Small Text".into())),
                            ),
                        )
                        .clicked()
                    {
                        _frame.close();
                    }
                });

                if ui
                    .add(egui::SelectableLabel::new(
                        self.current_screen != Screen::TimerScreen,
                        "Settings",
                    ))
                    .clicked()
                {
                    match &self.current_screen {
                        Screen::SettingsScreen { editable_settings } => {
                            TimerApp::validate_work_time_setting(
                                &mut self.settings,
                                &editable_settings[0],
                                &WorkTimes::Work,
                            );
                            TimerApp::validate_work_time_setting(
                                &mut self.settings,
                                &editable_settings[1],
                                &WorkTimes::Long,
                            );
                            TimerApp::validate_work_time_setting(
                                &mut self.settings,
                                &editable_settings[2],
                                &WorkTimes::Short,
                            );
                            self.current_screen = Screen::TimerScreen
                        }
                        Screen::TimerScreen => {
                            self.current_screen = Screen::SettingsScreen {
                                editable_settings: TimerData::load_editable_settings(
                                    &self.settings,
                                ),
                            }
                        }
                    }
                }
            });
        });
        let cur_screen = self.current_screen.clone();
        egui::CentralPanel::default().show(ctx, |ui| {
            match cur_screen {
                Screen::TimerScreen => self.draw_timer_screen(ui),
                Screen::SettingsScreen { editable_settings } => {
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
