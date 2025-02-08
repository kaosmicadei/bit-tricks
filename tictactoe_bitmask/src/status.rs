#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
  Ongoing = 0,
  Draw = 1,
  Player1Win = 2,
  Player2Win = 3,
}

impl GameStatus {
  pub fn from_value(value: u32) -> Self {
      match value {
          0 => GameStatus::Ongoing,
          1 => GameStatus::Draw,
          2 => GameStatus::Player1Win,
          3 => GameStatus::Player2Win,
          _ => panic!("Expected numbers from 0 to 3 only, got {} instead.", value),
      }
  }

  pub fn from_cells(player1_cells: u32, player2_cells: u32) -> Self {
    const WIN_MASKS: [u32; 8] = [
      0b000_000_111, 0b000_111_000, 0b111_000_000,  // rows
      0b001_001_001, 0b010_010_010, 0b100_100_100,  // columns
      0b001_010_100, 0b100_010_001,                 // diagonals
    ];

    for mask in WIN_MASKS {
      if player1_cells & mask == mask {
        return GameStatus::Player1Win
      } else if player2_cells & mask == mask {
        return GameStatus::Player2Win
      }
    }

    if player1_cells | player2_cells == 0b1_1111_1111 {
      return GameStatus::Draw
    }

    GameStatus::Ongoing
  }
}

#[cfg(test)]
mod tests {
  use super::GameStatus;

  #[test]
  fn test_from_value() {
    assert_eq!(GameStatus::from_value(2), GameStatus::Player1Win)
  }

  #[test]
  #[should_panic]
  fn test_from_wrong_value() {
    GameStatus::from_value(8);
  }

  #[test]
  fn test_from_cells() {
    assert_eq!(GameStatus::from_cells(0b001_010_101, 0b000_101_010), GameStatus::Player1Win)
  }
}
