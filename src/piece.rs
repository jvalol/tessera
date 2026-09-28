use glam::Vec4;

/// The twelve free pentominoes, named for the letters they look like.
///
/// Free means a shape and its mirror are the same piece. Six of the twelve are
/// chiral, F L N P Y Z, so taking their mirrors as well would give eighteen.
/// See spec 0008.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Shape {
    F,
    I,
    L,
    N,
    P,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

pub const SHAPES: [Shape; 12] = [
    Shape::F,
    Shape::I,
    Shape::L,
    Shape::N,
    Shape::P,
    Shape::T,
    Shape::U,
    Shape::V,
    Shape::W,
    Shape::X,
    Shape::Y,
    Shape::Z,
];

/// How many cells a piece has. A tessera is one of them, and the game is named
/// for it: a tessera is the single tile in a mosaic.
pub const CELLS: usize = 5;

impl Shape {
    /// Seven of these carry over from the tetrominoes, so a player who knew the
    /// old game reads the same colour for a similar letter. The other five sit
    /// in the gaps between them.
    pub fn color(self) -> Vec4 {
        let (r, g, b) = match self {
            Shape::F => (0.95, 0.35, 0.65),
            Shape::I => (0.0, 0.85, 0.9),
            Shape::L => (0.95, 0.55, 0.15),
            Shape::N => (0.25, 0.4, 0.9),
            Shape::P => (0.6, 0.85, 0.2),
            Shape::T => (0.7, 0.3, 0.85),
            Shape::U => (0.15, 0.7, 0.65),
            Shape::V => (0.9, 0.25, 0.25),
            Shape::W => (0.75, 0.5, 0.25),
            Shape::X => (0.9, 0.9, 0.95),
            Shape::Y => (0.95, 0.85, 0.1),
            Shape::Z => (0.3, 0.8, 0.3),
        };

        Vec4::new(r, g, b, 1.0)
    }

    /// The piece's five cells around its own origin, in its starting rotation.
    /// x grows right and y grows down, so a cell at y -1 sits above the origin.
    ///
    /// Each is centred on the middle of its bounding box, so it turns about
    /// roughly its own middle rather than swinging from a corner.
    pub fn cells(self) -> [(i32, i32); CELLS] {
        match self {
            Shape::F => [(0, -1), (1, -1), (-1, 0), (0, 0), (0, 1)],
            Shape::I => [(0, -2), (0, -1), (0, 0), (0, 1), (0, 2)],
            Shape::L => [(0, -1), (0, 0), (0, 1), (0, 2), (1, 2)],
            Shape::N => [(1, -1), (1, 0), (0, 1), (1, 1), (0, 2)],
            Shape::P => [(0, -1), (1, -1), (0, 0), (1, 0), (0, 1)],
            Shape::T => [(-1, -1), (0, -1), (1, -1), (0, 0), (0, 1)],
            Shape::U => [(-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)],
            Shape::V => [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)],
            Shape::W => [(-1, -1), (-1, 0), (0, 0), (0, 1), (1, 1)],
            Shape::X => [(0, -1), (-1, 0), (0, 0), (1, 0), (0, 1)],
            Shape::Y => [(1, -1), (0, 0), (1, 0), (1, 1), (1, 2)],
            Shape::Z => [(-1, -1), (0, -1), (0, 0), (0, 1), (1, 1)],
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub shape: Shape,
    /// Cells around the piece's origin, rotated as the piece has been.
    cells: [(i32, i32); CELLS],
    /// Where the origin sits on the board.
    pub position: (i32, i32),
}

impl Piece {
    pub fn new(shape: Shape) -> Piece {
        Piece {
            shape,
            cells: shape.cells(),
            position: (0, 0),
        }
    }

    pub fn at(shape: Shape, position: (i32, i32)) -> Piece {
        let mut piece = Piece::new(shape);
        piece.position = position;
        piece
    }

    pub fn color(&self) -> Vec4 {
        self.shape.color()
    }

    /// The cells the piece covers on the board.
    pub fn board_cells(&self) -> [(i32, i32); CELLS] {
        let mut cells = self.cells;
        for cell in cells.iter_mut() {
            *cell = (cell.0 + self.position.0, cell.1 + self.position.1);
        }
        cells
    }

    pub fn moved(&self, dx: i32, dy: i32) -> Piece {
        let mut piece = *self;
        piece.position = (self.position.0 + dx, self.position.1 + dy);
        piece
    }

    /// A quarter turn clockwise about the origin.
    ///
    /// X needs no special case the way O did: it is centred on its middle cell
    /// and a quarter turn maps it onto itself.
    pub fn rotated_cw(&self) -> Piece {
        self.rotated(|(x, y)| (-y, x))
    }

    pub fn rotated_ccw(&self) -> Piece {
        self.rotated(|(x, y)| (y, -x))
    }

