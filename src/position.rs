use std::io::BufReader;
use crate::Bitboard;
use crate::bitboard::EMPTY;
use crate::types::{Castling, Color, Piece, PieceType, Square};
use crate::types::PieceType::{*};

/// Board position for new game
pub const DEFAULT_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
/// The total number of colors on a chessboard.
pub const COLOR_CNT: usize = 2;
/// The total number of types of pieces on a chessboard.
pub const PIECE_TYPE_CNT: usize = 8;
/// The total number of squares on a chessboard.
pub const SQ_CNT: usize = 64;

pub static SQ_DISPLAY_ORDER: [Square; SQ_CNT] = [
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
];

pub static SQ_INDEX_ORDER: [Square; SQ_CNT] = [
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
];

#[derive(Copy, Clone)]
pub struct Position {
    // AND the following masks to get the pieces per color
    /// Bitboards for each type of piece regardless of color
    bbs: [Bitboard; PIECE_TYPE_CNT],
    /// Bitboards for pieces of each color
    bbs_color: [Bitboard; COLOR_CNT],
    /// array of u8's, with standard ordering mapping index to square
    board: [Piece; SQ_CNT],
    /// Who's turn is it to play?
    turn: Color,
    /// Castle rights for both sides
    castle_rights: Castling,
    /// En passant target square, square behind pawn that just moved two spaces
    ep_square: Square,
    /// Number of halfmoves since last pawn advance or capture
    rule50_count: u32,
    /// Number of halfmoves starting at 0.
    game_ply: u32,
}

impl Position {

    pub fn from_fen(fen_str: &str) -> Self {
        /*
           A FEN string defines a particular position using only the ASCII character set.
           A FEN string contains six fields separated by a space. The fields are:
           1) Piece placement (from white's perspective). Each rank is described, starting
              with rank 8 and ending with rank 1. Within each rank, the contents of each
              square are described from file A through file H. Following the Standard
              Algebraic Notation (SAN), each piece is identified by a single letter taken
              from the standard English names. White pieces are designated using upper-case
              letters ("PNBRQK") whilst Black uses lowercase ("pnbrqk"). Blank squares are
              noted using digits 1 through 8 (the number of blank squares), and "/"
              separates ranks.
           2) Active color. "w" means white moves next, "b" means black.
           3) Castling availability. If neither side can castle, this is "-". Otherwise,
              this has one or more letters: "K" (White can castle kingside), "Q" (White
              can castle queenside), "k" (Black can castle kingside), and/or "q" (Black
              can castle queenside).
           4) En passant target square (in algebraic notation). If there's no en passant
              target square, this is "-". If a pawn has just made a 2-square move, this
              is the position "behind" the pawn. Following X-FEN standard, this is recorded only
              if there is a pawn in position to make an en passant capture, and if there really
              is a pawn that might have advanced two squares.
           5) Halfmove clock. This is the number of halfmoves since the last pawn advance
              or capture. This is used to determine if a draw can be claimed under the
              fifty-move rule.
           6) Fullmove number. The number of the full move. It starts at 1, and is
              incremented after Black's move.
        */
        let mut fields: Vec<&str> = fen_str.split_whitespace().collect();
        let mut idx: usize = 0;

        let mut p = Position {
            bbs: [Bitboard(0); PIECE_TYPE_CNT],
            bbs_color: [Bitboard(0); COLOR_CNT],
            board: [Piece::None; SQ_CNT],
            turn: Color::White,
            castle_rights: Castling::C_NONE,
            ep_square: Square::NONE,
            rule50_count: 0,
            game_ply: 1,
        };

        // 1. Piece placement
        for c in fields[0].chars() {
            if c.is_digit(10) {
                idx += 1 * c.to_digit(10).unwrap() as usize; // Advance the given number of files
            } else if c == '/' {
                // do nothing
            } else {
                let color = if c.is_uppercase() {Color::White} else {Color::Black};
                let piece = match c {
                    'p' | 'P' => PieceType::P,
                    'n' | 'N' => PieceType::N,
                    'b' | 'B' => PieceType::B,
                    'r' | 'R' => PieceType::R,
                    'q' | 'Q' => PieceType::Q,
                    'k' | 'K' => PieceType::K,
                    _ => PieceType::None,
                };
                p.put_piece(Piece::make(color, piece), &SQ_DISPLAY_ORDER[idx]);
                idx += 1;
            }
        }

        // 2. Active color
        p.turn = match fields[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!(),
        };

        // 3. Castling availability
        for c in fields[2].chars() {
            p.castle_rights |= match c {
                'K' => Castling::C_WHITE_K,
                'Q' => Castling::C_WHITE_Q,
                'k' => Castling::C_BLACK_K,
                'q' => Castling::C_BLACK_Q,
                _ => Castling::C_NONE,
            };
        }

        // 4. En passant target square
        for (i, c) in fields[3].chars().enumerate() {
            if i == 0 {
                p.ep_square = match c {
                    '-' => Square::NONE,
                    'a' => Square::A3,
                    'b' => Square::B3,
                    'c' => Square::C3,
                    'd' => Square::D3,
                    'e' => Square::E3,
                    'f' => Square::F3,
                    'g' => Square::G3,
                    'h' => Square::H3,
                    _ => panic!(),
                };
            } else if i == 1 {
                p.ep_square = SQ_INDEX_ORDER[p.ep_square.index() + 24] // 24 squares to move 3 ranks
            }
        }

        // 5. Halfmove clock
        p.rule50_count = fields[4].parse().unwrap();

        // 6. Convert fullmove number to game ply
        let fullmove: u32 = fields[5].parse().unwrap();
        p.game_ply = 2 * (fullmove - 1) + if matches!(p.turn, Color::Black) {1} else {0};
        p
    }

    pub fn put_piece(&mut self, pc: Piece, s: &Square) {
        self.board[s.index()] = pc;
        self.bbs[pc.type_of() as usize] |= s.to_bb();
        self.bbs_color[pc.color() as usize] |= s.to_bb();
    }

    pub fn pretty(&self) -> String {
        let mut s = String::from("+---+---+---+---+---+---+---+---+\n");
        for sq in SQ_DISPLAY_ORDER.iter() {
            let pc = self.board[sq.index()];
            let c = if !matches!(pc, Piece::None) {
                pc.character()
            } else {
                ' '
            };
            s.push_str(format!("| {} ", c).as_str());
            if sq.index() % 8 == 7 {
                s.push_str("| ");
                s.push_str(&(sq.index() / 8 + 1).to_string());
                s.push_str("\n+---+---+---+---+---+---+---+---+\n");
            }
        }
        s += "  a   b   c   d   e   f   g   h\n";
        s
    }
}
