use crate::row::*;

#[derive(Debug)]
pub struct NQueens {
  n: usize,
  solution: Box<[u32]>,
  stack: Vec<Row>,
}

impl NQueens {
  pub fn new(n: usize) -> Self {
    Self {
      n,
      solution: vec![u32::MAX; n].into_boxed_slice(),
      stack: vec![Row::new(n)]
    }
  }
}

impl Iterator for NQueens {
  type Item = Vec<u32>;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(mut row) = self.stack.pop() {
      if row.id() == self.n {
        let solution = self.solution.to_vec();
        self.solution[row.id()-1] = u32::MAX;
        return Some(solution)
      }

      if !row.has_available_positions() {
        // The solver has failed in adding the next queen to the board.
        // We remove the last queen from the solution and proceed with the next
        // option (backtracking). 
        if row.id() > 0 {
          self.solution[row.id()-1] = u32::MAX;
        }
        continue;
      }

      self.solution[row.id()] = row.get_next_column();
      
      let next_row = row.next_row();
      self.stack.push(row);
      self.stack.push(next_row);
    }

    None
  }
}
