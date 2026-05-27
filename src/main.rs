use::eframe::egui;
use std::{cmp::Ordering, vec::Vec};
use rand;

fn main() {
    let native_options = eframe::NativeOptions::default();

    let _app = eframe::run_native(
        "Guessing Game App",
        native_options,
        Box::new(|cc| {
            // Set style once here
            setup_fonts_and_style(&cc.egui_ctx);
            Ok(Box::new(GuessApp::new()))
        })
    );
}

#[derive(Clone)]
enum MyOrdering {
    NotEstablished,
    TooBig,
    TooSmall
}

struct GuessApp {
    answer: u32,
    guess: String,
    previous:  Vec<u32>,
    is_correct:  bool,
    relation: MyOrdering,
    low: u32,   // highest "too small" guess so far
    high: u32,  // lowest "too big" guess so far
}

impl GuessApp {
    pub fn new() -> Self {
        return GuessApp::default();
    }

    fn process_guess(&mut self) {
        let my_guess: u32 = match self.guess.trim().parse() {
            Ok(num) => {
                if num > 100 {
                    self.guess = "Over 100!".to_string();
                    return;
                }
                else if num == 0 {
                    self.guess = "Let's not include 0...".to_string();
                    return;
                }
                num
            },
            Err(_) => {
                self.guess = "Your Guess was invalid!".to_string();
                return;
            },
        };

        match my_guess.cmp(&self.answer) {
            Ordering::Equal => self.is_correct = true,
            Ordering::Greater => {
                self.relation = MyOrdering::TooBig;
                self.high = self.high.min(my_guess);
            },
            Ordering::Less => {
                self.relation = MyOrdering::TooSmall;
                self.low = self.low.max(my_guess);
            },
        }

        self.previous.insert(0, my_guess);
    }

    fn new_game(&mut self) {
        self.answer = rand::random_range(1..=100);
        self.guess = "N/A".to_owned();
        self.relation = MyOrdering::NotEstablished;
        self.is_correct = false;
        self.previous.clear();
        self.low = 1;
        self.high = 100;
    }
}

impl Default for GuessApp {
    fn default() -> Self {
        let secret_number = rand::random_range(1..=100);
        let my_guess = String::from("N/A");
        let previous: Vec<u32> = Vec::new();

        return Self {
            answer: secret_number,
            guess: my_guess,
            previous: previous,
            is_correct: false,
            relation: MyOrdering::NotEstablished,
            low: 1,
            high: 100,
        }
    }
}

impl eframe::App for GuessApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {

        // Top Panel
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            ui.heading("Guess The Number!");
            ui.label("From 1 to 100");
            ui.text_edit_singleline(&mut self.guess);
            
            if ui.button("Enter").clicked() && !self.is_correct {
                self.process_guess();
            }

        });

        // Right Panel (Too Lazy to make it as a function icl)
        egui::Panel::right("right_panel")
        .resizable(true)
        .default_size(200.0)
        .size_range(100.0..=400.0)
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Previous Guesses");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, num) in self.previous.iter().enumerate() {
                        let color = match num.cmp(&self.answer) {
                            Ordering::Less => egui::Color32::RED,
                            Ordering::Greater => egui::Color32::YELLOW,
                            Ordering::Equal => egui::Color32::GREEN,
                        };
                        let size = guess_font_size(i);
                        ui.label(egui::RichText::new(num.to_string()).color(color).size(size));
                    }
                });
            });
        });

        // Main Painel
        egui::CentralPanel::default().show_inside(ui, |ui| {
            
            // Game Loop of Guessing the Number
            if !self.is_correct {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("Your Guess:").size(32.0));
                    ui.label(egui::RichText::new(format!("{}", self.guess)).size(48.0).strong());
                    ui.heading(status(self.relation.clone()));
                });
                ui.separator();
                ui.add_space(100.0);
                let previous = if self.previous.len() != 0 { self.previous[0] } else { 0 };
                number_line(ui, previous, self.low.clone(), self.high.clone());
            } 
            
            // Finish State; Play again?
            else {

                let available = ui.available_size();
                // vertical centering via top padding
                let total_height = 48.0 + 16.0 + 50.0;
                ui.add_space((available.y - total_height) / 2.0);

                // You win Text and Play Again, and Buttons
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("You win!").size(48.0).strong());
                    ui.add_space(16.0);

                    // Button, and their accompanying formatting
                    ui.horizontal(|ui| {
                        let btn_width = 200.0 * 2.0 + 8.0;
                        ui.add_space((available.x - btn_width) / 2.0);

                        if ui.add(egui::Button::new(
                            egui::RichText::new("Play again?").size(24.0)
                        ).min_size(egui::vec2(200.0, 50.0))).clicked() {
                            self.new_game();
                        }

                        ui.add_space(8.0);

                        if ui.add(egui::Button::new(
                            egui::RichText::new("Exit").size(24.0)
                        ).min_size(egui::vec2(200.0, 50.0))).clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
            }
        });

    }
}


