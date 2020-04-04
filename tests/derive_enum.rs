use bs_diesel_utils_codegen::BSDieselEnum;

#[test]
fn test_derive_enum() {
  #[derive(Debug, BSDieselEnum)]
  #[repr(i32)]
  enum Enum {
    A = 1,
  }
}
