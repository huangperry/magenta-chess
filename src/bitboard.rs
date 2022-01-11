use std::ops::{BitAnd, BitOr, BitOrAssign, Mul};

#[derive(Copy, Clone)]
pub struct Bitboard(pub u64);
// Empty bitboard. !EMPTY to get the universe of squares.
pub const EMPTY: Bitboard = Bitboard(0);

impl Bitboard {

    /// Bitboards::pretty() returns an ASCII representation of a bitboard suitable
    /// to be printed to standard output. Useful for debugging.
    pub fn pretty(&self) -> String {

        let mut s = String::from("+---+---+---+---+---+---+---+---+\n");
        let mut stack = Vec::new();
        let mut row = String::new();
        for x in 0..64 {
            if self.0 & (1u64 << x) == (1u64 << x) {
                row.push_str("| X ");
            } else {
                row.push_str("|   ");
            }
            if x % 8 == 7 {
                row.push_str("| ");
                row.push_str(&(x / 8 + 1).to_string());
                row.push_str("\n+---+---+---+---+---+---+---+---+\n");
                stack.push(row.clone());
                row.clear();
            }
        }
        while let Some(top) = stack.pop() {
            s.push_str(&top);
        }

        s += "  a   b   c   d   e   f   g   h\n";

        s
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;
    fn bitor(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 | other.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, other: Bitboard) -> () { self.0 |= other.0; }
}