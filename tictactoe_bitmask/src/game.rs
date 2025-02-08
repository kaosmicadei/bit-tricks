use crate::status::GameStatus;

/*
  Bit-fields:
    Occupied cells: 9 bits (0 = empty, 1 = occupied)
    Player 1 cells: 9 bits (0 = not used by player 1, 1 = used by player 1)
    Player to move: 1 bit  (0 = player 1, 1 = player 2)
    Game status:    2 bits (00 = ongoing, 01 = draw, 10 = player 1 victory, 11 = player 2 victory)
  
  Total: 21 bits

  Because it only requires a single `u32`, it's possible to write it in a stateless way (functional)
  since copy a `u32` is a cheap operation.
 */

#[derive(Debug, Clone, Copy)]
pub struct Game {
  packed: u32,
}

impl Game {
  pub fn new() -> Self {
    return Self { packed: 0 }
  }

  fn occupied_cells(&self) -> u32 {
    self.packed & 0b1_1111_1111
  }

  fn with_occupied_cells(&self, cells: u32) -> Self {
    let reseted_cells = self.packed & !0b1_1111_1111;
    let new_cells = cells & 0b1_1111_1111;

    Self { packed: reseted_cells | new_cells }
  }

  fn player1_cells(&self) -> u32 {
    (self.packed >> 9) & 0b1_1111_1111
  }

  fn player2_cells(&self) -> u32 {
    (self.packed ^ (self.packed >> 9)) & 0b1_1111_1111
  }

  fn with_player1_cells(&self, cells: u32) -> Self {
    let reseted_cells = self.packed & !(0b1_1111_1111 << 9);
    let new_cells = (cells & 0b1_1111_1111) << 9;

    Self { packed: reseted_cells | new_cells }
  }

  fn player_to_move(&self) -> u32 {
    (self.packed >> 18) & 1
  }

  fn with_player_to_move(&self, player: u32) -> Self {
    let reseted_player = self.packed & !(1 << 18);
    let new_player = (player & 1) << 18;

    Self { packed: reseted_player | new_player }
  }

  fn is_player1_move(&self) -> bool {
    ((self.packed >> 18) & 1) == 0
  }

  fn game_status(&self) -> GameStatus {
    GameStatus::from_value((self.packed >> 19) & 0b11)
  }

  fn with_game_status(&self, status: GameStatus) -> Self {
    let reseted_status = self.packed & !(0b11 << 19);
    let new_status = (status as u32) << 19;

    Self { packed: reseted_status | new_status}
  }

  fn is_valid_move(&self, player_move: u32) -> bool {
    if player_move > 8 {
      return false
    }

    let cell = 1 << player_move;
    (self.occupied_cells() & cell) == 0
  }

  pub fn make_move(&self, player_move: u32) -> Option<Self> {
    if !self.is_valid_move(player_move) {
      return None
    }

    let cell = 1u32 << player_move;
    let new_occupied_cells = self.occupied_cells() | cell;
    let is_player1_move = self.is_player1_move() as u32;
    let new_player1_cells = self.player1_cells() | (cell * is_player1_move);
    let new_game_status = GameStatus::from_cells(new_player1_cells,
      new_occupied_cells ^ new_player1_cells);
    let next_player = self.player_to_move() ^ 1;
    
    Some(Game::new()
      .with_occupied_cells(new_occupied_cells)
      .with_player1_cells(new_player1_cells)
      .with_player_to_move(next_player)
      .with_game_status(new_game_status))
  }
}

impl std::fmt::Display for Game {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let game_string = (0..=8).into_iter()
      .map(|i| {
        if (self.player1_cells() & (1u32 << i)) != 0 {
          String::from(" X ")
        } else if (self.player2_cells() & (1u32 << i)) != 0 {
          String::from(" O ")
        } else {
          format!("[{}]", i)
        }
      })
      .collect::<Vec<String>>()
      .chunks(3)
      .map(|row| row.join(" "))
      .collect::<Vec<String>>()
      .join("\n");

      write!(f, "{}\nStatus: {:?}\nNext: Player {}",
             game_string,
             self.game_status(),
             self.player_to_move() + 1)
  }
}

#[cfg(test)]
mod tests {
  use super::Game;

  #[test]
  fn test_move() {
    let game = Game::new().make_move(0).unwrap();
    let cell = 1u32;
    let player_cell = 1u32 << 9;
    let player_to_move = 1u32 << 18;

    assert_eq!(game.packed, cell | player_cell | player_to_move)
  }
}
