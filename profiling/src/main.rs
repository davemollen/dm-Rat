mod utils;
use rat::Rat;
use utils::generate_signal;

fn main() {
  let mut rat = Rat::new(44100.);

  loop {
    let input = generate_signal();
    rat.process(input, 0.5, 0.5, 0.5);
  }
}
