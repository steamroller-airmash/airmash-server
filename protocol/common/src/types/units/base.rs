use std::cmp::*;
use std::fmt;
use std::ops::*;
use std::time::Duration;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use self::typedefs::*;

mod inner {
    pub trait Even {}

    pub struct IsEvenPred<const V: bool>;

    impl Even for IsEvenPred<{ true }> {}
}

type CheckEven<const X: isize> = self::inner::IsEvenPred<{ X % 2 == 0 }>;
use self::inner::Even;

/// Inner type used for all unit type declarations.
///
/// All units can be converted into this type by
/// calling the [`inner()`][0] method.
///
/// [0]: struct.AirmashUnits.html#method.inner
pub type BaseType = f32;

#[derive(Copy, Clone, Default)]
pub struct BaseUnit<
    T,
    const LENGTH: isize,
    const TIME: isize,
    const HEALTH: isize,
    const ENERGY: isize,
    const ROT: isize,
> {
    value: T,
}

pub trait UnitOps<B> {
    type Mul;
    type Div;
}

pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

#[rustfmt::skip]
mod typedefs {
	use super::*;

	pub type Unit<T>     = BaseUnit<T, {0}, {0}, {0}, {0}, {0}>;
	pub type Distance<T> = BaseUnit<T, {1}, {0}, {0}, {0}, {0}>;
	pub type Time<T>     = BaseUnit<T, {0}, {1}, {0}, {0}, {0}>;
	pub type Health<T>   = BaseUnit<T, {0}, {0}, {1}, {0}, {0}>;
	pub type Energy<T>   = BaseUnit<T, {0}, {0}, {0}, {1}, {0}>;
	pub type Rotation<T> = BaseUnit<T, {0}, {0}, {0}, {0}, {1}>;

	pub type HealthRegen<T> = <Health<T> 	as UnitOps<Time<T>>>::Div;
	pub type EnergyRegen<T> = <Energy<T> 	as UnitOps<Time<T>>>::Div;
	pub type Speed<T>		= <Distance<T> 	as UnitOps<Time<T>>>::Div;
	pub type Accel<T>		= <Speed<T> 	as UnitOps<Time<T>>>::Div;
	pub type RotationRate<T>= <Rotation<T> 	as UnitOps<Time<T>>>::Div;
}

impl<
        V,
        const L1: isize,
        const L2: isize,
        const T1: isize,
        const T2: isize,
        const H1: isize,
        const H2: isize,
        const E1: isize,
        const E2: isize,
        const R1: isize,
        const R2: isize,
    > UnitOps<BaseUnit<V, L2, T2, H2, E2, R2>> for BaseUnit<V, L1, T1, H1, E1, R1>
{
    type Mul = BaseUnit<V, { L1 + L2 }, { T1 + T2 }, { H1 + H2 }, { E1 + E2 }, { R1 + R2 }>;
    type Div = BaseUnit<V, { L1 - L2 }, { T1 - T2 }, { H1 - H2 }, { E1 - E2 }, { R1 - R2 }>;
}

impl<V, const L: isize, const T: isize, const H: isize, const E: isize, const R: isize>
    BaseUnit<V, L, T, H, E, R>
{
    pub const fn new(value: V) -> Self {
        Self { value }
    }

    pub fn into_inner(self) -> V {
        let Self { value } = self;
        value
    }

	pub fn inner(&self) -> V 
	where
		V: Clone
	{
        self.value.clone()
	}
	
	pub fn sqrt(self) -> <Self as Sqrt>::Output
	where
		Self: Sqrt
	{
		<Self as Sqrt>::sqrt(self)
	}
}

//===============================================
// Ops Traits
//===============================================

impl<
        V: Add<Output = V>,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > Add for BaseUnit<V, L, T, H, E, R>
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl<
        V: Sub<Output = V>,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > Sub for BaseUnit<V, L, T, H, E, R>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

impl<
        V: AddAssign,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > AddAssign for BaseUnit<V, L, T, H, E, R>
{
    fn add_assign(&mut self, other: Self) {
        self.value += other.into_inner();
    }
}

impl<
        V: SubAssign,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > SubAssign for BaseUnit<V, L, T, H, E, R>
{
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.into_inner();
    }
}

