use super::*;

/// Marker trait for "plain old data".
///
/// The point of this trait is that once something is marked "plain old data"
/// you can really go to town with the bit fiddling and bit casting. Therefore,
/// it's a relatively strong claim to make about a type. Do not add this to your
/// type casually.
///
/// **Reminder:** The results of casting around bytes between data types are
/// _endian dependant_. Little-endian machines are the most common, but
/// big-endian machines do exist (and big-endian is also used for "network
/// order" bytes).
///
/// ## Safety
///
/// * The type must be inhabited (eg: no
///   [Infallible](core::convert::Infallible)).
/// * The type must allow any bit pattern (e.g., no `bool` or `char`, which have
///   illegal bit patterns).
/// * The type must not contain any uninit (or padding) bytes, either in the
///   middle or on the end (eg: no `#[repr(C)] struct Foo(u8, u16)`, which has
///   padding in the middle, and also no `#[repr(C)] struct Foo(u16, u8)`, which
///   has padding on the end).
/// * The type needs to have all fields also be `Pod`.
/// * The type needs to be `repr(C)` or `repr(transparent)`. In the case of
///   `repr(C)`, the `packed` and `align` repr modifiers can be used as long as
///   all other rules end up being followed.
/// * It is disallowed for types to contain pointer types, `Cell`, `UnsafeCell`,
///   atomics, and any other forms of interior mutability.
/// * More precisely: A shared reference to the type must allow reads, and
///   *only* reads. RustBelt's separation logic is based on the notion that a
///   type is allowed to define a sharing predicate, its own invariant must hold
///   for shared references, and this predicate is the reasoning allows it to
///   deal with atomic and cells etc. We require the sharing predicate to be
///   trivial and permit only read-only access.
pub unsafe trait Pod:
  Zeroable + Copy + NoUninit + AnyBitPattern + CheckedBitPattern + 'static
{
}

#[cfg(feature = "ordered_float")]
unsafe impl<T: Pod> Pod for ordered_float::OrderedFloat<T> {}

#[cfg(feature = "ordered_float")]
unsafe impl<T: Pod> CheckedBitPattern for ordered_float::OrderedFloat<T> {
  type Bits = Self;
}

#[cfg(feature = "chrono")]
unsafe impl<Tz: chrono::TimeZone + 'static> Pod for chrono::DateTime<Tz> where
  Self: Copy
{
}

#[cfg(feature = "chrono")]
unsafe impl<Tz: chrono::TimeZone + 'static> CheckedBitPattern
  for chrono::DateTime<Tz>
where
  Self: Copy,
{
  type Bits = Self;
}

#[cfg(feature = "chrono")]
unsafe impl<Tz: chrono::TimeZone + 'static> NoUninit for chrono::DateTime<Tz> where
  Self: Copy
{
}

#[cfg(feature = "chrono")]
unsafe impl<Tz: chrono::TimeZone + 'static> Zeroable for chrono::DateTime<Tz> where
  Self: Copy
{
}

#[cfg(feature = "chrono")]
unsafe impl<Tz: chrono::TimeZone + 'static> AnyBitPattern
  for chrono::DateTime<Tz>
where
  Self: Copy,
{
}

#[cfg(feature = "ordered_float")]
unsafe impl<T: AnyBitPattern> AnyBitPattern for ordered_float::OrderedFloat<T> {}

#[cfg(feature = "ordered_float")]
unsafe impl<T: Zeroable> Zeroable for ordered_float::OrderedFloat<T> {}

#[cfg(feature = "ordered_float")]
unsafe impl<T: NoUninit> NoUninit for ordered_float::OrderedFloat<T> {}

unsafe impl Pod for () {}
unsafe impl CheckedBitPattern for () {
  type Bits = ();
}

unsafe impl NoUninit for () {}

unsafe impl Pod for u8 {}
unsafe impl NoUninit for u8 {}

unsafe impl CheckedBitPattern for i8 {
  type Bits = i8;
}
unsafe impl Pod for i8 {}
unsafe impl NoUninit for i8 {}

unsafe impl CheckedBitPattern for u16 {
  type Bits = u16;
}
unsafe impl Pod for u16 {}
unsafe impl NoUninit for u16 {}

unsafe impl CheckedBitPattern for i16 {
  type Bits = Self;
}
unsafe impl Pod for i16 {}
unsafe impl NoUninit for i16 {}

unsafe impl CheckedBitPattern for u32 {
  type Bits = Self;
}
unsafe impl Pod for u32 {}
unsafe impl NoUninit for u32 {}

unsafe impl CheckedBitPattern for i32 {
  type Bits = Self;
}
unsafe impl Pod for i32 {}
unsafe impl NoUninit for i32 {}

