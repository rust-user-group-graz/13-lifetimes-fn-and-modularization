static NUM: u64 = 42;

fn main() {
  let x: &'static str = "hello world";
  println!("{}", x);

  let y: &'static u64 = &NUM;
  println!("{}", y);
}
