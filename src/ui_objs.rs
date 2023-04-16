use eframe::{
    egui::{self, Response},
    epaint::{self, CircleShape, Color32, FontId, RectShape},
};

use crate::engine::{Cell, CellState};

fn fill_color(cell: CellState, response: &Response) -> Color32 {
    let hvrd = response.hovered() || response.has_focus();
    match cell {
        CellState::Hidden | CellState::Flagged => {
            if hvrd {
                Color32::from_rgb(0, 115, 160)
            } else {
                Color32::from_rgb(0, 92, 128)
            }
        }
        CellState::Visible(Cell::Mine) => Color32::DARK_RED,
        CellState::Visible(Cell::Clear) => Color32::TRANSPARENT,
        CellState::Visible(Cell::Neighbouring(_)) => {
            if hvrd {
                Color32::from_gray(70)
            } else {
                Color32::from_gray(55)
            }
        }
    }
}

fn cell_btn_ui(ui: &mut egui::Ui, cell: CellState) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 2.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());
    if ui.is_rect_visible(rect) {
        let color = fill_color(cell, &response);
        let shape: epaint::Shape = match cell {
            CellState::Flagged | CellState::Visible(Cell::Mine) => {
                CircleShape::filled(rect.center(), 0.5 * rect.height(), color).into()
            }
            _ => RectShape::filled(rect, 0.0, color).into(),
        };
        let painter = ui.painter();
        painter.add(shape);
        if let CellState::Visible(Cell::Neighbouring(i)) = cell {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                i.to_string(),
                FontId::default(),
                Color32::from_gray(180),
            );
        }
    }
    response
}

pub(crate) fn cell_btn(cell: CellState) -> impl egui::Widget {
    move |ui: &mut egui::Ui| cell_btn_ui(ui, cell)
}
