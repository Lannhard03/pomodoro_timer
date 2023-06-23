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
use crate::app::Setting;

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

impl Default for TimerData{
    fn default() -> Self{
        Self {
            timer_state: TimerState::Done, 
            work_time: WorkTimes::Work,
        }
    }
    
}
impl TimerData{
    pub fn timer_state(&self) -> &TimerState{
        &self.timer_state
    }
    pub fn timer_state_mut(&mut self) -> &mut TimerState{
        &mut self.timer_state
    }

    pub fn work_time(&self) -> &WorkTimes {
        &self.work_time
    }
    pub fn work_time_mut(&mut self) -> &mut WorkTimes {
        &mut self.work_time
    }
    pub fn dur_as_minutes(dur: &Duration) -> String {
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
    pub fn minutes_as_dur(minute: &String) -> Option<Duration> {
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
    pub fn get_work_time(work_time: &WorkTimes, work_time_setting: & HashMap<WorkTimes, Duration>) -> Duration {
        work_time_setting.get(work_time).unwrap_or(& Duration::from_secs(0)).clone()
    }
    pub fn calculate_timer_text(&self, settings: & Setting) -> String {
        
        match self.timer_state {
            TimerState::Started(time_stamp) => {
                format!("{}", TimerData::dur_as_minutes(&(TimerData::get_work_time(&self.work_time, &settings.work_time_settings())
                                                          .checked_sub(time_stamp.elapsed()).unwrap_or(Duration::from_secs(0)))))
            }
            TimerState::Done => {
                format!("{}", TimerData::dur_as_minutes(&TimerData::get_work_time(&self.work_time, &settings.work_time_settings())))
            }
            TimerState::Paused(paused_time) => {
                format!("{}", TimerData::dur_as_minutes(&(TimerData::get_work_time(&self.work_time, &settings.work_time_settings())-paused_time)))
            }
        }
    }
    pub fn load_editable_settings(settings: &Setting) -> Vec<String> {
        let worktimes_map = &settings.work_time_settings();
        let mut editable_strings: Vec<String> = Vec::new();
        editable_strings.push(TimerData::dur_as_minutes(worktimes_map.get(&WorkTimes::Work).unwrap()));
        editable_strings.push(TimerData::dur_as_minutes(worktimes_map.get(&WorkTimes::Long).unwrap()));
        editable_strings.push(TimerData::dur_as_minutes(worktimes_map.get(&WorkTimes::Short).unwrap()));
        editable_strings
    }


    pub fn update(&mut self, settings: & Setting) -> Result<(), AlertPlayingError> {
        match self.timer_state {
            TimerState::Done => return Ok(()),
            TimerState::Paused(_) => Ok(()),
            TimerState::Started(time_stamp) => {
                let time = TimerData::get_work_time(&self.work_time, &settings.work_time_settings()).checked_sub(time_stamp.elapsed());
                if time == None {
                    self.timer_state = TimerState::Done;
                    println!("heje");
                    let res = TimerData::play_alert(settings.alert_sound_setting());
                    match self.work_time {
                        WorkTimes::Work => {self.work_time = WorkTimes::Short},
                        _ => {self.work_time = WorkTimes::Work},
                    }
                    return res
                } else {return Ok(())}
            }

        }
    }
    pub fn play_alert(audio_path: &String) -> Result<(), AlertPlayingError>{
        let path_clone = audio_path.clone();
        thread::spawn(move || {
            let (_stream, stream_handle) = match rodio::OutputStream::try_default() {
                Ok((output_stream, stream_handle)) => Some((output_stream, stream_handle)),
                Err(_) => {return Err(AlertPlayingError)},
            }.unwrap();
            //not hanlding errors?? Cringe!
            let file = BufReader::new(match File::open(path_clone) {
                Ok(file) => Some(file),
                Err(_) => {return Err(AlertPlayingError)},
            }.unwrap());
            let source = match rodio::Decoder::new(file) {
                Ok(source) => Some(source),
                Err(_) => {return Err(AlertPlayingError)},
            }.unwrap();
            //If step in thread errors... too bad I won't bother with errors from multithreading.
            stream_handle.play_raw(source.convert_samples()).or(Err(AlertPlayingError));
            thread::sleep(Duration::from_secs(5)); 
            Ok(())
        });
        Ok(())
    }

}
#[derive(Debug, Clone)]
pub struct AlertPlayingError;





