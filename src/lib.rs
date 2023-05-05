mod engine;
mod ui_objs;

use std::{
    mem,
    time::{Duration, Instant},
};

use eframe::{
    egui::{self, Button, RichText},
    epaint::Vec2,
};

use engine::{Board, Cell, CellState, MineField, Outcome, Shape};
use ui_objs::{theme_picker, CellButton, ColorTheme};

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    let millis = duration.subsec_millis();
    if minutes > 0 {
        format!("{minutes}:{seconds:02}.{millis:03}")
    } else {
        format!("{seconds}.{millis:03}")
    }
}

enum BoardState {
    Waiting(Shape, usize),
    Initialized(Board, Instant),
    Won(Board, Duration),
    Lost(Board),
}

pub struct MineHunterApp {
    board: BoardState,
    theme: ColorTheme,
}

impl BoardState {
    fn shape(&self) -> &Shape {
        match self {
            Self::Waiting(shape, _) => shape,
            Self::Initialized(board, _) => board.shape(),
            Self::Won(board, _) => board.shape(),
            Self::Lost(board) => board.shape(),
        }
    }

    fn nmines(&self) -> usize {
        match self {
            Self::Waiting(_, nmines) => *nmines,
            Self::Initialized(b, _) | Self::Won(b, _) | Self::Lost(b) => b.nmines(),
        }
    }

    fn get(&self, irow: usize, icol: usize) -> CellState {
        match self {
            Self::Waiting(..) => CellState::Hidden,
            Self::Initialized(board, _) => board.get(irow, icol),
            Self::Won(board, _) => board.get(irow, icol),
            Self::Lost(board) => board.get(irow, icol),
        }
    }

    fn reveal(&mut self, irow: usize, icol: usize) {
        match self {
            Self::Waiting(shape, nmines) => {
                let mut board = Board::new(MineField::with_rand_mines_avoiding(
                    shape.nrows,
                    shape.ncols,
                    *nmines,
                    irow,
                    icol,
                ));
                board.reveal(irow, icol);
                *self = Self::Initialized(board, Instant::now());
            }
            Self::Initialized(board, _) => {
                board.reveal(irow, icol);
            }
            Self::Won(_, _) | Self::Lost(_) => {}
        }
    }

    fn reveal_around_nb(&mut self, irow: usize, icol: usize) {
        if let CellState::Visible(Cell::Neighbouring(n_nb)) = self.get(irow, icol) {
            let n_flagged = self
                .shape()
                .neighbours(irow, icol)
                .filter(|&(ir, ic)| matches!(self.get(ir, ic), CellState::Flagged))
                .count();
            if n_flagged == n_nb.into() {
                for (ir, ic) in self.shape().neighbours(irow, icol) {
                    if matches!(self.get(ir, ic), CellState::Hidden) {
                        self.reveal(ir, ic);
                    }
                }
            }
        }
    }

    fn toggle_flag(&mut self, irow: usize, icol: usize) {
        if let Self::Initialized(board, _) = self {
            board.toggle_flag(irow, icol);
        }
    }

    fn update_win_lost(&mut self) {
        if let Self::Initialized(board, start_time) = self {
            match board.outcome() {
                Outcome::Won => {
                    for (ir, ic) in board.shape().cells() {
                        if let CellState::Hidden = board.get(ir, ic) {
                            board.toggle_flag(ir, ic);
                        }
                    }
                    *self = Self::Won(mem::take(board), Instant::now() - *start_time);
                }
                Outcome::Lost => {
                    *self = Self::Lost(mem::take(board));
                }
                Outcome::Ongoing => {}
            }
        }
    }
}

impl MineHunterApp {
    pub fn new(_cc: &::eframe::CreationContext<'_>) -> Self {
        Self {
            board: BoardState::Waiting(
                Shape {
                    nrows: 16,
                    ncols: 16,
                },
                40,
            ),
            theme: ColorTheme::Blue,
        }
    }
}

trait LaxClicked {
    fn lax_clicked(&self) -> bool;
    fn lax_r_clicked(&self) -> bool;
}

impl LaxClicked for egui::Response {
    fn lax_clicked(&self) -> bool {
        self.clicked() || (self.drag_released_by(egui::PointerButton::Primary) && self.hovered())
    }

    fn lax_r_clicked(&self) -> bool {
        self.secondary_clicked()
            || (self.drag_released_by(egui::PointerButton::Secondary) && self.hovered())
    }
}