impl<
        V: Mul<Output = V>,
        const L1: isize,
        const L2: isize,
        const T1: isize,
        const T2: isize,
        const H1: isize,
        const H2: isize,
        const E1: isize,
        const E2: isize,
        const R1: isize,
        const R2: isize,
    > Mul<BaseUnit<V, L2, T2, H2, E2, R2>> for BaseUnit<V, L1, T1, H1, E1, R1>
{
    type Output = BaseUnit<V, { L1 + L2 }, { T1 + T2 }, { H1 + H2 }, { E1 + E2 }, { R1 + R2 }>;

    fn mul(self, other: BaseUnit<V, L2, T2, H2, E2, R2>) -> Self::Output {
        Self::Output::new(self.into_inner() * other.into_inner())
    }
}

impl<
        V: Div<Output = V>,
        const L1: isize,
        const L2: isize,
        const T1: isize,
        const T2: isize,
        const H1: isize,
        const H2: isize,
        const E1: isize,
        const E2: isize,
        const R1: isize,
        const R2: isize,
    > Div<BaseUnit<V, L2, T2, H2, E2, R2>> for BaseUnit<V, L1, T1, H1, E1, R1>
{
    type Output = BaseUnit<V, { L1 - L2 }, { T1 - T2 }, { H1 - H2 }, { E1 - E2 }, { R1 - R2 }>;

    fn div(self, other: BaseUnit<V, L2, T2, H2, E2, R2>) -> Self::Output {
        Self::Output::new(self.into_inner() / other.into_inner())
    }
}

impl<
        V: Rem<Output = V>,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > Rem for BaseUnit<V, L, T, H, E, R>
{
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self::new(self.value % other.value)
    }
}

impl<
        V: MulAssign,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > MulAssign<Unit<V>> for BaseUnit<V, L, T, H, E, R>
{
    fn mul_assign(&mut self, other: Unit<V>) {
        self.value *= other.into_inner();
    }
}

impl<
        V: DivAssign,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > DivAssign<Unit<V>> for BaseUnit<V, L, T, H, E, R>
{
    fn div_assign(&mut self, other: Unit<V>) {
        self.value /= other.into_inner();
    }
}

//========================================================
// Ops trait implementations involving the underlying type
//========================================================

impl<V: Add<Output = V>> Add<V> for Unit<V> {
    type Output = Self;

    fn add(self, other: V) -> Self {
        self + Unit::new(other)
    }
}

impl<V: Sub<Output = V>> Sub<V> for Unit<V> {
    type Output = Self;

    fn sub(self, other: V) -> Self {
        self - Unit::new(other)
    }
}

impl<
        V: Mul<Output = V>,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > Mul<V> for BaseUnit<V, L, T, H, E, R>
{
    type Output = Self;

    fn mul(self, other: V) -> Self {
        Self::new(self.value * other)
    }
}

impl<
        V: Div<Output = V>,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > Div<V> for BaseUnit<V, L, T, H, E, R>
{
    type Output = Self;

    fn div(self, other: V) -> Self {
        Self::new(self.value / other)
    }
}

impl<
        V: MulAssign,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > MulAssign<V> for BaseUnit<V, L, T, H, E, R>
{
    fn mul_assign(&mut self, other: V) {
        self.value *= other;
    }
}

impl<
        V: DivAssign,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > DivAssign<V> for BaseUnit<V, L, T, H, E, R>
{
    fn div_assign(&mut self, other: V) {
        self.value /= other;
    }
}

impl<V: Sqrt, const L: isize, const T: isize, const H: isize, const E: isize, const R: isize> Sqrt
    for BaseUnit<V, L, T, H, E, R>
where
    CheckEven<L>: Even,
    CheckEven<T>: Even,
    CheckEven<H>: Even,
    CheckEven<E>: Even,
    CheckEven<R>: Even,
{
    type Output = BaseUnit<V::Output, { L / 2 }, { T / 2 }, { H / 2 }, { E / 2 }, { R / 2 }>;

    fn sqrt(self) -> Self::Output {
        Self::Output::new(self.value.sqrt())
    }
}

impl<
        V: PartialEq,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > PartialEq for BaseUnit<V, L, T, H, E, R>
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<V: Eq, const L: isize, const T: isize, const H: isize, const E: isize, const R: isize> Eq
    for BaseUnit<V, L, T, H, E, R>
{
}

