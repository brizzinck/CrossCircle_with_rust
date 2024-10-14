use crate::grid::Grid;
use crate::state::GameState;
use crate::state::GameState::{Bootstrap, Game};
use eframe::egui::{Button, CentralPanel, Context, FontDefinitions, Slider};
use eframe::Frame;
use std::ops::RangeInclusive;
use egui::Vec2;

#[derive(Default)]
pub struct MainWindow {
    pub grid: Grid,
    pub game_state: GameState,
}

impl MainWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        MainWindow::setup_custom_fonts(&cc.egui_ctx);
        MainWindow { grid: Grid::new(3), game_state: Bootstrap }
    }

    pub fn render_bootstrap(&mut self, ctx: &Context, frame: &mut Frame) {
        match self.game_state {
            Bootstrap => {
                CentralPanel::default().show(ctx, |ui| {
                    let _slider = ui.add_sized
                    ([220., 50.],
                     Slider::new(&mut self.grid.size,
                                 RangeInclusive::new(3, 10))
                         .smart_aim(true).text("SIZE GRID"));
                    let start_button = ui.add_sized([100., 30.], Button::new("START"));
                    if start_button.clicked() {
                        self.game_state = Game;
                        self.grid.create_new_grid();
                    }
                });
            }
            Game => {
                ctx.send_viewport_cmd(egui::ViewportCommand::
                InnerSize(Vec2::new(self.grid.size as f32 * 100. + 9. * self.grid.size as f32,
                                    self.grid.size as f32 * 100. + 14. * self.grid.size as f32)));
                self.grid.update(ctx, frame);
            }
        }
    }
    fn setup_custom_fonts(ctx: &Context) {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "MesloLGS-Regular".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../MesloLGS_NF_Regular.ttf"
            )));

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "MesloLGS-Regular".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .push("MesloLGS-Regular".to_owned());
        ctx.set_fonts(fonts);
    }
}