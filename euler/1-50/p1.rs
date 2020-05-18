fn main () {
  let mut answer = 0;
  for x in 0..1000 {
    if x % 3 == 0 || x % 5 == 0 {
      answer += x;
    }
  }
  println!("{}", answer)
}
