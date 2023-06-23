#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod visuals;
mod custom_widgets;
mod timer;
pub use app::TimerApp;
pub use visuals::AppColorScheme;
pub use timer::{TimerData, TimerState, WorkTimes};