/// # Description
/// Turns ordering enum into a string.
/// # Arguments
/// * `status` - Custom Ordering Enum
fn status(status: MyOrdering) -> String {
    match status {
        MyOrdering::TooBig => return "Too Big".to_owned(),
        MyOrdering::TooSmall => return "Too Small".to_owned(),
        MyOrdering::NotEstablished => return "N/A".to_owned()
    }
}

/// # Description
/// Constructs the number line.
/// # Arguments
/// * `ui` - Egui ui Context
/// * `guess` - Input guess number
/// * `low` - lowest guessed number; paints number lower than the guess red
/// * `high` - highest guessed number; paints number line higher than the highest yellow
fn number_line(ui: &mut egui::Ui, guess: u32, low: u32, high: u32) {
    let desired_size = egui::vec2(ui.available_width(), 40.0);
    let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
    let painter = ui.painter();

    let to_x = |val: u32| rect.left() + (val as f32 - 1.0) / 99.0 * rect.width();

    let mid_y = rect.center().y;
    let stroke_w = 3.0;

    // Red segment: 1 to low
    if low > 1 {
        painter.line_segment(
            [egui::pos2(rect.left(), mid_y), egui::pos2(to_x(low), mid_y)],
            egui::Stroke::new(stroke_w, egui::Color32::RED),
        );
    }

    // Gray segment: low to high (still in play)
    painter.line_segment(
        [egui::pos2(to_x(low), mid_y), egui::pos2(to_x(high), mid_y)],
        egui::Stroke::new(stroke_w, egui::Color32::GRAY),
    );

    // Yellow segment: high to 100
    if high < 100 {
        painter.line_segment(
            [egui::pos2(to_x(high), mid_y), egui::pos2(rect.right(), mid_y)],
            egui::Stroke::new(stroke_w, egui::Color32::YELLOW),
        );
    }

    if guess != 0 {
        // Triangle + label at guess position
        let tick_x = to_x(guess);
        let tri_tip = egui::pos2(tick_x, mid_y - 12.0);
        let tri_left = egui::pos2(tick_x - 6.0, mid_y - 24.0);
        let tri_right = egui::pos2(tick_x + 6.0, mid_y - 24.0);

        painter.add(egui::Shape::convex_polygon(
            vec![tri_tip, tri_left, tri_right],
            egui::Color32::WHITE,
            egui::Stroke::NONE,
        ));

        painter.text(
            egui::pos2(tick_x, mid_y - 28.0),
            egui::Align2::CENTER_BOTTOM,
            guess.to_string(),
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );
    }
}

/// # Description
/// Dynamically changes size of Previous guess count based on index (From most recent guess to latest)
/// # Parameters
/// * `i` - usize index of number
fn guess_font_size(i: usize) -> f32 {
    match i {
        0 => 32.0,
        1 => 24.0,
        2 => 22.0,
        _ => 16.0,
    }
}

fn setup_fonts_and_style(ctx: &egui::Context) {
    // --- Fonts ---
    let mut fonts = egui::FontDefinitions::default();
    let mut visuals = egui::Visuals::dark(); // or ::light()

    fonts.font_data.insert(
        "comic_sans".to_owned(),
        egui::FontData::from_static(include_bytes!(".././assets/comic.ttf")).into(),
    );
    fonts.font_data.insert(
        "comic_sans_bold".to_owned(),
        egui::FontData::from_static(include_bytes!(".././assets/comicbd.ttf")).into(),
    );

    // Regular comic sans as default proportional font
    fonts.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "comic_sans".to_owned());

    // Bold variant under its own family name
    fonts.families.insert(
        egui::FontFamily::Name("comic_sans_bold".into()),
        vec!["comic_sans_bold".to_owned()],
    );

    ctx.set_fonts(fonts);
    visuals.override_text_color = Some(egui::Color32::WHITE);

    // --- Style ---
    let mut style: egui::Style = egui::Style::default();

    // Point Heading at the bold variant
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(24.0, egui::FontFamily::Name("comic_sans_bold".into())),
    );

    style.visuals = visuals;
    ctx.set_global_style(style);
}