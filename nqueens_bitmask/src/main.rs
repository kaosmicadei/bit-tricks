use solver::NQueens;

fn main() {
  let solver = NQueens::new(4);
  for solution in solver {
    println!("{:?}", solution);
  }
}
