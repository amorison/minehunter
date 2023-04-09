use std::collections::BTreeSet;

use rand::seq::IteratorRandom;

#[derive(Copy, Clone, Debug)]
pub enum Cell {
    Clear,
    Neighbouring(u8),
    Mine,
}

#[derive(Default, Copy, Clone)]
pub struct Shape {
    pub nrows: usize,
    pub ncols: usize,
}

impl Shape {
    pub fn ncells(&self) -> usize {
        self.nrows * self.ncols
    }

    fn idx(&self, irow: usize, icol: usize) -> usize {
        assert!(irow < self.nrows);
        assert!(icol < self.ncols);
        irow * self.ncols + icol
    }

    pub fn cells(&self) -> impl Iterator<Item = (usize, usize)> {
        let nrows = self.nrows;
        let ncols = self.ncols;
        (0..nrows)
            .map(move |ir| (0..ncols).map(move |ic| (ir, ic)))
            .flatten()
    }

    pub fn neighbours(&self, irow: usize, icol: usize) -> impl Iterator<Item = (usize, usize)> {
        let row_nbs = irow.saturating_sub(1)..=(irow + 1).min(self.nrows - 1);
        let col_nbs = icol.saturating_sub(1)..=(icol + 1).min(self.ncols - 1);
        row_nbs
            .map(move |ir| col_nbs.clone().map(move |ic| (ir, ic)))
            .flatten()
    }
}

#[derive(Default)]
pub struct MineField {
    shape: Shape,
    cells: Vec<Cell>,
    n_mines: usize,
}

impl MineField {
    pub fn new<T>(nrows: usize, ncols: usize, mines: T) -> Self
    where
        T: IntoIterator<Item = (usize, usize)>,
    {
        let shape = Shape { nrows, ncols };
        let mines: BTreeSet<_> = mines.into_iter().collect();
        let n_mines = mines.len();
        let mut cells = Vec::with_capacity(shape.ncells());
        for (irow, icol) in shape.cells() {
            if mines.contains(&(irow, icol)) {
                cells.push(Cell::Mine);
            } else {
                let n_nb = shape
                    .neighbours(irow, icol)
                    .filter(|nb| mines.contains(nb))
                    .count();
                cells.push(if n_nb == 0 {
                    Cell::Clear
                } else {
                    Cell::Neighbouring(n_nb as u8)
                });
            }
        }
        MineField {
            shape,
            cells,
            n_mines,
        }
    }

    pub fn with_rand_mines(nrows: usize, ncols: usize, nmines: usize) -> Self {
        let mut rng = ::rand::thread_rng();
        let mines = ::rand::seq::index::sample(&mut rng, nrows * ncols, nmines);
        let mines = mines.iter().map(|i| (i / ncols, i % ncols));
        Self::new(nrows, ncols, mines)
    }

    pub fn with_rand_mines_avoiding(
        nrows: usize,
        ncols: usize,
        nmines: usize,
        irow: usize,
        icol: usize,
    ) -> Self {
        let shape = Shape { nrows, ncols };
        let mut cells: BTreeSet<_> = shape.cells().collect();
        for nb in shape.neighbours(irow, icol) {
            cells.remove(&nb);
        }
        let mut rng = ::rand::thread_rng();
        let mines = cells.into_iter().choose_multiple(&mut rng, nmines);
        MineField::new(nrows, ncols, mines)
    }

    pub fn get(&self, irow: usize, icol: usize) -> Cell {
        let icell = self.shape.idx(irow, icol);
        self.cells[icell]
    }

    pub fn nrows(&self) -> usize {
        self.shape.nrows
    }

    pub fn ncols(&self) -> usize {
        self.shape.ncols
    }
}

#[derive(Copy, Clone)]
pub enum CellState {
    Hidden,
    Flagged,
    Visible(Cell),
}

pub enum Outcome {
    Won,
    Lost,
    Ongoing,
}

#[derive(Default)]
pub struct Board {
    field: MineField,
    state: Vec<CellState>,
}

