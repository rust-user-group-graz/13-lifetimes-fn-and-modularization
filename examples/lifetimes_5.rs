fn first<'a, 'b>(x: &'a u64, _y: &'b u64) -> &'a u64 {
  x
}

fn main() {
  println!("{}", first(&42, &72));
}
