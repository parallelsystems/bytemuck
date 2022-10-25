use super::*;

// Note(Lokathor): This is the neat part!!
unsafe impl<T: PodInOption> Pod for Option<T> {}

// Note(Lokathor): This is the neat part!!
unsafe impl<T: PodInOption> AnyBitPattern for Option<T> {}

unsafe impl<T: NoUninit> NoUninit for Option<T> {}

/// Trait for types, which are [Pod](Pod) when wrapped in
/// [Option](core::option::Option).
///
/// ## Safety
///
/// * `Option<T>` must uphold the same invariants as [Pod](Pod).
/// * **Reminder:** pointers are **not** pod! **Do not** mix this trait with a
///   newtype over [NonNull](core::ptr::NonNull).
pub unsafe trait PodInOption:
  ZeroableInOption + Copy + NoUninit + CheckedBitPattern + 'static
{
}

unsafe impl CheckedBitPattern for NonZeroI16 {
  type Bits = i16;

  fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
    *bits != 0
  }
}
unsafe impl PodInOption for NonZeroI16 {}

unsafe impl CheckedBitPattern for NonZeroI32 {
  type Bits = i32;

  fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
    *bits != 0
  }
}
unsafe impl PodInOption for NonZeroI32 {}

unsafe impl CheckedBitPattern for NonZeroI64 {
  type Bits = i64;

  fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
    *bits != 0
  }
}
unsafe impl PodInOption for NonZeroI64 {}

unsafe impl CheckedBitPattern for NonZeroI128 {
  type Bits = i128;

  fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
    *bits != 0
  }
}
unsafe impl PodInOption for NonZeroI128 {}

unsafe impl CheckedBitPattern for NonZeroIsize {
  type Bits = isize;

  fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
    *bits != 0
  }
}
unsafe impl PodInOption for NonZeroIsize {}

unsafe impl CheckedBitPattern for NonZeroU8 {
  type Bits = u8;

  fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
    *bits != 0
  }
}
unsafe impl PodInOption for NonZeroU8 {}

unsafe impl CheckedBitPattern for NonZeroU16 {
  type Bits = u16;

  fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
    *bits != 0
  }
}
unsafe impl PodInOption for NonZeroU16 {}

unsafe impl CheckedBitPattern for NonZeroU32 {
  type Bits = u32;

  fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
    *bits != 0
  }
}
unsafe impl PodInOption for NonZeroU32 {}

unsafe impl CheckedBitPattern for NonZeroU64 {
  type Bits = u64;

  fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
    *bits != 0
  }
}
unsafe impl PodInOption for NonZeroU64 {}

unsafe impl CheckedBitPattern for NonZeroU128 {
  type Bits = u128;

  fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
    *bits != 0
  }
}
unsafe impl PodInOption for NonZeroU128 {}

unsafe impl CheckedBitPattern for NonZeroUsize {
  type Bits = usize;

  fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
    *bits != 0
  }
}
unsafe impl PodInOption for NonZeroUsize {}
