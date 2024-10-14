mod grid;
mod player;
mod state;
mod bootstrap_window;

use crate::bootstrap_window::MainWindow;
use eframe::egui::{CtxRef, Label, TopBottomPanel, Ui, Vec2};
use eframe::epi::{Frame, Storage};
use eframe::{epi::App, run_native, NativeOptions};

impl App for MainWindow {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        self.render_bootstrap(ctx, frame);

    }
    fn setup(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        self.configure_fonts(ctx);
    }

    fn name(&self) -> &str {
        "Bootstrap Window"
    }
}

fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.add(Label::new("v 0.1").monospace());
        });
    });
}

fn render_header(_ui: &mut Ui) {}

fn main() {
    let window = MainWindow::new();
    let mut option: NativeOptions = NativeOptions::default();
    option.initial_window_size = Some(Vec2::new(1500., 1350.));
    option.resizable = false;
    run_native(Box::new(window), option);
}
