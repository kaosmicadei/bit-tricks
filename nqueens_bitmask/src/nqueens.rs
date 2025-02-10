use crate::row::*;

#[derive(Debug)]
pub struct NQueens {
  n: usize,
  solution: Vec<u32>,
  stack: Vec<Row>,
}

impl NQueens {
  pub fn new(n: usize) -> Self {
    Self {
      n,
      solution: Vec::new(),
      stack: vec![Row::new(n)]
    }
  }
}

impl Iterator for NQueens {
  type Item = Vec<u32>;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(mut row) = self.stack.pop() {
      if row.id() == self.n {
        let solution = self.solution.clone();
        self.solution.pop();
        return Some(solution)
      }

      if !row.has_available_positions() {
        // The solver has failed in adding the next queen to the board.
        // We remove the last queen from the solution and proceed with the next
        // option (backtracking). 
        self.solution.pop();
        continue;
      }

      self.solution.push(row.get_next_column());
      
      let next_row = row.next_row();
      self.stack.push(row);
      self.stack.push(next_row);
    }

    None
  }
}
