extern crate rand;

use rand::{Rng};

#[derive(Clone, Copy)]
pub struct Fix24x64 {
  inner:    i64,
}

impl Fix24x64 {
  pub fn round_nearest_f32(x: f32) -> Fix24x64 {
    Fix24x64{inner: (x * 16_777_216.0).round() as i64}
  }

  pub fn round_stochastic_f32<R>(x: f32, rng: &mut R) -> Fix24x64
  where R: Rng {
    // FIXME(20160326)
    unimplemented!();
  }

  pub fn from_i64_repr(y: i64) -> Fix24x64 {
    Fix24x64{inner: y}
  }

  pub fn from_usize_repr(y: usize) -> Fix24x64 {
    Fix24x64{inner: y as isize as i64}
  }

  pub fn into_f32(self) -> f32 {
    self.inner as f32 * 5.960_464_477_539_062_5e-8
  }

  pub fn into_i64_repr(self) -> i64 {
    self.inner
  }

  pub fn into_usize_repr(self) -> usize {
    self.inner as isize as usize
  }
}
