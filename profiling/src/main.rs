mod utils;
use rat::{Params, Rat};
use utils::generate_signal;

fn main() {
  let mut rat = Rat::new(44100.);
  let mut params = Params::new(44100.);
  params.set(0.5, 0.5, 0.5);

  loop {
    let input = generate_signal();
    rat.process(input, &mut params);
  }
}
