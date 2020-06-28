fn dump<'a, 'b>(x: &'a u64, y: &'b u64) {
  println!("x={} y={}", x, y);
}

fn main() {
  dump(&42, &72);
}
