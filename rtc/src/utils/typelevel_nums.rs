// TODO: Maybe model this after something like typenum at some point to bring
// down compile times. Typenum uses essentially a type level list of
// values(Or bits) to build a number system of types from what I can
// tell.

// Also see the peano crate for a more in depths (in some regards) implementation of this.

use std::fmt;
use std::marker::PhantomData;

/// The mathematical set Z
//pub trait Int {}

/// The mathematical set N(+0)
pub trait Nat /*: Int*/ {}

pub trait NonZero {}

// pub trait Positive = NonZero + Nat; currently experimental

/// Zero
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct N0;

/// Successor of a natural number
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Succ<N: Nat> {
    _marker: PhantomData<N>,
}

impl<N: Nat + Val> fmt::Debug for Succ<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Nat: {}", Self::val())
    }
}

/*
/// Predecessor of a natural number
pub struct Pred<N: Nat> {
    _marker: PhantomData<N>,
}
*/

impl Nat for N0 {}
impl<N: Nat> Nat for Succ<N> {}
impl<N: Nat> NonZero for Succ<N> {}
//impl<I: Int> Int for Succ<I> {}
//impl<I: Int> Int for Pred<I> {}

// impl<N: Nat + NonZero> Nat for Pred<N> {}

pub type N1 = Succ<N0>;
pub type N2 = Succ<N1>;
pub type N3 = Succ<N2>;
pub type N4 = Succ<N3>;
pub type N5 = Succ<N4>;
pub type N6 = Succ<N5>;
pub type N7 = Succ<N6>;
pub type N8 = Succ<N7>;
pub type N9 = Succ<N8>;
pub type N10 = Succ<N9>;
pub type N20 = <N10 as Mul<N2>>::Output;
pub type N30 = <N10 as Mul<N3>>::Output;
pub type N40 = <N10 as Mul<N4>>::Output;
pub type N50 = <N10 as Mul<N5>>::Output;
pub type N60 = <N10 as Mul<N6>>::Output;
pub type N70 = <N10 as Mul<N7>>::Output;
pub type N80 = <N10 as Mul<N8>>::Output;
pub type N90 = <N10 as Mul<N9>>::Output;
pub type N100 = <N10 as Mul<N10>>::Output;
pub type N200 = <N100 as Mul<N2>>::Output;
pub type N300 = <N100 as Mul<N3>>::Output;
pub type N400 = <N100 as Mul<N4>>::Output;
pub type N500 = <N100 as Mul<N5>>::Output;
pub type N600 = <N100 as Mul<N6>>::Output;
pub type N700 = <N100 as Mul<N7>>::Output;
pub type N800 = <N100 as Mul<N8>>::Output;
pub type N900 = <N100 as Mul<N9>>::Output;
pub type N1000 = <N100 as Mul<N10>>::Output;
pub type N16 = <N2 as Pow<N4>>::Output;
pub type N32 = <N16 as Mul<N2>>::Output;
pub type N64 = <N32 as Mul<N2>>::Output;
pub type N128 = <N64 as Mul<N2>>::Output;
pub type N256 = <N128 as Mul<N2>>::Output;
pub type N512 = <N256 as Mul<N2>>::Output;
pub type N1024 = <N512 as Mul<N2>>::Output;

/// Extract the value-level value of a type
pub trait Val {
    fn val() -> usize;
}

impl Val for N0 {
    fn val() -> usize {
        0
    }
}

impl<N: Nat + Val> Val for Succ<N> {
    fn val() -> usize {
        N::val() + 1
    }
}

/// Calculate Lhs + Rhs for any implementation for type Lhs
pub trait Add<Rhs = Self> {
    type Output;
}

impl<Rhs: Nat> Add<Rhs> for N0 {
    type Output = Rhs;
}

impl<Rhs, Lhs> Add<Rhs> for Succ<Lhs>
where
    Rhs: Nat + Add<Lhs>,
    Lhs: Nat,
    <Rhs as Add<Lhs>>::Output: Nat,
{
    type Output = Succ<<Rhs as Add<Lhs>>::Output>;
}

/// Comparison of Rhs and whatever type it's implemented for
/// yields the bigger of both of the types
pub trait Greater<Rhs> {
    type Output;
}

impl<Rhs: Nat + NonZero> Greater<Rhs> for N0 {
    type Output = Rhs;
}

impl<Rhs: Nat + NonZero> Greater<N0> for Rhs {
    type Output = Rhs;
}

