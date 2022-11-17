pub trait AsPrimatives {
    fn as_u8(&self) -> u8;
}

#[macro_export]
macro_rules! impl_as_primative {
  ($($t:ty),*) => {
    use atrl_macros::prelude::AsPrimatives;

      $(
          impl AsPrimatives for $t {
              fn as_u8(&self) -> u8 {
                  *self as u8
              }
          }
      )*
  };
}
