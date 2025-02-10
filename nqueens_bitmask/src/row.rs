#[derive(Debug, Clone, Copy)]
pub struct Row {
  id: usize,
  mask: usize,
  cols: usize,
  diag_l: usize,
  diag_r: usize,
  avialble_pos: usize,
}

impl Row {
  pub fn new(size: usize) -> Self {
    let all_cols = (1 << size) - 1;

    Self {
      id: 0,
      mask: all_cols,
      cols: 0,
      diag_l: 0,
      diag_r: 0,
      avialble_pos: all_cols,
    }
  }

  pub fn id(&self) -> usize {
    self.id
  }

  pub fn has_available_positions(&self) -> bool {
    self.avialble_pos != 0
  }

  fn get_lowest_bit(&self) -> usize {
    self.avialble_pos & self.avialble_pos.wrapping_neg()
  }

  pub fn get_next_column(&self) -> u32 {
    self.get_lowest_bit().trailing_zeros()
  }

  pub fn next_row(&mut self) -> Self {
    let bit = self.get_lowest_bit();

    // Remove the lowest 1 bit from the available positions in the current row.
    self.avialble_pos ^= bit;

    let cols = self.cols | bit;
    let diag_l = (self.diag_l | bit) << 1;
    let diag_r = (self.diag_r | bit) >> 1;
    
    Self {
      id: self.id+1,
      mask: self.mask,
      cols,
      diag_l,
      diag_r,
      avialble_pos: !(cols | diag_l | diag_r) & self.mask,
    }
  }
}
