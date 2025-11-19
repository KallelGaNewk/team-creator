use eframe::egui::{Slider, Ui};
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};
use std::{io::Cursor, time::Duration};

/// Formats [`Duration`] into a [`String`] with HH:MM:SS or MM:SS depending on inputted [`Duration`]
///
/// # Examples
///
/// ```
/// let formatted = format_duration(Duration::from_secs(64)); // Returns "01:04"
/// let formatted = format_duration(Duration::from_secs(5422)); // Returns "01:30:22"
/// ```
pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / 60) / 60;
    if hours >= 1 {
        format!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}")
    } else {
        format!("{minutes:0>2}:{seconds:0>2}")
    }
}

/// Reflects the current state of the [`Player`]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerState {
    Playing,
    Paused,
    Ended,
}

/// Audio player that plays audio from bytes
pub struct Player {
    audio_bytes: Vec<u8>,
    pub total_time: Duration,
    pub player_state: PlayerState,
    was_playing_before_drag: bool,
    seek_position: Option<Duration>,

    // Rodio handles all the complexity
    _stream: OutputStream,
    sink: Sink,
}

impl Player {
    /// Creates a new audio player from audio bytes
    ///
    /// # Example
    ///
    /// ```
    /// let audio_bytes = std::fs::read("audio.mp3").unwrap();
    /// let player = Player::new(audio_bytes);
    /// ```
    pub fn new(bytes: Vec<u8>) -> Self {
        let total_time = Self::get_total_duration(&bytes);

        let stream =
            OutputStreamBuilder::open_default_stream().expect("Failed to open audio stream");
        let sink = Sink::connect_new(&stream.mixer());
        sink.pause(); // Start paused

        // Load the audio source
        let cursor = Cursor::new(bytes.clone());
        if let Ok(source) = Decoder::new(cursor) {
            sink.append(source);
        }

        Self {
            audio_bytes: bytes,
            total_time,
            player_state: PlayerState::Paused,
            was_playing_before_drag: false,
            seek_position: None,
            _stream: stream,
            sink,
        }
    }

    /// Gets the total duration of the audio from bytes
    fn get_total_duration(bytes: &[u8]) -> Duration {
        let cursor = Cursor::new(bytes.to_vec());
        if let Ok(decoder) = Decoder::new(cursor) {
            // Try to get duration directly (works for some formats)
            if let Some(duration) = decoder.total_duration() {
                return duration;
            }

            // For formats like OGG that don't provide duration upfront,
            // calculate it from samples
            let sample_rate = decoder.sample_rate();
            let channels = decoder.channels();
            let total_samples: u64 = decoder.count() as u64;

            if sample_rate > 0 && channels > 0 {
                let frames = total_samples / channels as u64;
                let seconds = frames as f64 / sample_rate as f64;
                return Duration::from_secs_f64(seconds);
            }
        }

        Duration::ZERO
    }

    /// Gets current playback position
    fn elapsed_time(&self) -> Duration {
        self.sink.get_pos()
    }

    /// Call this to show the player on screen
    pub fn ui(&mut self, ui: &mut Ui) {
        let elapsed = self.elapsed_time();

        // Check if playback ended - sink.empty() means all sources finished playing
        if self.player_state == PlayerState::Playing && self.sink.empty() {
            self.player_state = PlayerState::Ended;
        }

        ui.horizontal(|ui| {
            ui.label(format!(
                "{} / {}",
                format_duration(elapsed),
                format_duration(self.total_time)
            ));

            let state_icon = match self.player_state {
                PlayerState::Playing => "⏸",
                PlayerState::Paused => "▶",
                PlayerState::Ended => "↺",
            };

            if ui.button(state_icon).clicked() {
                match self.player_state {
                    PlayerState::Playing => {
                        self.sink.pause();
                        self.player_state = PlayerState::Paused;
                    }
                    PlayerState::Paused => {
                        self.sink.play();
                        self.player_state = PlayerState::Playing;
                    }
                    PlayerState::Ended => {
                        self.reload_audio();
                        self.sink.play();
                        self.player_state = PlayerState::Playing;
                    }
                }
            }

            let mut slider_value = elapsed.as_secs_f32();
            let slider = Slider::new(&mut slider_value, 0.0..=self.total_time.as_secs_f32())
                .show_value(false);
            let slider_response = ui.add(slider);

            if slider_response.drag_started() {
                self.was_playing_before_drag = self.player_state == PlayerState::Playing;
                self.sink.pause();
                self.player_state = PlayerState::Paused;
            }

            if slider_response.dragged() {
                // Just store the position while dragging
                self.seek_position = Some(Duration::from_secs_f32(slider_value));
            }

            if slider_response.drag_stopped() {
                // Reload audio and seek once when drag stops
                if let Some(seek_pos) = self.seek_position.take() {
                    self.reload_audio();
                    let _ = self.sink.try_seek(seek_pos);

                    // Resume playback if we were playing before dragging
                    if self.was_playing_before_drag {
                        self.sink.play();
                        self.player_state = PlayerState::Playing;
                    }
                }
                self.was_playing_before_drag = false;
            }

            let mut volume = (self.sink.volume() * 100.0) as i32;
            let volume_icon = if volume > 70 {
                "🔊"
            } else if volume > 40 {
                "🔉"
            } else if volume > 0 {
                "🔈"
            } else {
                "🔇"
            };

            ui.menu_button(volume_icon, |ui| {
                if ui
                    .add(Slider::new(&mut volume, 0..=100).vertical())
                    .changed()
                {
                    self.sink.set_volume(volume as f32 / 100.0);
                }
            });
        });

        ui.ctx().request_repaint_after(Duration::from_millis(100));
    }

    /// Reloads the audio source (used for restart)
    fn reload_audio(&mut self) {
        self.sink.clear();
        let cursor = Cursor::new(self.audio_bytes.clone());
        if let Ok(source) = Decoder::new(cursor) {
            self.sink.append(source);
        }
    }
}
