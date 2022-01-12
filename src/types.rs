use std::ops::{AddAssign, BitOr, BitOrAssign};
use crate::Bitboard;

#[derive(Copy, Clone)]
pub enum Color {
    White = 0,
    Black = 1,
}

pub enum PieceType {
    None = 0,
    P = 1,
    N = 2,
    B = 3,
    R = 4,
    Q = 5,
    K = 6,
    All = 7
}

// Fourth bit of white pieces is 0
// Fourth bit of black pieces is 1
// 0b1000 is undefined
#[derive(Copy, Clone)]
pub enum Piece {
    None        = 0b0000,
    WhitePawn   = 0b0001,
    WhiteKnight = 0b0010,
    WhiteBishop = 0b0011,
    WhiteRook   = 0b0100,
    WhiteQueen  = 0b0101,
    WhiteKing   = 0b0110,
    BlackPawn   = 0b1001,
    BlackKnight = 0b1010,
    BlackBishop = 0b1011,
    BlackRook   = 0b1100,
    BlackQueen  = 0b1101,
    BlackKing   = 0b1110,
}

impl Piece {
    pub fn make(color: Color, piece_type: PieceType) -> Piece {
        let bits: u8 = (color as u8) << 3 | piece_type as u8;
        match bits {
            0b0001 => Piece::WhitePawn,
            0b0010 => Piece::WhiteKnight,
            0b0011 => Piece::WhiteBishop,
            0b0100 => Piece::WhiteRook,
            0b0101 => Piece::WhiteQueen,
            0b0110 => Piece::WhiteKing,
            0b1001 => Piece::BlackPawn,
            0b1010 => Piece::BlackKnight,
            0b1011 => Piece::BlackBishop,
            0b1100 => Piece::BlackRook,
            0b1101 => Piece::BlackQueen,
            0b1110 => Piece::BlackKing,
            _ => Piece::None,
        }
    }

    pub fn color(self) -> Color {
        let color_bit = (self as u8 >> 3) & 0b1;
        return if color_bit == 0 {Color::White} else {Color::Black}
    }

    pub fn type_of(self) -> PieceType {
        let piece_type = self as u8 & 0b0111;
        match piece_type {
            0 => PieceType::None,
            1 => PieceType::P,
            2 => PieceType::N,
            3 => PieceType::B,
            4 => PieceType::R,
            5 => PieceType::Q,
            6 => PieceType::K,
            7 => PieceType::All,
            _ => PieceType::None,
        }
    }

    pub fn character(self) -> char {
        match self {
            Piece::None        => panic!(),
            Piece::WhitePawn   => 'P',
            Piece::WhiteKnight => 'N',
            Piece::WhiteBishop => 'B',
            Piece::WhiteRook   => 'R',
            Piece::WhiteQueen  => 'Q',
            Piece::WhiteKing   => 'K',
            Piece::BlackPawn   => 'p',
            Piece::BlackKnight => 'n',
            Piece::BlackBishop => 'b',
            Piece::BlackRook   => 'r',
            Piece::BlackQueen  => 'q',
            Piece::BlackKing   => 'k',
        }
    }

    pub fn symbol(self) -> char {
        match self {
            Piece::None        => panic!(),
            Piece::WhitePawn   => '♙',
            Piece::WhiteKnight => '♘',
            Piece::WhiteBishop => '♗',
            Piece::WhiteRook   => '♖',
            Piece::WhiteQueen  => '♕',
            Piece::WhiteKing   => '♔',
            Piece::BlackPawn   => '♟',
            Piece::BlackKnight => '♞',
            Piece::BlackBishop => '♝',
            Piece::BlackRook   => '♜',
            Piece::BlackQueen  => '♛',
            Piece::BlackKing   => '♚',
        }
    }
}

#[derive(Copy, Clone)]
pub struct Square(pub u8);

// Constants
impl Square {
    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);
    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);
    pub const A3: Square = Square(16);
    pub const B3: Square = Square(17);
    pub const C3: Square = Square(18);
    pub const D3: Square = Square(19);
    pub const E3: Square = Square(20);
    pub const F3: Square = Square(21);
    pub const G3: Square = Square(22);
    pub const H3: Square = Square(23);
    pub const A4: Square = Square(24);
    pub const B4: Square = Square(25);
    pub const C4: Square = Square(26);
    pub const D4: Square = Square(27);
    pub const E4: Square = Square(28);
    pub const F4: Square = Square(29);
    pub const G4: Square = Square(30);
    pub const H4: Square = Square(31);
    pub const A5: Square = Square(32);
    pub const B5: Square = Square(33);
    pub const C5: Square = Square(34);
    pub const D5: Square = Square(35);
    pub const E5: Square = Square(36);
    pub const F5: Square = Square(37);
    pub const G5: Square = Square(38);
    pub const H5: Square = Square(39);
    pub const A6: Square = Square(40);
    pub const B6: Square = Square(41);
    pub const C6: Square = Square(42);
    pub const D6: Square = Square(43);
    pub const E6: Square = Square(44);
    pub const F6: Square = Square(45);
    pub const G6: Square = Square(46);
    pub const H6: Square = Square(47);
    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);
    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);

    pub const NONE: Square = Square(64);
}

impl Square {
    pub fn index(&self) -> usize {
        self.0 as usize
    }

    pub fn to_bb(&self) -> Bitboard {
        Bitboard(1u64 >> self.0 as u64)
    }
}

/// Castling denotes castle rights
/// Each of the rightmost 4 bits in the u8 marks the right to castle
#[derive(Copy, Clone)]
pub struct Castling(u8);

impl Castling {
    pub const C_WHITE_K: Castling = Castling(0b1000);
    pub const C_WHITE_Q: Castling = Castling(0b0100);
    pub const C_BLACK_K: Castling = Castling(0b0010);
    pub const C_BLACK_Q: Castling = Castling(0b0001);
    pub const C_ALL: Castling = Castling(0b1111);
    pub const C_NONE: Castling = Castling(0b0000);
}

impl BitOr for Castling {
    type Output = Castling;
    fn bitor(self, other: Castling) -> Castling {
        Castling(self.0 | other.0)
    }
}

impl BitOrAssign for Castling {
    fn bitor_assign(&mut self, other: Castling) -> () { self.0 |= other.0; }
}