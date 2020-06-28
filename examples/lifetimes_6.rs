fn to_string<'a>(msg: &str) -> &'a String {
  &String::from(msg)
}

fn main() {
  println!("{}", to_string(&"hello"));
}
