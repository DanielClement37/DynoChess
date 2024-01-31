use std::sync::{Arc, Mutex};
use crate::{board, perft};
use crate::movegen::MoveGen;

pub fn run_tests() {
    let mut test_fens: Vec<(&str, i8, u64)> = vec![];
    test_fens.push(("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ".trim(), 6, 119060324));
    test_fens.push(("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ".trim(), 5, 193690690));
    test_fens.push(("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -".trim(), 6, 11030083));
    test_fens.push(("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".trim(), 5, 15833292));
    test_fens.push(("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".trim(), 5, 89941194));
    test_fens.push(("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10".trim(), 5, 164075551));

    //iterate through the test_fens vector and run the tests
    for (fen, depth, expected) in test_fens {
        let mut board = board::Board::new();
        board.fen_read(Some(fen)).ok();
        let movegen = Arc::new(MoveGen::new());
        let result = perft::run(Arc::new(Mutex::new(board)), depth, movegen);

        assert_eq!(result, expected);
    }
}