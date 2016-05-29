extern crate rand;

fn main() {
  use rand::Rng;

  const WIDTH: usize = 600;
  const HEIGHT: usize = 600;
  const TOTAL_CELLS: usize = WIDTH * HEIGHT;

  let mut field: [bool; TOTAL_CELLS] = [false; TOTAL_CELLS];
  let mut field_buffer: [bool; TOTAL_CELLS] = [false; TOTAL_CELLS];

  let mut rng = rand::thread_rng();

  // fill the initial field with a random state
  for i in 0..field.len() {
    field[i] = rng.gen::<bool>()
  }

}
