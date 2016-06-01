extern crate rand;
extern crate piston_window;

use piston_window::*;

fn main() {
  use rand::Rng;

  const WIDTH: usize = 600;
  const HEIGHT: usize = 600;
  const TOTAL_CELLS: usize = WIDTH * HEIGHT;

  let mut window: PistonWindow =
    WindowSettings::new("Life.rs", [600, 600])
    .exit_on_esc(true).build().unwrap();

  let mut field: [bool; TOTAL_CELLS] = [false; TOTAL_CELLS];
  let mut field_buffer: [bool; TOTAL_CELLS] = [false; TOTAL_CELLS];

  let mut rng = rand::thread_rng();

  // fill the initial field with a random state
  for i in 0..field.len() {
    field[i] = rng.gen::<bool>()
  }

  while let Some(event) = window.next() {
    //step();
    draw(&mut window, &event, &field);
  }

}

fn draw(window: &mut PistonWindow, event: &piston_window::Event, field: &[bool]) {
  window.draw_2d(event, |c, g| {
    clear([1.0; 4], g);
    rectangle([1.0, 0.0, 0.0, 1.0],
              [0.0, 0.0, 100.0, 100.0],
              c.transform, g);
  });
}
