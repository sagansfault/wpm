#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::time::Instant;

use egui::{RichText, TextBuffer};
use rand::{rngs::ThreadRng, seq::SliceRandom};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 500.0)),
        ..Default::default()
    };
    eframe::run_native(
        "WPM",
        options,
        Box::new(|_cc| Box::new(WordsPerMinute::new())),
    );

    Ok(())
}

struct WordsPerMinute {
    words: Vec<String>,
    current: Vec<String>,
    current_ind: usize,
    current_word: String,
    rng: ThreadRng,
    started: Option<Instant>,
    wpm: Option<f64>
}

impl WordsPerMinute {
    fn new() -> Self {
        let s = include_str!("words.txt");
        let words = s.split("\n").map(|s| String::from(s.trim())).collect::<Vec<String>>();
        Self {
            words,
            current: vec![],
            current_ind: 0,
            current_word: String::from(""),
            rng: rand::thread_rng(),
            started: None,
            wpm: None,
        }
    }

    fn generate(&mut self) {
        let set = {
            let mut clone = self.words.clone();
            clone.shuffle(&mut self.rng);
            clone
        };
        self.current = set;
    }

    fn intake(&mut self) {
        if let Some(check) = self.current.get(self.current_ind) {
            if self.current_word.trim().eq_ignore_ascii_case(check) {
                self.current_ind += 1;
            }
        }
    }
}

impl eframe::App for WordsPerMinute {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("my_grid")
                .num_columns(1)
                .spacing([40.0, 40.0])
                .striped(true)
                .show(ui, |ui| {

                    ui.vertical_centered(|ui| {
                        if ui.button("Start").clicked() {
                            self.generate();
                            self.current_ind = 0;
                            self.started = Some(Instant::now());
                        }
                    });

                    ui.end_row();

                    ui.style_mut().spacing.interact_size.y = 0.0;
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 5.0;
                        if !self.current.is_empty() {
                            for (ind, word) in self.current.iter().enumerate() {
                                let mut text = RichText::new(word);
                                if ind < self.current_ind {
                                    text = text.weak();
                                } else if ind == self.current_ind {
                                    text = text.underline();
                                }
                                ui.label(text);
                            }
                        }
                    });

                    ui.end_row();

                    ui.add(egui::TextEdit::singleline(&mut self.current_word));
                    if ui.input().key_pressed(egui::Key::Space) && self.started.is_some() {
                        self.intake();
                        self.current_word.delete_char_range(0..self.current_word.len());
                        self.current_word.insert_text("", 0);

                        if self.current_ind >= self.current.len() {
                            let mins_elapsed = self.started.unwrap().elapsed().as_secs_f64() / 60.0;
                            self.wpm = Some(self.current.len() as f64 / mins_elapsed);
                            self.started = None;
                        }
                    }

                    ui.end_row();

                    ui.label(format!("WPM: {:.2}", self.wpm.unwrap_or(0.0)));

                });
        });
        ctx.request_repaint();
    }
}
