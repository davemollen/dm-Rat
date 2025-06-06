#[path = "../src/utils.rs"]
mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use rat::{Params, Rat};
use utils::generate_signal_stream;

fn rat_bench(c: &mut Criterion) {
  let mut rat = Rat::new(44100.);
  let mut params = Params::new(44100.);
  params.set(0.5, 0.5, 0.5);
  let signal_stream = generate_signal_stream(44100);

  c.bench_function("rat", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        rat.process(*signal, &mut params);
      }
    })
  });
}

criterion_group!(benches, rat_bench);
criterion_main!(benches);