impl<
        V: PartialOrd,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > PartialOrd for BaseUnit<V, L, T, H, E, R>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<V: Ord, const L: isize, const T: isize, const H: isize, const E: isize, const R: isize> Ord
    for BaseUnit<V, L, T, H, E, R>
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

//==================================================
// Other useful impls
//==================================================

impl<const L: isize, const T: isize, const H: isize, const E: isize, const R: isize>
    BaseUnit<BaseType, L, T, H, E, R>
{
    /// Absolute value
    pub fn abs(self) -> Self {
        Self::new(self.inner().abs())
    }
    /// Get the sign of the inner value of the unit.
    pub fn signum(self) -> BaseType {
        self.inner().signum()
    }

    /// Calculate the max of two values with the same
    /// units.
    pub fn max(self, o: Self) -> Self {
        Self::new(self.inner().max(o.inner()))
    }
    /// Calculate the min of two values with the same
    /// units.
    pub fn min(self, o: Self) -> Self {
        Self::new(self.inner().min(o.inner()))
    }

    /// Combined sin and cos, can be done more
    /// efficiently then doing both calculations
    /// on their own.
    pub fn sin_cos(self) -> (BaseType, BaseType) {
        self.inner().sin_cos()
    }
    /// Calculate the sine of the inner value.
    pub fn sin(&self) -> BaseType {
        self.inner().sin()
    }
    /// Calculate the cosine of the inner value.
    pub fn cos(&self) -> BaseType {
        self.inner().cos()
    }
    /// Calculate the tangent of the inner value.
    pub fn tan(&self) -> BaseType {
        self.inner().tan()
    }
}

#[cfg(feature = "serde")]
impl<V, const L: isize, const T: isize, const H: isize, const E: isize, const R: isize> Serialize
    for BaseUnit<V, L, T, H, E, R>
where
    V: Serialize,
{
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(ser)
    }
}

#[cfg(feature = "serde")]
impl<'de, V, const L: isize, const T: isize, const H: isize, const E: isize, const R: isize>
    Deserialize<'de> for BaseUnit<V, L, T, H, E, R>
where
    V: Deserialize<'de>,
{
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::new(V::deserialize(de)?))
    }
}

impl<V, const L: isize, const T: isize, const H: isize, const E: isize, const R: isize> From<V>
    for BaseUnit<V, L, T, H, E, R>
{
    fn from(value: V) -> Self {
        Self::new(value)
    }
}

impl From<Duration> for Time<BaseType> {
    fn from(dt: Duration) -> Time<BaseType> {
        Time::new(dt.as_secs() as BaseType + 1.0e-9 * (dt.subsec_nanos() as BaseType)) * 60.0
    }
}
impl Into<Duration> for Time<BaseType> {
    fn into(self) -> Duration {
        Duration::from_nanos((self.inner() * (1.0e9 / 60.0)) as u64)
    }
}

macro_rules! display_unit {
    ($fmt:expr, $name:literal, $count:expr) => {
        match $count {
            0 => Ok(()),
            cnt => write!($fmt, " {}", cnt),
        }
    };
}

impl<
        V: fmt::Display,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > fmt::Display for BaseUnit<V, L, T, H, E, R>
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(fmt)?;

        display_unit!(fmt, "distance", L)?;
        display_unit!(fmt, "time", T)?;
        display_unit!(fmt, "health", H)?;
        display_unit!(fmt, "energy", E)?;
        display_unit!(fmt, "rotation", R)?;

        Ok(())
    }
}

impl<
        V: fmt::Debug,
        const L: isize,
        const T: isize,
        const H: isize,
        const E: isize,
        const R: isize,
    > fmt::Debug for BaseUnit<V, L, T, H, E, R>
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(fmt)?;

        display_unit!(fmt, "distance", L)?;
        display_unit!(fmt, "time", T)?;
        display_unit!(fmt, "health", H)?;
        display_unit!(fmt, "energy", E)?;
        display_unit!(fmt, "rotation", R)?;

        Ok(())
    }
}

impl Sqrt for f32 {
    type Output = Self;

    fn sqrt(self) -> Self {
        Self::sqrt(self)
    }
}

impl Sqrt for f64 {
    type Output = Self;

    fn sqrt(self) -> Self {
        Self::sqrt(self)
    }
}