unsafe impl CheckedBitPattern for u64 {
  type Bits = Self;
}
unsafe impl Pod for u64 {}
unsafe impl NoUninit for u64 {}

unsafe impl CheckedBitPattern for i64 {
  type Bits = Self;
}
unsafe impl Pod for i64 {}
unsafe impl NoUninit for i64 {}

unsafe impl CheckedBitPattern for usize {
  type Bits = Self;
}
unsafe impl Pod for usize {}
unsafe impl NoUninit for usize {}

unsafe impl CheckedBitPattern for isize {
  type Bits = Self;
}
unsafe impl Pod for isize {}
unsafe impl NoUninit for isize {}

unsafe impl CheckedBitPattern for u128 {
  type Bits = Self;
}
unsafe impl Pod for u128 {}
unsafe impl NoUninit for u128 {}

unsafe impl CheckedBitPattern for i128 {
  type Bits = Self;
}
unsafe impl Pod for i128 {}
unsafe impl NoUninit for i128 {}

unsafe impl CheckedBitPattern for f32 {
  type Bits = Self;
}
unsafe impl Pod for f32 {}
unsafe impl NoUninit for f32 {}

unsafe impl CheckedBitPattern for f64 {
  type Bits = Self;
}
unsafe impl Pod for f64 {}
unsafe impl NoUninit for f64 {}

unsafe impl<T: Pod> CheckedBitPattern for Wrapping<T> {
  type Bits = Self;
}
unsafe impl<T: Pod> Pod for Wrapping<T> {}
unsafe impl<T: NoUninit> NoUninit for Wrapping<T> {}

#[cfg(feature = "unsound_ptr_pod_impl")]
unsafe impl<T: 'static> Pod for *mut T {}

#[cfg(feature = "unsound_ptr_pod_impl")]
unsafe impl<T: 'static> CheckedBitPattern for *mut T {
  type Bits = Self;
}

#[cfg(feature = "unsound_ptr_pod_impl")]
unsafe impl<T: 'static> NoUninit for *mut T {}

#[cfg(feature = "unsound_ptr_pod_impl")]
unsafe impl<T: 'static> Pod for *const T {}
#[cfg(feature = "unsound_ptr_pod_impl")]

unsafe impl<T: 'static> CheckedBitPattern for *const T {
  type Bits = Self;
}
#[cfg(feature = "unsound_ptr_pod_impl")]
unsafe impl<T: 'static> NoUninit for *const T {}

#[cfg(feature = "unsound_ptr_pod_impl")]
unsafe impl<T: 'static> PodInOption for NonNull<T> {}

#[cfg(feature = "unsound_ptr_pod_impl")]
unsafe impl<T: 'static> AnyBitPattern for NonNull<T> {}

#[cfg(feature = "unsound_ptr_pod_impl")]
unsafe impl<T: 'static> Zeroable for NonNull<T> {}

#[cfg(feature = "unsound_ptr_pod_impl")]
unsafe impl<T: 'static> CheckedBitPattern for NonNull<T> {
  type Bits = Self;
}

#[cfg(feature = "unsound_ptr_pod_impl")]
unsafe impl<T: 'static> NoUninit for NonNull<T> {}

unsafe impl<T: Pod> Pod for PhantomData<T> {}
unsafe impl<T: NoUninit> NoUninit for PhantomData<T> {}
unsafe impl<T: Pod> CheckedBitPattern for PhantomData<T> {
  type Bits = Self;
}

unsafe impl Pod for PhantomPinned {}
unsafe impl NoUninit for PhantomPinned {}
unsafe impl CheckedBitPattern for PhantomPinned {
  type Bits = Self;
}

unsafe impl<T: Pod> Pod for ManuallyDrop<T> {}
unsafe impl<T: NoUninit> NoUninit for ManuallyDrop<T> {}
unsafe impl<T: Pod> CheckedBitPattern for ManuallyDrop<T> {
  type Bits = Self;
}

// Note(Lokathor): MaybeUninit can NEVER be Pod.

#[cfg(feature = "min_const_generics")]
unsafe impl<T, const N: usize> Pod for [T; N] where T: Pod {}

#[cfg(feature = "min_const_generics")]
unsafe impl<T, const N: usize> NoUninit for [T; N] where T: NoUninit {}

#[cfg(all(target_arch = "wasm32", feature = "wasm_simd"))]
unsafe impl Pod for wasm32::v128 {}

#[cfg(all(target_arch = "wasm32", feature = "wasm_simd"))]
unsafe impl NoUninit for wasm32::v128 {}