    fn rotated(&self, turn: fn((i32, i32)) -> (i32, i32)) -> Piece {
        let mut piece = *self;
        for cell in piece.cells.iter_mut() {
            *cell = turn(*cell);
        }
        piece
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn sorted(mut cells: [(i32, i32); CELLS]) -> [(i32, i32); CELLS] {
        cells.sort_unstable();
        cells
    }

    /// A shape's cells with the corner brought to the origin, so two pieces can
    /// be compared whatever their position.
    fn normalized(cells: [(i32, i32); CELLS]) -> Vec<(i32, i32)> {
        let left = cells.iter().map(|c| c.0).min().expect("cells");
        let top = cells.iter().map(|c| c.1).min().expect("cells");
        let mut out: Vec<(i32, i32)> = cells.iter().map(|c| (c.0 - left, c.1 - top)).collect();
        out.sort_unstable();
        out
    }

    fn turns(shape: Shape) -> Vec<Vec<(i32, i32)>> {
        let mut piece = Piece::new(shape);
        let mut out = Vec::new();
        for _ in 0..4 {
            out.push(normalized(piece.board_cells()));
            piece = piece.rotated_cw();
        }
        out
    }

    #[test]
    fn every_piece_has_five_cells() {
        for shape in SHAPES {
            let cells = Piece::new(shape).board_cells();
            let unique: HashSet<(i32, i32)> = cells.iter().copied().collect();

            assert_eq!(unique.len(), CELLS, "{:?} has overlapping cells", shape);
        }
    }

    #[test]
    fn there_are_twelve_pieces() {
        assert_eq!(SHAPES.len(), 12);
    }

    #[test]
    fn no_two_pieces_are_the_same_shape() {
        for (i, shape) in SHAPES.iter().enumerate() {
            let mine: HashSet<Vec<(i32, i32)>> = turns(*shape).into_iter().collect();

            for other in SHAPES.iter().skip(i + 1) {
                let theirs = normalized(Piece::new(*other).board_cells());
                assert!(
                    !mine.contains(&theirs),
                    "{:?} is {:?} turned around",
                    other,
                    shape
                );
            }
        }
    }

    #[test]
    fn no_piece_is_another_s_mirror() {
        // free pentominoes: a shape and its reflection are one piece, so no two
        // of the twelve may be reflections of each other
        for (i, shape) in SHAPES.iter().enumerate() {
            let mine: HashSet<Vec<(i32, i32)>> = turns(*shape).into_iter().collect();

            for other in SHAPES.iter().skip(i + 1) {
                let cells = Piece::new(*other).board_cells();
                let flipped: [(i32, i32); CELLS] =
                    std::array::from_fn(|i| (-cells[i].0, cells[i].1));

                assert!(
                    !mine.contains(&normalized(flipped)),
                    "{:?} is {:?} reflected",
                    other,
                    shape
                );
            }
        }
    }

    #[test]
    fn every_piece_has_its_own_color() {
        for (i, shape) in SHAPES.iter().enumerate() {
            for other in SHAPES.iter().skip(i + 1) {
                assert_ne!(shape.color(), other.color(), "{:?} and {:?}", shape, other);
            }
        }
    }

    #[test]
    fn four_rotations_come_back_around() {
        for shape in SHAPES {
            let piece = Piece::new(shape);
            let turned = piece.rotated_cw().rotated_cw().rotated_cw().rotated_cw();

            assert_eq!(
                sorted(turned.board_cells()),
                sorted(piece.board_cells()),
                "{:?}",
                shape
            );
        }
    }

    #[test]
    fn x_does_not_change_when_rotated() {
        let piece = Piece::new(Shape::X);

        assert_eq!(
            sorted(piece.rotated_cw().board_cells()),
            sorted(piece.board_cells())
        );
    }

    #[test]
    fn each_piece_has_the_rotations_its_symmetry_allows() {
        // X is a plus and never changes; I and Z come back after a half turn;
        // the other nine take a full four
        for shape in SHAPES {
            let distinct: HashSet<Vec<(i32, i32)>> = turns(shape).into_iter().collect();
            let want = match shape {
                Shape::X => 1,
                Shape::I | Shape::Z => 2,
                _ => 4,
            };

            assert_eq!(distinct.len(), want, "{:?}", shape);
        }
    }

    #[test]
    fn rotations_undo_each_other() {
        for shape in SHAPES {
            let piece = Piece::new(shape);

            assert_eq!(
                sorted(piece.rotated_cw().rotated_ccw().board_cells()),
                sorted(piece.board_cells()),
                "{:?}",
                shape
            );
        }
    }

    #[test]
    fn moving_shifts_every_cell() {
        let piece = Piece::at(Shape::T, (4, 1));
        let moved = piece.moved(1, 2);

        for (before, after) in piece.board_cells().iter().zip(moved.board_cells().iter()) {
            assert_eq!(*after, (before.0 + 1, before.1 + 2));
        }
    }
}
