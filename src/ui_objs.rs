use eframe::{
    egui::{self, Response},
    epaint::{self, CircleShape, Color32, FontId, RectShape, Vec2},
};

use crate::engine::{Cell, CellState};

pub(crate) struct CellButton {
    cell: CellState,
    scaling: f32,
}

impl CellButton {
    pub(crate) fn new(cell: CellState, scaling: f32) -> Self {
        Self { cell, scaling }
    }

    pub(crate) fn base_size(ui: &egui::Ui) -> f32 {
        ui.spacing().interact_size.y * 2.0
    }

    fn fill_color(&self, response: &Response) -> Color32 {
        let hvrd = response.hovered() || response.has_focus();
        match self.cell {
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
}

impl egui::Widget for CellButton {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(
            Vec2::splat(Self::base_size(ui) * self.scaling),
            egui::Sense::click_and_drag(),
        );
        if ui.is_rect_visible(rect) {
            let color = self.fill_color(&response);
            let shape: epaint::Shape = match self.cell {
                CellState::Flagged | CellState::Visible(Cell::Mine) => {
                    CircleShape::filled(rect.center(), 0.5 * rect.height(), color).into()
                }
                _ => RectShape::filled(rect, 0.0, color).into(),
            };
            let painter = ui.painter();
            painter.add(shape);
            if let CellState::Visible(Cell::Neighbouring(i)) = self.cell {
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    i.to_string(),
                    FontId {
                        size: 14.0 * self.scaling,
                        family: epaint::FontFamily::Proportional,
                    },
                    Color32::from_gray(180),
                );
            }
        }
        response
    }
}
