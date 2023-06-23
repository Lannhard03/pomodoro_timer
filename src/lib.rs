#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod custom_widgets;
mod timer;
mod visuals;
pub use app::TimerApp;
pub use timer::{TimerData, TimerState, WorkTimes};
pub use visuals::AppColorScheme;
