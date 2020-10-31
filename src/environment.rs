#[derive(Debug)]
pub struct Environment(String);

impl Environment {
  pub fn new<T: ToString>(value: T) -> Self {
    Environment(value.to_string())
  }

  pub fn is_test(&self) -> bool {
    self.0 == "TEST"
  }
}
