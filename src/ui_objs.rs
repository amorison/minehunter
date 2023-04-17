use eframe::{
    egui::{self, Response},
    epaint::{self, CircleShape, Color32, FontId, RectShape, Vec2},
};

use crate::engine::{Cell, CellState};

pub(crate) enum ColorTheme {
    Blue,
}

struct Colors {
    main: Color32,
    highlighted: Color32,
}

impl ColorTheme {
    fn colors(&self) -> Colors {
        match self {
            ColorTheme::Blue => Colors {
                main: Color32::from_rgb(0, 92, 128),
                highlighted: Color32::from_rgb(0, 115, 160),
            },
        }
    }

    fn on_response(&self, response: &Response) -> Color32 {
        let colors = self.colors();
        if response.hovered() || response.has_focus() {
            colors.highlighted
        } else {
            colors.main
        }
    }
}

pub(crate) struct CellButton {
    cell: CellState,
    scaling: f32,
    theme: ColorTheme,
}

impl CellButton {
    pub(crate) fn new(cell: CellState, scaling: f32, theme: ColorTheme) -> Self {
        Self {
            cell,
            scaling,
            theme,
        }
    }

    pub(crate) fn base_size(ui: &egui::Ui) -> f32 {
        ui.spacing().interact_size.y * 2.0
    }

    fn fill_color(&self, response: &Response) -> Color32 {
        match self.cell {
            CellState::Hidden | CellState::Flagged => self.theme.on_response(response),
            CellState::Visible(Cell::Mine) => Color32::DARK_RED,
            CellState::Visible(Cell::Clear) => Color32::TRANSPARENT,
            CellState::Visible(Cell::Neighbouring(_)) => Color32::from_gray(35),
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
                        size: 18.0 * self.scaling,
                        family: epaint::FontFamily::Proportional,
                    },
                    self.theme.on_response(&response),
                );
            }
        }
        response
    }
}
