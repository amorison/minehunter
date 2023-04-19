use eframe::{
    egui::{self, Response},
    epaint::{self, CircleShape, Color32, FontId, RectShape, Vec2},
};

use crate::engine::{Cell, CellState};

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum ColorTheme {
    Blue,
    Green,
    Pink,
}

struct Colors {
    main: Color32,
    highlighted: Color32,
}

impl ColorTheme {
    pub(crate) fn main_color(&self) -> Color32 {
        self.colors().main
    }

    fn colors(&self) -> Colors {
        match self {
            ColorTheme::Blue => Colors {
                main: Color32::from_rgb(0, 92, 128),
                highlighted: Color32::from_rgb(0, 115, 160),
            },
            ColorTheme::Green => Colors {
                main: Color32::from_rgb(0, 128, 92),
                highlighted: Color32::from_rgb(0, 160, 115),
            },
            ColorTheme::Pink => Colors {
                main: Color32::from_rgb(255, 128, 191),
                highlighted: Color32::from_rgb(255, 179, 217),
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

struct ThemeOption {
    selected: bool,
    theme: ColorTheme,
}

impl egui::Widget for ThemeOption {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let size = ui.spacing().interact_size.y * 2.0;
        let (rect, response) =
            ui.allocate_exact_size(Vec2::splat(size), egui::Sense::click_and_drag());
        ui.painter()
            .circle_filled(rect.center(), size / 3.0, self.theme.on_response(&response));
        if self.selected {
            ui.painter()
                .circle_stroke(rect.center(), size / 2.0, (4.0, Color32::from_gray(70)));
        }

        response
    }
}

pub(crate) fn theme_picker(theme: &mut ColorTheme, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        for ct in [ColorTheme::Blue, ColorTheme::Green, ColorTheme::Pink] {
            if ui
                .add(ThemeOption {
                    selected: ct == *theme,
                    theme: ct,
                })
                .clicked()
            {
                *theme = ct;
            };
        }
    });
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