macro_rules! aarch64 {
  ($($kind: ident), *) => {
    $(
    #[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
    unsafe impl Pod for aarch64::$kind {}

    #[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
    unsafe impl NoUninit for aarch64::$kind {}

    #[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
    unsafe impl CheckedBitPattern for aarch64::$kind {
      type Bits = Self;
    }

    #[cfg(all(target_arch = "aarch64", feature = "aarch64_simd"))]
    unsafe impl AnyBitPattern for aarch64::$kind {}
    )*
  };
}

aarch64!(
  float32x2x2_t,
  float32x2x3_t,
  float32x2x4_t,
  float32x4_t,
  float32x4x2_t,
  float32x4x3_t,
  float32x4x4_t,
  float64x1_t,
  float64x1x2_t,
  float64x1x3_t,
  float64x1x4_t,
  float64x2_t,
  float64x2x2_t,
  float64x2x3_t,
  float64x2x4_t,
  int16x4_t,
  int16x4x2_t,
  int16x4x3_t,
  int16x4x4_t,
  int16x8_t,
  int16x8x2_t,
  int16x8x3_t,
  int16x8x4_t,
  int32x2_t,
  int32x2x2_t,
  int32x2x3_t,
  int32x2x4_t,
  int32x4_t,
  int32x4x2_t,
  int32x4x3_t,
  int32x4x4_t,
  int64x1_t,
  int64x1x2_t,
  int64x1x3_t,
  int64x1x4_t,
  int64x2_t,
  int64x2x2_t,
  int64x2x3_t,
  int64x2x4_t,
  int8x16_t,
  int8x16x2_t,
  int8x16x3_t,
  int8x16x4_t,
  int8x8_t,
  int8x8x2_t,
  int8x8x3_t,
  int8x8x4_t,
  poly16x4_t,
  poly16x4x2_t,
  poly16x4x3_t,
  poly16x4x4_t,
  poly16x8_t,
  poly16x8x2_t,
  poly16x8x3_t,
  poly16x8x4_t,
  poly64x1_t,
  poly64x1x2_t,
  poly64x1x3_t,
  poly64x1x4_t,
  poly64x2_t,
  poly64x2x2_t,
  poly64x2x3_t,
  poly64x2x4_t,
  poly8x16_t,
  poly8x16x2_t,
  poly8x16x3_t,
  poly8x16x4_t,
  poly8x8_t,
  poly8x8x2_t,
  poly8x8x3_t,
  poly8x8x4_t,
  uint16x4_t,
  uint16x4x2_t,
  uint16x4x3_t,
  uint16x4x4_t,
  uint16x8_t,
  uint16x8x2_t,
  uint16x8x3_t,
  uint16x8x4_t,
  uint32x2_t,
  uint32x2x2_t,
  uint32x2x3_t,
  uint32x2x4_t,
  uint32x4_t,
  uint32x4x2_t,
  uint32x4x3_t,
  uint32x4x4_t,
  uint64x1_t,
  uint64x1x2_t,
  uint64x1x3_t,
  uint64x1x4_t,
  uint64x2_t,
  uint64x2x2_t,
  uint64x2x3_t,
  uint64x2x4_t,
  uint8x16_t,
  uint8x16x2_t,
  uint8x16x3_t,
  uint8x16x4_t,
  uint8x8_t,
  uint8x8x2_t,
  uint8x8x3_t,
  uint8x8x4_t
);

macro_rules! x84 {
  ($($kind: ident), *) => {
    $(
    #[cfg(target_arch = "x86")]
    unsafe impl Pod for x86::$kind {}

    #[cfg(target_arch = "x86")]
    unsafe impl NoUninit for x86::$kind {}

    #[cfg(target_arch = "x86")]
    unsafe impl CheckedBitPattern for x86::$kind {
      type Bits = Self;
    }

    #[cfg(target_arch = "x86")]
    unsafe impl AnyBitPattern for x86::$kind {}
    )*
  };
}

x84!(
  __m128i, __m128, __m128d, __m256i, __m256, __m256d, __m128i, __m128, __m128d,
  __m256i, __m256, __m256d
);

#[cfg(feature = "nightly_portable_simd")]
unsafe impl<T, const N: usize> Pod for core::simd::Simd<T, N>
where
  T: core::simd::SimdElement + Pod,
  core::simd::LaneCount<N>: core::simd::SupportedLaneCount,
{
}

#[cfg(feature = "nightly_portable_simd")]
unsafe impl<T, const N: usize> CheckedBitPattern for core::simd::Simd<T, N>
where
  T: core::simd::SimdElement + Pod,
  core::simd::LaneCount<N>: core::simd::SupportedLaneCount,
{
  type Bits = Self;
}

#[cfg(feature = "nightly_portable_simd")]
unsafe impl<T, const N: usize> NoUninit for core::simd::Simd<T, N>
where
  T: core::simd::SimdElement + NoUninit,
  core::simd::LaneCount<N>: core::simd::SupportedLaneCount,
{
}
