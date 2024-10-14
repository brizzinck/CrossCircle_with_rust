mod grid;
mod player;
mod state;
mod bootstrap_window;

use crate::bootstrap_window::MainWindow;
use eframe::egui::{Context, Label, TopBottomPanel, Ui};
use eframe::{run_native, App, Frame, NativeOptions};

impl App for MainWindow {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.render_bootstrap(ctx, frame);
    }
}

fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.add(Label::new("v0.1.0"));
        });
    });
}

fn render_header(_ui: &mut Ui) {}

fn main() -> eframe::Result {
    env_logger::init();
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1500., 1500.])
            .with_resizable(false),
        ..Default::default()
    };
    run_native(
        "Cross & Circle",
        options,
        Box::new(|cc| Ok(Box::new(MainWindow::new(cc)))),
    )
}