impl ::eframe::App for MineHunterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut ::eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(111));
        egui::SidePanel::left("ctrl_panel").show(ctx, |ui| {
            ui.add_space(15.0);
            let shape = self.board.shape();
            let mut nrows = shape.nrows;
            let mut ncols = shape.ncols;
            let mut nmines = self.board.nmines();
            let nmines_min = shape.ncells() / 10;
            let nmines_max = 2 * shape.ncells() / 5;
            ui.add(egui::Slider::new(&mut nrows, 8..=30).text("Rows"));
            ui.add(egui::Slider::new(&mut ncols, 8..=50).text("Cols"));
            ui.add(egui::Slider::new(&mut nmines, nmines_min..=nmines_max).text("Mines"));

            ui.add_space(15.0);
            if !matches!(self.board, BoardState::Initialized(_, _)) {
                if nrows != shape.nrows || ncols != shape.ncols {
                    nmines = nrows * ncols / 5;
                }
                if nrows != shape.nrows || ncols != shape.ncols || nmines != self.board.nmines() {
                    self.board = BoardState::Waiting(Shape { nrows, ncols }, nmines);
                }
            }

            let msg: String = match &self.board {
                BoardState::Won(_, _) => "Congratulations!".to_owned(),
                BoardState::Lost(_) => "You lost...".to_owned(),
                BoardState::Waiting(..) => "Pick a cell".to_owned(),
                BoardState::Initialized(board, _) => {
                    format!("Flagged: {} / {}", board.nflagged(), board.nmines())
                }
            };
            let mut msg = RichText::new(msg).size(20.0);
            if matches!(self.board, BoardState::Won(_, _)) {
                msg = msg.color(self.theme.main_color());
            }
            ui.label(msg);

            ui.add_space(15.0);

            let btn_size = Vec2::splat(ui.available_width() / 2.5);
            egui::Grid::new(1).show(ui, |ui| {
                let btn = Button::new("Restart").min_size(btn_size);
                if ui.add(btn).clicked() {
                    self.board = BoardState::Waiting(*self.board.shape(), self.board.nmines());
                }
                let presets = [(8, 8, 10), (16, 16, 40), (16, 32, 100)];
                for (ip, (nrows, ncols, nmines)) in presets.into_iter().enumerate() {
                    let btn =
                        Button::new(format!("{nrows}x{ncols}\n{nmines} mines")).min_size(btn_size);
                    if ui.add(btn).clicked() {
                        self.board = BoardState::Waiting(Shape { nrows, ncols }, nmines);
                    }
                    if ip % 2 == 0 {
                        ui.end_row();
                    }
                }
            });

            ui.add_space(15.0);
            theme_picker(&mut self.theme, ui);

            ui.add_space(15.0);
            if let BoardState::Initialized(_, start_time) = self.board {
                let time = Instant::now() - start_time;
                let msg = RichText::new(format_duration(time)).size(20.0);
                ui.label(msg);
            } else if let BoardState::Won(_, time) = self.board {
                let msg = RichText::new(format_duration(time)).size(20.0);
                ui.label(msg);
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let nrows = self.board.shape().nrows;
            let ncols = self.board.shape().ncols;
            let Vec2 {
                x: width,
                y: height,
            } = ui.available_size();
            let max_btn_width = width / ncols as f32 - 2.0;
            let max_btn_height = height / nrows as f32 - 2.0;
            let btn_size = max_btn_width.min(max_btn_height);
            let scaling = (btn_size / CellButton::base_size(ui)).max(1.0);
            egui::Grid::new(0)
                .min_col_width(0.0)
                .min_row_height(0.0)
                .spacing((2.0, 2.0))
                .show(ui, |ui| {
                    for irow in 0..nrows {
                        for icol in 0..ncols {
                            let cell = self.board.get(irow, icol);
                            let response = ui.add(CellButton::new(cell, scaling, self.theme));
                            match cell {
                                CellState::Hidden if response.lax_clicked() => {
                                    self.board.reveal(irow, icol);
                                }
                                CellState::Hidden | CellState::Flagged
                                    if response.lax_r_clicked() =>
                                {
                                    self.board.toggle_flag(irow, icol);
                                }
                                CellState::Visible(_) if response.lax_clicked() => {
                                    self.board.reveal_around_nb(irow, icol);
                                }
                                _ => {}
                            }
                        }
                        ui.end_row();
                    }
                });
        });
        self.board.update_win_lost();
    }
}
