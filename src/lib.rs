mod engine;

use std::mem;

use eframe::{egui, epaint::FontId};
use engine::{Board, Cell, CellState, MineField, Outcome, Shape};

pub enum MineHunterApp {
    WaitingBoard(Shape, usize),
    InitializedBoard(Board),
    WonBoard(Board),
    LostBoard(Board),
}

impl MineHunterApp {
    fn shape(&self) -> &Shape {
        match self {
            Self::WaitingBoard(shape, _) => shape,
            Self::InitializedBoard(board) => board.shape(),
            Self::WonBoard(board) => board.shape(),
            Self::LostBoard(board) => board.shape(),
        }
    }

    fn nmines(&self) -> usize {
        match self {
            Self::WaitingBoard(_, nmines) => *nmines,
            Self::InitializedBoard(b) | Self::WonBoard(b) | Self::LostBoard(b) => b.nmines(),
        }
    }

    fn get(&self, irow: usize, icol: usize) -> CellState {
        match self {
            Self::WaitingBoard(..) => CellState::Hidden,
            Self::InitializedBoard(board) => board.get(irow, icol),
            Self::WonBoard(board) => board.get(irow, icol),
            Self::LostBoard(board) => board.get(irow, icol),
        }
    }

    fn reveal(&mut self, irow: usize, icol: usize) {
        match self {
            Self::WaitingBoard(shape, nmines) => {
                let mut board = Board::new(MineField::with_rand_mines_avoiding(
                    shape.nrows,
                    shape.ncols,
                    *nmines,
                    irow,
                    icol,
                ));
                board.reveal(irow, icol);
                *self = Self::InitializedBoard(board);
            }
            Self::InitializedBoard(board) => {
                board.reveal(irow, icol);
            }
            Self::WonBoard(_) | Self::LostBoard(_) => {}
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
        if let Self::InitializedBoard(board) = self {
            board.toggle_flag(irow, icol);
        }
    }

    fn update_win_lost(&mut self) {
        if let Self::InitializedBoard(board) = self {
            match board.outcome() {
                Outcome::Won => {
                    *self = Self::WonBoard(mem::take(board));
                }
                Outcome::Lost => {
                    *self = Self::LostBoard(mem::take(board));
                }
                Outcome::Ongoing => {}
            }
        }
    }
}

impl MineHunterApp {
    pub fn new(_cc: &::eframe::CreationContext<'_>) -> Self {
        Self::WaitingBoard(
            Shape {
                nrows: 16,
                ncols: 16,
            },
            40,
        )
    }
}

fn cell_selectable(cell: CellState) -> bool {
    match cell {
        CellState::Hidden | CellState::Flagged => true,
        CellState::Visible(_) => false,
    }
}

fn cell_btn_ui(ui: &mut egui::Ui, cell: CellState) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 2.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());
    if ui.is_rect_visible(rect) {
        let visuals = ui
            .style()
            .interact_selectable(&response, cell_selectable(cell));
        let label = match cell {
            CellState::Hidden => String::new(),
            CellState::Flagged => "F".to_owned(),
            CellState::Visible(Cell::Clear) => String::new(),
            CellState::Visible(Cell::Mine) => "B!".to_owned(),
            CellState::Visible(Cell::Neighbouring(i)) => format!("{i}"),
        };
        let rect = rect.expand(visuals.expansion);
        let painter = ui.painter();
        painter.rect(
            rect,
            0.05 * rect.height(),
            visuals.bg_fill,
            visuals.fg_stroke,
        );
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            FontId::default(),
            visuals.text_color(),
        );
    }
    response
}

fn cell_btn(cell: CellState) -> impl egui::Widget {
    move |ui: &mut egui::Ui| cell_btn_ui(ui, cell)
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
        egui::SidePanel::left("ctrl_panel").show(ctx, |ui| {
            ui.add_space(15.0);
            let shape = self.shape();
            let mut nrows = shape.nrows;
            let mut ncols = shape.ncols;
            let mut nmines = self.nmines();
            let nmines_min = shape.ncells() / 10;
            let nmines_max = 2 * shape.ncells() / 5;
            match self {
                Self::WonBoard(_) | Self::LostBoard(_) | Self::WaitingBoard(..) => {
                    ui.add(egui::Slider::new(&mut nrows, 8..=30).text("Rows"));
                    ui.add(egui::Slider::new(&mut ncols, 8..=50).text("Cols"));
                    ui.add(egui::Slider::new(&mut nmines, nmines_min..=nmines_max).text("Mines"));
                    ui.label(format!("{nmines} mines"));
                    if nrows != shape.nrows || ncols != shape.ncols {
                        nmines = nrows * ncols / 5;
                    }
                    if nrows != shape.nrows || ncols != shape.ncols || nmines != self.nmines() {
                        *self = Self::WaitingBoard(Shape { nrows, ncols }, nmines);
                    }
                }
                Self::InitializedBoard(_) => {
                    ui.label(format!("{nrows} x {ncols}"));
                    ui.label(format!("Mines: {}", self.nmines()));
                }
            }
            ui.add_space(15.0);

            match self {
                Self::WonBoard(_) => {
                    ui.label("Congratulations!");
                }
                Self::LostBoard(_) => {
                    ui.label("You lost...");
                }
                Self::WaitingBoard(..) => {
                    ui.label("Pick a cell");
                }
                Self::InitializedBoard(_) => {}
            }

            ui.add_space(15.0);
            if ui.button("Restart").clicked() {
                *self = Self::WaitingBoard(*self.shape(), self.nmines());
            }
            let presets = [(8, 8, 10), (16, 16, 40), (16, 32, 100)];
            for (nrows, ncols, nmines) in presets {
                if ui
                    .button(format!("{nrows}x{ncols}, {nmines} mines"))
                    .clicked()
                {
                    *self = Self::WaitingBoard(Shape { nrows, ncols }, nmines);
                }
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(1.0, 4.0);
            let nrows = self.shape().nrows;
            let ncols = self.shape().ncols;
            egui::Grid::new(0).show(ui, |ui| {
                for irow in 0..nrows {
                    for icol in 0..ncols {
                        let cell = self.get(irow, icol);
                        let response = ui.add(cell_btn(cell));
                        match cell {
                            CellState::Hidden if response.lax_clicked() => {
                                self.reveal(irow, icol);
                            }
                            CellState::Hidden | CellState::Flagged if response.lax_r_clicked() => {
                                self.toggle_flag(irow, icol);
                            }
                            CellState::Visible(_) if response.lax_clicked() => {
                                self.reveal_around_nb(irow, icol);
                            }
                            _ => {}
                        }
                    }
                    ui.end_row();
                }
            });
        });
        self.update_win_lost();
    }
}
