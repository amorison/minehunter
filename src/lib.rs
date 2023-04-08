mod engine;

use eframe::{egui, epaint::FontId};
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

fn cell_btn_ui(ui: &mut egui::Ui, cell: Option<Cell>) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 2.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());
    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact_selectable(&response, cell.is_none());
        let label = match cell {
            None => String::new(),
            Some(Cell::Clear) => String::new(),
            Some(Cell::Mine) => "B!".to_owned(),
            Some(Cell::Neighbouring(i)) => format!("{i}"),
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

fn cell_btn(cell: Option<Cell>) -> impl egui::Widget {
    move |ui: &mut egui::Ui| cell_btn_ui(ui, cell)
}

impl ::eframe::App for MineHunterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut ::eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(1.0, 4.0);
            let nrows = self.shape().nrows;
            let ncols = self.shape().ncols;
            egui::Grid::new(0).show(ui, |ui| {
                for irow in 0..nrows {
                    for icol in 0..ncols {
                        let cell = self.get(irow, icol);
                        let response = ui.add(cell_btn(cell));
                        if cell.is_none()
                            && (response.clicked()
                                || (response.drag_released_by(egui::PointerButton::Primary)
                                    && response.hovered()))
                        {
                            self.reveal(irow, icol);
                        };
                    }
                    ui.end_row();
                }
            });
        });
    }
}
