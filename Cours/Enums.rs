#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                    // Simple variant
    Run(Direction),          // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

fn main() {
    let dir = Direction::Left;
    println!("On this turn: {:?}", dir);
    let player_move: PlayerMove = PlayerMove::Run(dir);
    println!("On this turn: {:?}", player_move);
    let player_move: PlayerMove = PlayerMove::Teleport { x: 10, y: 20 };
    println!("On this turn: {:?}", player_move);
    let player_move: PlayerMove = PlayerMove::Pass;
    println!("On this turn: {:?}", player_move);
}
