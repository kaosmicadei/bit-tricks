use solver::NQueens;

fn main() {
  let mut solver = NQueens::new(32);
  println!("{:?}", solver.next().unwrap());
}
