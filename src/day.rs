pub trait Day {
  fn day() -> u8;

  fn solve_a(year: u16) {
    Self::not_implemented("a", year);
  }

  fn solve_b(year: u16) {
    Self::not_implemented("a", year);
  }

  fn input_dir(year: u16) -> String {
    format!("inputs/{}/{}", year, Self::day())
  }

  fn not_implemented(variant: &str, year: u16) {
    println!("Warning: Variant {} of day {} ({}) not implemented yet", variant, Self::day(), year);
  }

  fn solve(variant: &str, year: u16) {
    match variant {
      "a" => Self::solve_a(year),
      "b" => Self::solve_b(year),
      _ => println!("Error: Invalid variant: {}", variant),
    }
  }
}
