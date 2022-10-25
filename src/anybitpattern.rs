use core::{
  marker::{PhantomData, PhantomPinned},
  mem::ManuallyDrop,
  num::Wrapping,
};

use crate::{Zeroable};

/// Marker trait for "plain old data" types that are valid for any bit pattern.
///
/// The requirements are very similar to [`Pod`],
/// except that the type can allow uninit (or padding) bytes.
/// This limits what you can do with a type of this kind, but also broadens the
/// included types to `repr(C)` `struct`s that contain padding as well as
/// `union`s. Notably, you can only cast *immutable* references and *owned*
/// values into [`AnyBitPattern`] types, not *mutable* references.
///
/// [`Pod`] is a subset of [`AnyBitPattern`], meaning any `T: Pod` is also
/// [`AnyBitPattern`] but any `T: AnyBitPattern` is not necessarily [`Pod`].
///
/// [`AnyBitPattern`] is a subset of [`Zeroable`], meaning any `T:
/// AnyBitPattern` is also [`Zeroable`], but any `T: Zeroable` is not
/// necessarily [`AnyBitPattern  ]
///
/// # Derive
///
/// A `#[derive(AnyBitPattern)]` macro is provided under the `derive` feature
/// flag, which will automatically validate the requirements of this trait and
/// implement the trait for you for both structs and enums. This is the
/// recommended method for implementing the trait, however, it's also possible
/// to do manually. If you implement it manually, you *must* carefully follow
/// the below safety rules.
///
/// * *NOTE: even `C-style`, fieldless enums are intentionally **excluded** from
/// this trait, since it is **unsound** for an enum to have a discriminant value
/// that is not one of its defined variants.
///
/// # Safety
///
/// Similar to [`Pod`] except we disregard the rule about it must not contain
/// uninit bytes. Still, this is a quite strong guarantee about a type, so *be
/// careful* when implementing it manually.
///
/// * The type must be inhabited (eg: no
///   [Infallible](core::convert::Infallible)).
/// * The type must be valid for any bit pattern of its backing memory.
/// * Structs need to have all fields also be `AnyBitPattern`.
/// * It is disallowed for types to contain pointer types, `Cell`, `UnsafeCell`,
///   atomics, and any other forms of interior mutability.
/// * More precisely: A shared reference to the type must allow reads, and
///   *only* reads. RustBelt's separation logic is based on the notion that a
///   type is allowed to define a sharing predicate, its own invariant must hold
///   for shared references, and this predicate is the reasoning allows it to
///   deal with atomic and cells etc. We require the sharing predicate to be
///   trivial and permit only read-only access.
/// * There's probably more, don't mess it up (I mean it).
pub unsafe trait AnyBitPattern:
  Zeroable + Sized + Copy + 'static
{
}

#[cfg(feature = "min_const_generics")]
unsafe impl<T: AnyBitPattern, const N: usize> AnyBitPattern for [T; N] {}

#[cfg(not(feature = "min_const_generics"))]
impl_unsafe_marker_for_array!(
  AnyBitPattern,
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  48,
  64,
  96,
  128,
  256,
  512,
  1024,
  2048,
  4096
);

#[cfg(all(target_arch = "wasm32", feature = "wasm_simd"))]
unsafe impl AnyBitPattern for wasm32::v128 {}

unsafe impl AnyBitPattern for () {}
unsafe impl AnyBitPattern for u8 {}
unsafe impl AnyBitPattern for i8 {}
unsafe impl AnyBitPattern for u16 {}
unsafe impl AnyBitPattern for i16 {}
unsafe impl AnyBitPattern for u32 {}
unsafe impl AnyBitPattern for i32 {}
unsafe impl AnyBitPattern for u64 {}
unsafe impl AnyBitPattern for i64 {}
unsafe impl AnyBitPattern for usize {}
unsafe impl AnyBitPattern for isize {}
unsafe impl AnyBitPattern for u128 {}
unsafe impl AnyBitPattern for i128 {}
unsafe impl AnyBitPattern for f32 {}
unsafe impl AnyBitPattern for f64 {}
unsafe impl<T: AnyBitPattern> AnyBitPattern for Wrapping<T> {}

