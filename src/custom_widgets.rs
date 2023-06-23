use eframe::egui::{Color32, RichText};
use egui::Rounding;
use egui::{Margin, Response};
use egui::{TextStyle, Ui, Widget};

pub struct TimerDisplay {
    timer_bg_color: Color32,
    border_color: Color32,
    timer_text: String,
}
impl Widget for TimerDisplay {
    fn ui(self, ui: &mut Ui) -> Response {
        let inner_respons = egui::Frame::none()
            .fill(self.timer_bg_color)
            .inner_margin(Margin::same(0.0))
            .outer_margin(Margin::same(0.0))
            .rounding(Rounding::same(5.0))
            .stroke(egui::Stroke::new(2.0, self.border_color))
            .show(ui, |ui| {
                ui.add_sized(
                    [100.0, 30.0],
                    egui::Label::new(
                        RichText::new(self.timer_text).text_style(TextStyle::Name("Timer".into())),
                    ),
                )
            });
        inner_respons.inner
    }
}
impl TimerDisplay {
    pub fn new(timer_bg_color: Color32, border_color: Color32, timer_text: String) -> TimerDisplay {
        TimerDisplay {
            timer_bg_color,
            timer_text,
            border_color,
        }
    }
}
