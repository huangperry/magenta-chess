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

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
    None,
}

impl Square {
    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn bb(&self) -> Bitboard {
        Bitboard(1u64 >> *self as u64)
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