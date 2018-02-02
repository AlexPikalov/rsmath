use std::marker::Sized;
use std::ops::Add;

pub trait Algebraic: Add<Output = Self>
where
    Self: Sized,
{
    fn pow(p: f64) -> Self;
}
