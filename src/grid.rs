use crate::player::Player;
use crate::state::PlayerState;
use crate::{render_footer, render_header};
use eframe::egui::TextStyle::Heading;
use eframe::egui::{Button, CentralPanel, Context, Ui, Vec2};
use eframe::Frame;
use std::vec::Vec;

#[derive(Clone)]
struct Block {
    data: String,
}

impl Block {
    fn set_data(&mut self, data: String) {
        self.data = data;
    }
}

#[derive(Default)]
pub struct Grid {
    blocks: Vec<Vec<Box<Block>>>,
    player: Player,
    win: bool,
    pub size: usize,
}

impl Grid {
    pub fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            self.render_step(ui);
            self.render_blocks(ui, ctx);
            render_footer(ctx);
        });
    }
    pub fn name(&self) -> &str {
        "CROSS & CIRCLE"
    }

    pub fn render_blocks(&mut self, ui: &mut Ui, ctx: &Context) {
        let mut i = 0;
        let mut j = 0;
        for horizontal_block in &mut self.blocks {
            ui.horizontal(|h_ui| {
                for vertical_block in horizontal_block {
                    h_ui.vertical(|v_ui| {
                        let button = v_ui.add_sized([100., 100.],
                                                    Button::new(&vertical_block.data));
                        if button.clicked() && !self.win {
                            if vertical_block.data == " " {
                                match self.player.current_state {
                                    PlayerState::Cross => {
                                        vertical_block.set_data("X".to_string());
                                    }
                                    PlayerState::Circle => {
                                        vertical_block.set_data("O".to_string());
                                    }
                                }
                                self.player.change_state();
                            }
                        }
                    });
                    j += 1;
                }
            });
            i += 1;
        }
    }

    pub fn render_step(&mut self, main_ui: &mut Ui) {
        main_ui.label(match &self.check_winner() {
            Some(_) => format!("Winner {}", match &self.player.current_state {
                PlayerState::Circle => "Cross",
                PlayerState::Cross => "Circle",
            }),
            None => match &self.player.current_state {
                PlayerState::Circle => "Step Circle",
                PlayerState::Cross => "Step Cross",
            }.to_string(),
        });
    }

    pub fn new(size: usize) -> Grid {
        Grid {
            blocks: vec![vec![Box::new(Block { data: " ".to_string() }); size]; size],
            player: Player { current_state: PlayerState::Cross },
            win: false,
            size,
        }
    }

    pub fn create_new_grid(&mut self) {
        self.blocks = vec![vec![Box::new(Block { data: " ".to_string() }); self.size]; self.size];
    }

    fn check_winner(&mut self) -> Option<String> {
        for x in 0..self.blocks.len() {
            if self.blocks[x][0].data == " " {
                continue;
            }
            let mut win = true;
            for y in 1..self.blocks[x].len() {
                if self.blocks[x][0].data != self.blocks[x][y].data {
                    win = false;
                    break;
                }
            }
            if win == true {
                self.win = true;
                return Some(format!("Winner {}", self.blocks[x][0].data));
            }
        }
        for x in 0..self.blocks.len() {
            if self.blocks[0][x].data == " " {
                continue;
            }
            let mut win = true;
            for y in 1..self.blocks[x].len() {
                if self.blocks[0][x].data != self.blocks[y][x].data {
                    win = false;
                    break;
                }
            }
            if win == true {
                self.win = true;
                return Some(format!("Winner {}", self.blocks[0][x].data));
            }
        }
        let mut win = true;
        for x in 0..self.blocks.len() {
            if (self.blocks[0][0].data != self.blocks[x][x].data) || self.blocks[x][x].data == " " {
                win = false;
                break;
            }
        }
        if win == true {
            self.win = true;
            return Some(format!("Winner {}", self.blocks[0][0].data));
        }
        win = true;
        for x in (0..self.blocks.len()).rev() {
            if (self.blocks[0][self.blocks.len() - 1].data != self.blocks[x][(x as i32 - (self.blocks.len() - 1) as i32).abs() as usize].data) ||
                self.blocks[x][((x as i32) - (self.blocks.len() - 1) as i32).abs() as usize].data == " "
            {
                win = false;
                break;
            }
        }
        if win == true {
            self.win = true;
            return Some(format!("Winner {}", self.blocks[0][self.blocks.len() - 1].data));
        }
        None
    }
}