impl<Rhs, Lhs> Greater<Succ<Rhs>> for Succ<Lhs>
where
    Rhs: Nat,
    Lhs: Nat + Greater<Rhs>,
    <Lhs as Greater<Rhs>>::Output: Nat,
{
    type Output = Succ<<Lhs as Greater<Rhs>>::Output>;
}

/// Calculate Lhs * Rhs for any implementation for type Lhs
pub trait Mul<Rhs = Self> {
    type Output;
}

impl<Rhs: Nat + NonZero> Mul<Rhs> for N0 {
    type Output = N0;
}

impl<Rhs: Nat + NonZero> Mul<N0> for Rhs {
    type Output = N0;
}

impl Mul for N0 {
    type Output = N0;
}

impl<Rhs, Lhs> Mul<Succ<Rhs>> for Lhs
where
    Lhs: Nat + NonZero + Mul<Rhs> + Add<<Lhs as Mul<Rhs>>::Output>,
    Rhs: Nat,
    <Lhs as Mul<Rhs>>::Output: Nat,
    <Lhs as Add<<Lhs as Mul<Rhs>>::Output>>::Output: Nat,
{
    type Output = <Lhs as Add<<Lhs as Mul<Rhs>>::Output>>::Output;
}

/// Calculate Lhs - Rhs for any implementation for type Lhs
pub trait Sub<Rhs = Self> {
    type Output;
}

impl<Lhs: Nat> Sub<N0> for Lhs {
    type Output = Lhs;
}

impl<Rhs, Lhs> Sub<Succ<Rhs>> for Succ<Lhs>
where
    Rhs: Nat,
    Lhs: Nat + NonZero + Sub<Rhs>,
    <Lhs as Sub<Rhs>>::Output: Nat,
{
    type Output = <Lhs as Sub<Rhs>>::Output;
}

/// Calculate Lhs**Rhs (or lhs.pow(rhs) on the value level) for any type Lhs
pub trait Pow<Rhs> {
    type Output;
}

impl<Lhs: Nat + NonZero> Pow<N0> for Lhs {
    type Output = N1;
}

impl<Rhs: Nat + NonZero> Pow<Rhs> for N0 {
    type Output = N0;
}

impl<Lhs, Rhs> Pow<Succ<Rhs>> for Lhs
where
    Lhs: Nat + NonZero + Pow<Rhs> + Mul<<Lhs as Pow<Rhs>>::Output>,
    Rhs: Nat,
    <Lhs as Pow<Rhs>>::Output: Nat,
    <Lhs as Mul<<Lhs as Pow<Rhs>>::Output>>::Output: Nat,
{
    type Output = <Lhs as Mul<<Lhs as Pow<Rhs>>::Output>>::Output;
}

/*
pub trait Bool {}

#[derive(Copy, Clone, Debug)]
pub struct True;
#[derive(Copy, Clone, Debug)]
pub struct False;
impl Bool for True {}
impl Bool for False {}

pub trait IsGreater<Rhs> {
    type Output;
}

impl<Rhs: Nat, Lhs: Nat> IsGreater<Rhs> for <Rhs as Greater<Lhs>::Output> {
    type Output = <>;
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn natural() {
        assert_eq!(N8::val(), 8);
        assert_eq!(<Succ<N8>>::val(), 9);
    }

    #[test]
    fn add() {
        assert_eq!(<N7 as Add<N8>>::Output::val(), 15);
    }

    #[test]
    fn greater() {
        assert_eq!(<N0 as Greater<N1>>::Output::val(), N1::val());
        assert_eq!(<N1 as Greater<N0>>::Output::val(), N1::val());
        assert_eq!(<N8 as Greater<N5>>::Output::val(), N8::val());
    }

    #[test]
    fn mul() {
        assert_eq!(<N10 as Mul<N5>>::Output::val(), 50);
        assert_eq!(<N10 as Mul<<N10 as Mul<N10>>::Output>>::Output::val(), 1000);
    }

    #[test]
    fn sub() {
        assert_eq!(<N10 as Sub<N5>>::Output::val(), 5);
        assert_eq!(<N10 as Sub<N0>>::Output::val(), 10);
        assert_eq!(<N0 as Sub<N0>>::Output::val(), 0);
    }

    #[test]
    fn pow() {
        // assert_eq!(<N0 as Pow<N3>>::Output::val(), 0); // FIXME breaks currently, fix this
        assert_eq!(<N5 as Pow<N0>>::Output::val(), 1);
        assert_eq!(<N2 as Pow<N4>>::Output::val(), 16);
    }
}
