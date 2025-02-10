#[derive(Debug, Clone, Copy)]
pub struct Row {
  id: usize,
  mask: usize,
  cols: usize,
  diag_l: usize,
  diag_r: usize,
  available_pos: usize,
}

impl Row {
  pub fn new(size: usize) -> Self {
    let all_cols = (1 << size) - 1;

    // Taking advantage of the problem symmetry.
    let half_cols = (1 << (size / 2)) - 1;

    Self {
      id: 0,
      mask: all_cols,
      cols: 0,
      diag_l: 0,
      diag_r: 0,
      available_pos: half_cols,
    }
  }

  pub fn id(&self) -> usize {
    self.id
  }

  pub fn has_available_positions(&self) -> bool {
    self.available_pos != 0
  }

  fn get_lowest_bit(&self) -> usize {
    self.available_pos & self.available_pos.wrapping_neg()  // Same as: x & -x.
  }

  pub fn get_next_column(&self) -> u32 {
    self.get_lowest_bit().trailing_zeros()
  }

  pub fn next_row(&mut self) -> Self {
    let bit = self.get_lowest_bit();

    // Remove the lowest 1 bit from the available positions in the current row.
    self.available_pos ^= bit;

    let cols = self.cols | bit;
    let diag_l = (self.diag_l | bit) << 1;
    let diag_r = (self.diag_r | bit) >> 1;
    let occupied = cols | diag_l | diag_r;
    
    Self {
      id: self.id+1,
      mask: self.mask,
      cols,
      diag_l,
      diag_r,
      available_pos: !occupied & self.mask,
    }
  }
}
