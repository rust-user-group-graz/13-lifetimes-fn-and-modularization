fn long_a_but_short_number<'a>() {
  // <number>
  let number = 12;
  let number_ref: &'a i32 = &number;
  println!("{}", number_ref);
  // </number>
}


fn main() {
    long_a_but_short_number();
}