#[cfg(feature = "unsound_ptr_pod_impl")]
unsafe impl<T: 'static> AnyBitPattern for *mut T {}

#[cfg(feature = "unsound_ptr_pod_impl")]
unsafe impl<T: 'static> AnyBitPattern for *const T {}

unsafe impl<T: AnyBitPattern> AnyBitPattern for PhantomData<T> {}

unsafe impl AnyBitPattern for PhantomPinned {}

unsafe impl<T: AnyBitPattern> AnyBitPattern for ManuallyDrop<T> {}

// Note(Lokathor): MaybeUninit can NEVER be AnyBitPattern.

#[cfg(all(target_arch = "wasm32", feature = "wasm_simd"))]
unsafe impl AnyBitPattern for wasm32::v128 {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float32x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float32x2x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float32x2x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float32x2x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float32x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float32x4x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float32x4x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float32x4x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float64x1_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float64x1x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float64x1x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float64x1x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float64x2_t {}
#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float64x2x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float64x2x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::float64x2x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int16x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int16x4x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int16x4x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int16x4x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int16x8_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int16x8x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int16x8x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int16x8x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int32x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int32x2x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int32x2x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int32x2x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int32x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int32x4x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int32x4x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int32x4x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int64x1_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int64x1x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int64x1x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int64x1x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int64x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int64x2x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int64x2x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int64x2x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int8x16_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int8x16x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int8x16x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int8x16x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int8x8_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int8x8x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int8x8x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::int8x8x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly16x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly16x4x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly16x4x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly16x4x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly16x8_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly16x8x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly16x8x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly16x8x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly64x1_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly64x1x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly64x1x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly64x1x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly64x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly64x2x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly64x2x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly64x2x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly8x16_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly8x16x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly8x16x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly8x16x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly8x8_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly8x8x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly8x8x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::poly8x8x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint16x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint16x4x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint16x4x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint16x4x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint16x8_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint16x8x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint16x8x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint16x8x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint32x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint32x2x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint32x2x3_t {}
#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint32x2x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint32x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint32x4x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint32x4x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint32x4x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint64x1_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint64x1x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint64x1x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint64x1x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint64x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint64x2x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint64x2x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint64x2x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint8x16_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint8x16x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint8x16x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint8x16x4_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint8x8_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint8x8x2_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint8x8x3_t {}

#[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
unsafe impl AnyBitPattern for aarch64::uint8x8x4_t {}

#[cfg(target_arch = "x86")]
unsafe impl AnyBitPattern for x86::__m128i {}

#[cfg(target_arch = "x86")]
unsafe impl AnyBitPattern for x86::__m128 {}

#[cfg(target_arch = "x86")]
unsafe impl AnyBitPattern for x86::__m128d {}

#[cfg(target_arch = "x86")]
unsafe impl AnyBitPattern for x86::__m256i {}

#[cfg(target_arch = "x86")]
unsafe impl AnyBitPattern for x86::__m256 {}

#[cfg(target_arch = "x86")]
unsafe impl AnyBitPattern for x86::__m256d {}

#[cfg(target_arch = "x86_64")]
unsafe impl AnyBitPattern for x86_64::__m128i {}

#[cfg(target_arch = "x86_64")]
unsafe impl AnyBitPattern for x86_64::__m128 {}

#[cfg(target_arch = "x86_64")]
unsafe impl AnyBitPattern for x86_64::__m128d {}

#[cfg(target_arch = "x86_64")]
unsafe impl AnyBitPattern for x86_64::__m256i {}

#[cfg(target_arch = "x86_64")]
unsafe impl AnyBitPattern for x86_64::__m256 {}

#[cfg(target_arch = "x86_64")]
unsafe impl AnyBitPattern for x86_64::__m256d {}

#[cfg(feature = "nightly_portable_simd")]
unsafe impl<T, const N: usize> AnyBitPattern for core::simd::Simd<T, N>
where
  T: core::simd::SimdElement + AnyBitPattern,
  core::simd::LaneCount<N>: core::simd::SupportedLaneCount,
{
}
