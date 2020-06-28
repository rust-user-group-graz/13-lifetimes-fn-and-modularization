fn dump<'a>(x: &'a u64, y: &'a u64) {
    println!("x={} y={}", x, y);
}

fn main() {
  {
    let a = &42;
    {
      let b = &72;
      dump(a, b);
    }
  }
}
