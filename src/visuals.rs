use eframe::{egui::{TextBuffer, Visuals, Color32, Frame, Rect, Pos2, Vec2, Sense, RichText, TextFormat, FontFamily, FontId, Label}};
use egui::{Margin, Response};
use egui::Rounding;
use egui::Stroke;
use egui::style::Spacing;
use egui::style::{Widgets, WidgetVisuals};
use egui::{text_edit, Button, Ui, TextStyle, Widget};

pub struct TimerAppVisuals {
    color_scheme: AppColorScheme, 
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppColorScheme{
    pub fill_color: Color32,
    pub timer_paused: Color32,
    pub timer_active: Color32,
    pub ligth_bg_color: Color32,
    pub dark_bg_color: Color32,
    pub ligth_bg_stroke: Color32,
    pub ligth_fg_stroke: Color32,
    pub dark_bg_stroke: Color32,
    pub dark_fg_stroke: Color32,
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


impl Default for TimerAppVisuals {
    fn default() -> Self {
        TimerAppVisuals {color_scheme: AppColorScheme::default(),}
    }
}

impl TimerAppVisuals {
    pub fn setup_app_visuals(&mut self, cc: &eframe::CreationContext<'_>) {
        cc.egui_ctx.set_pixels_per_point(2.5);
        TimerAppVisuals::setup_fonts(cc);
        TimerAppVisuals::setup_style(cc);
        self.setup_visuals(cc);
    }
    fn setup_fonts(cc: &eframe::CreationContext<'_>) {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert("Roboto".to_owned(), egui::FontData::from_static(include_bytes!("../assets/Roboto-Regular.ttf")));
        fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, "Roboto".to_owned());
        cc.egui_ctx.set_fonts(fonts);
    }
    fn setup_visuals(&self, cc: &eframe::CreationContext<'_>) {
        cc.egui_ctx.set_visuals(Visuals {
            panel_fill: self.color_scheme.fill_color,
            window_fill: self.color_scheme.fill_color,
            selection: egui::style::Selection {bg_fill: self.color_scheme.ligth_bg_color, stroke: Stroke::new(1.0, self.color_scheme.ligth_fg_stroke)},
            extreme_bg_color: self.color_scheme.timer_paused,
            widgets: self.standard_widget_visuals(),
            ..Default::default()
        });
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

    pub fn standard_widget_visuals(& self) -> Widgets {
        Widgets {
                inactive: WidgetVisuals {
                    bg_fill: self.color_scheme.ligth_bg_color,
                    weak_bg_fill: self.color_scheme.ligth_bg_color,
                    bg_stroke: egui::Stroke::new(1.0, self.color_scheme.ligth_bg_stroke),
                    fg_stroke: egui::Stroke::new(1.0, self.color_scheme.ligth_fg_stroke),
                    rounding: egui::Rounding::same(3.0),
                    expansion: 0.0
                },
                noninteractive: WidgetVisuals {
                    bg_fill: self.color_scheme.ligth_bg_color,
                    weak_bg_fill: self.color_scheme.ligth_bg_color,
                    bg_stroke: egui::Stroke::new(1.0, self.color_scheme.ligth_bg_stroke),
                    fg_stroke: egui::Stroke::new(1.0, self.color_scheme.ligth_fg_stroke),
                    rounding: egui::Rounding::same(3.0),
                    expansion: 0.0
                },
                hovered: WidgetVisuals {
                    bg_fill: self.color_scheme.dark_bg_color,
                    weak_bg_fill: self.color_scheme.dark_bg_color, 
                    bg_stroke: egui::Stroke::new(1.0, self.color_scheme.dark_bg_stroke),
                    fg_stroke: egui::Stroke::new(1.0, self.color_scheme.dark_fg_stroke),
                    rounding: egui::Rounding::same(3.0),
                    expansion: 0.5
                },
                active: WidgetVisuals {
                    bg_fill: self.color_scheme.dark_bg_color,
                    weak_bg_fill: self.color_scheme.dark_bg_color,
                    bg_stroke: egui::Stroke::new(1.0, self.color_scheme.dark_bg_stroke),
                    fg_stroke: egui::Stroke::new(1.0, self.color_scheme.dark_fg_stroke),
                    rounding: egui::Rounding::same(3.0),
                    expansion: -0.5
                },
                open: WidgetVisuals {
                    bg_fill: self.color_scheme.ligth_bg_color,
                    weak_bg_fill: self.color_scheme.ligth_bg_color,
                    bg_stroke: egui::Stroke::new(1.0, self.color_scheme.ligth_bg_stroke),
                    fg_stroke: egui::Stroke::new(1.0, self.color_scheme.ligth_fg_stroke),
                    rounding: egui::Rounding::same(3.0),
                    expansion: -0.5
                },
            }
    }
}





