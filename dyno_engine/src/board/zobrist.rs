use crate::defs::{NrOf, Piece, Side, Sides, Square, EMPTY};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_big_array::BigArray;

#[derive(Eq, PartialEq, PartialOrd, Copy, Clone, Ord, Debug)]
pub struct Array<T, const N: usize>(pub [T; N]);

impl<'de, T: Deserialize<'de>, const N: usize> Deserialize<'de> for Array<T, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        Ok(Self(<[T; N] as BigArray<T>>::deserialize(deserializer)?))
    }
}

impl<T: Serialize, const N: usize> Serialize for Array<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        <[T; N] as BigArray<T>>::serialize(&self.0, serializer)
    }
}

/* Random number for all sides for all pieces on all squares */
type PieceRandoms = Box<Array<Array<Array<u64, 64>, 6>,2>>;
type CastlingRandoms = [u64; NrOf::CASTLING_PERMISSIONS];
type SideRandoms = [u64; Sides::BOTH];
type EpRandoms = [u64; NrOf::SQUARES + 1];

pub type ZobristKey = u64;

// 256 bit (8 bits x 32) seed
const RNG_SEED: [u8; 32] = [125; 32];

#[derive(Serialize, Deserialize, Debug)]
pub struct ZobristRandoms {
    rnd_pieces: PieceRandoms,
    rnd_castling: CastlingRandoms,
    rnd_sides: SideRandoms,
    #[serde(with = "BigArray")]
    rnd_en_passant: EpRandoms,
}

impl ZobristRandoms {
    pub fn new() -> Self {
        let mut random = ChaChaRng::from_seed(RNG_SEED);
        let mut zobrist_randoms = Self {
            rnd_pieces: Box::new(Array([Array([Array([EMPTY; 64]); 6]); 2])),
            rnd_castling: [EMPTY; NrOf::CASTLING_PERMISSIONS],
            rnd_sides: [EMPTY; Sides::BOTH],
            rnd_en_passant: [EMPTY; NrOf::SQUARES + 1],
        };

        zobrist_randoms.rnd_pieces.0.iter_mut().for_each(|side| {
            side.0.iter_mut().for_each(|piece| {
                piece
                    .0.iter_mut()
                    .for_each(|square| *square = random.gen::<u64>())
            })
        });

        zobrist_randoms
            .rnd_castling
            .iter_mut()
            .for_each(|permission| *permission = random.gen::<u64>());

        zobrist_randoms
            .rnd_sides
            .iter_mut()
            .for_each(|side| *side = random.gen::<u64>());

        zobrist_randoms
            .rnd_en_passant
            .iter_mut()
            .for_each(|ep| *ep = random.gen::<u64>());

        zobrist_randoms
    }

    pub fn piece(&self, side: Side, piece: Piece, square: Square) -> ZobristKey {
        self.rnd_pieces.0[side].0[piece].0[square]
    }

    pub fn castling(&self, castling_permissions: u8) -> ZobristKey {
        self.rnd_castling[castling_permissions as usize]
    }

    pub fn side(&self, side: Side) -> u64 {
        self.rnd_sides[side]
    }

    pub fn en_passant(&self, en_passant: Option<u8>) -> ZobristKey {
        match en_passant {
            Some(ep) => self.rnd_en_passant[ep as usize],
            None => self.rnd_en_passant[NrOf::SQUARES],
        }
    }
}
