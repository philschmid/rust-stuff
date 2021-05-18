pub struct Calculator {}

impl Calculator {
  pub fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
  }
  pub fn div(n1: i32, n2: i32) -> Result<f32, String> {
    match n2 {
      0 => Err(String::from("div by 0 not possible")),
      _ => Ok(n1 as f32 / n2 as f32),
    }
  }
}
