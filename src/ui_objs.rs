use eframe::{
    egui,
    epaint::{self, CircleShape, Color32, FontId, RectShape},
};

use crate::engine::{Cell, CellState};

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
        let rounding = if response.hovered() || response.has_focus() {
            0.2
        } else {
            0.05
        } * rect.height();
        let shape: epaint::Shape = match cell {
            CellState::Hidden => RectShape::filled(rect, rounding, visuals.bg_fill).into(),
            CellState::Flagged => {
                let radius = if response.hovered() || response.has_focus() {
                    0.45
                } else {
                    0.5
                } * rect.height();
                CircleShape::filled(rect.center(), radius, visuals.bg_fill).into()
            }
            CellState::Visible(Cell::Clear) => {
                RectShape::filled(rect, rounding, Color32::TRANSPARENT).into()
            }
            CellState::Visible(Cell::Mine) => {
                CircleShape::filled(rect.center(), 0.5 * rect.height(), Color32::DARK_RED).into()
            }
            CellState::Visible(Cell::Neighbouring(_)) => {
                RectShape::filled(rect, rounding, visuals.bg_fill).into()
            }
        };
        let painter = ui.painter();
        painter.add(shape);
        if let CellState::Visible(Cell::Neighbouring(i)) = cell {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                i.to_string(),
                FontId::default(),
                visuals.text_color(),
            );
        }
    }
    response
}

pub(crate) fn cell_btn(cell: CellState) -> impl egui::Widget {
    move |ui: &mut egui::Ui| cell_btn_ui(ui, cell)
}
