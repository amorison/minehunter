mod engine;

use eframe::egui;
use engine::{Board, Cell, MineField, Shape};

pub enum MineHunterApp {
    WaitingBoard(Shape),
    InitializedBoard(Board),
}

impl MineHunterApp {
    fn shape(&self) -> &Shape {
        match self {
            Self::WaitingBoard(shape) => shape,
            Self::InitializedBoard(board) => board.shape(),
        }
    }

    fn get(&self, irow: usize, icol: usize) -> Option<Cell> {
        match self {
            Self::WaitingBoard(_) => None,
            Self::InitializedBoard(board) => board.get(irow, icol),
        }
    }

    fn reveal(&mut self, irow: usize, icol: usize) {
        match self {
            Self::WaitingBoard(shape) => {
                let mut board = Board::new(MineField::with_rand_mines_avoiding(
                    shape.nrows,
                    shape.ncols,
                    40,
                    irow,
                    icol,
                ));
                board.reveal(irow, icol);
                *self = Self::InitializedBoard(board);
            }
            Self::InitializedBoard(board) => {
                board.reveal(irow, icol);
            }
        }
    }
}

impl MineHunterApp {
    pub fn new(_cc: &::eframe::CreationContext<'_>) -> Self {
        Self::WaitingBoard(Shape {
            nrows: 16,
            ncols: 16,
        })
    }
}

impl ::eframe::App for MineHunterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut ::eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let nrows = self.shape().nrows;
            let ncols = self.shape().ncols;
            egui::Grid::new(0).show(ui, |ui| {
                for irow in 0..nrows {
                    for icol in 0..ncols {
                        let cell = self.get(irow, icol);
                        match cell {
                            None => {
                                if ui.button("hidden").clicked() {
                                    self.reveal(irow, icol);
                                }
                            }
                            Some(Cell::Clear) => {
                                ui.label("clear");
                            }
                            Some(Cell::Mine) => {
                                ui.label("bomb");
                            }
                            Some(Cell::Neighbouring(i)) => {
                                ui.label(format!("{i}"));
                            }
                        }
                    }
                    ui.end_row();
                }
            });
        });
    }
}
