use std::borrow::Cow;
use std::ops::RangeInclusive;
use eframe::egui::{Button, CentralPanel, CtxRef, FontDefinitions, FontFamily, Slider, Vec2};
use eframe::egui::TextStyle::{Body, Heading};
use eframe::epi::Frame;
use crate::grid::Grid;
use crate::state::GameState;
use crate::state::GameState::{Bootstrap, Game};

pub struct MainWindow {
    pub grid: Grid,
    pub game_state: GameState
}

impl MainWindow {
    pub fn new() -> MainWindow {
        MainWindow { grid: Grid::new(3), game_state: Bootstrap }
    }
    pub fn render_bootstrap(&mut self, ctx: &CtxRef, frame: &mut Frame) {
        match self.game_state {
            Bootstrap => {
                CentralPanel::default().show(ctx, |ui| {
                    let _slider = ui.add_sized([220., 50.], Slider::new(&mut self.grid.size, RangeInclusive::new(3, 10)).smart_aim(true).text("SIZE GRID"));
                    let start_button = ui.add_sized([100., 30.], Button::new("START"));
                    if start_button.clicked() {
                        self.game_state = Game;
                        self.grid.create_new_grid();
                    }
                });
            }
            Game => {
                self.grid.update(ctx, frame);
            }
        }
    }
    pub fn configure_fonts(&self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();

        font_def.font_data.insert(
            "MesloLGS-Regular".to_string(),
            Cow::Borrowed(include_bytes!("../MesloLGS_NF_Regular.ttf")),
        );

        font_def.family_and_size.insert(Heading, (FontFamily::Proportional, 55.));
        font_def.family_and_size.insert(Body, (FontFamily::Proportional, 35.));

        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS-Regular".to_string());

        ctx.set_fonts(font_def);
    }
}