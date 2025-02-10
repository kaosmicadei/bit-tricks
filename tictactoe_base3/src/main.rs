use libgame::{GameState, CellValue};

fn main() {
  use CellValue::*;

  let moves = [
    (0, Cross),
    (2, Circle),
    (4, Cross),
    (7, Circle),
    (7, Empty),
    (8, Circle),
  ];

  let game_state = moves
    .iter()
    .fold(GameState::new(), |acc, m| acc.set(m.0, m.1));

  assert_eq!(game_state, 0x33a6); // 0x33a6 = 13222
}