impl Board {
    pub fn new(field: MineField) -> Self {
        let state = vec![CellState::Hidden; field.shape.ncells()];
        Self { field, state }
    }

    pub fn nmines(&self) -> usize {
        self.field.n_mines
    }

    pub fn reveal(&mut self, irow: usize, icol: usize) -> Cell {
        if let CellState::Visible(cell) = self.get(irow, icol) {
            cell
        } else {
            let shape = &self.field.shape;
            let icell = shape.idx(irow, icol);
            let cell = self.field.get(irow, icol);
            self.state[icell] = CellState::Visible(cell);
            if matches!(cell, Cell::Clear) {
                for (ir, ic) in shape.neighbours(irow, icol) {
                    self.reveal(ir, ic);
                }
            }
            cell
        }
    }

    pub fn toggle_flag(&mut self, irow: usize, icol: usize) {
        let icell = self.field.shape.idx(irow, icol);
        self.state[icell] = match self.state[icell] {
            CellState::Hidden => CellState::Flagged,
            CellState::Flagged => CellState::Hidden,
            other => other,
        };
    }

    pub fn get(&self, irow: usize, icol: usize) -> CellState {
        let icell = self.field.shape.idx(irow, icol);
        self.state[icell]
    }

    pub fn shape(&self) -> &Shape {
        &self.field.shape
    }

    pub fn outcome(&self) -> Outcome {
        let n_mines = self.field.n_mines;
        let mut n_hidden = 0;
        for (ir, ic) in self.shape().cells() {
            match self.get(ir, ic) {
                CellState::Hidden | CellState::Flagged => n_hidden += 1,
                CellState::Visible(Cell::Mine) => return Outcome::Lost,
                CellState::Visible(_) => {}
            }
        }
        if n_hidden == n_mines {
            Outcome::Won
        } else {
            Outcome::Ongoing
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_minefield() {
        let mf = MineField::new(3, 4, [(1, 2), (0, 0)]);
        assert_eq!(mf.nrows(), 3);
        assert_eq!(mf.ncols(), 4);
        assert!(matches!(mf.get(1, 2), Cell::Mine));
        assert!(matches!(mf.get(0, 0), Cell::Mine));
        assert!(matches!(mf.get(2, 1), Cell::Neighbouring(1)));
        assert!(matches!(mf.get(0, 1), Cell::Neighbouring(2)));
        assert!(matches!(mf.get(1, 1), Cell::Neighbouring(2)));
        assert!(matches!(mf.get(2, 0), Cell::Clear));
        assert!(matches!(mf.get(1, 3), Cell::Neighbouring(1)));
    }

    #[test]
    fn rand_new_minefield() {
        let mf = MineField::with_rand_mines(15, 4, 8);
        let nmines: usize = (0..15)
            .map(|irow| {
                (0..4)
                    .filter(|&icol| matches!(mf.get(irow, icol), Cell::Mine))
                    .count()
            })
            .sum();
        assert_eq!(mf.nrows(), 15);
        assert_eq!(mf.ncols(), 4);
        assert_eq!(nmines, 8);
    }

    #[test]
    fn rand_new_minefield_avoiding() {
        let mf = MineField::with_rand_mines_avoiding(3, 3, 5, 0, 0);
        assert!(matches!(mf.get(0, 0), Cell::Clear));
        assert!(matches!(mf.get(0, 1), Cell::Neighbouring(2)));
        assert!(matches!(mf.get(1, 1), Cell::Neighbouring(5)));
    }

    #[test]
    fn board_reveal() {
        let mut board = Board::new(MineField::new(5, 5, [(2, 2)]));
        board.reveal(1, 1);
        assert!(matches!(board.get(0, 0), CellState::Hidden));
        assert!(matches!(
            board.get(1, 1),
            CellState::Visible(Cell::Neighbouring(1))
        ));
        board.reveal(4, 4);
        assert!(matches!(board.get(0, 0), CellState::Visible(Cell::Clear)));
    }
}
