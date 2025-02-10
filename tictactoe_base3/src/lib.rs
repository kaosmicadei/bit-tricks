#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum CellValue {
  Empty = 0,
  Cross = 1,
  Circle = 2,
}

#[derive(Debug, Clone, Copy)]
pub struct GameState {
  state: u16,
}

impl GameState {
  pub fn new() -> Self {
    Self { state: 0u16 }
  }

  // Avoid recomputation. 
  const fn power_of_three() -> [u16; 9] {
    [1, 3, 9, 27, 81, 243, 729, 2187, 6561]
  }

  pub fn get(&self, cell: usize) -> u16 {
    (self.state / GameState::power_of_three()[cell]) % 3
  }

  pub fn set(&self, cell: usize, value: CellValue) -> Self {
    let old_value = self.get(cell);
    let new_state = (value as u16)
      .wrapping_sub(old_value) // The wrapping_* operartions to avoid overflow.
      .wrapping_mul(GameState::power_of_three()[cell])
      .wrapping_add(self.state);

    Self { state: new_state }
  }
}

impl PartialEq<u16> for GameState {
    fn eq(&self, other: &u16) -> bool {
        self.state == *other
    }
}

impl std::fmt::Display for GameState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#06x}", self.state)
  }
}
