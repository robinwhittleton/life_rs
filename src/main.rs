extern crate rand;

fn main() {
  use rand::Rng;
  
  const WIDTH: usize = 600;
  const HEIGHT: usize = 600;
  const TOTAL_CELLS: usize = WIDTH * HEIGHT;

  let field: [bool; TOTAL_CELLS] = [false; TOTAL_CELLS];
  let field_buffer: [bool; TOTAL_CELLS] = [false; TOTAL_CELLS];


  let rng = rand::thread_rng();
  if rng.gen() { // random bool
      println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())
  }

  //println!("{}", field);

}
