use core::ops::{Add, Div, Mul, Range, Sub};
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

pub trait MapRange: Sized {
    fn map_range(self, from_range: Range<Self>, to_range: Range<Self>) -> Self;
}

pub trait CheckedMapRange: Sized {
    #[must_use]
    fn checked_map_range(self, from_range: Range<Self>, to_range: Range<Self>) -> Option<Self>;
}

impl<T> MapRange for T
where
    T: Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>,
{
    fn map_range(self, from_range: Range<Self>, to_range: Range<Self>) -> Self {
        to_range.start
            + (self - from_range.start) * (to_range.end - to_range.start)
                / (from_range.end - from_range.start)
    }
}

impl<T> CheckedMapRange for T
where
    T: CheckedAdd<Output = Self>
        + CheckedSub<Output = Self>
        + CheckedMul<Output = Self>
        + CheckedDiv<Output = Self>,
{
    fn checked_map_range(self, from_range: Range<Self>, to_range: Range<Self>) -> Option<Self> {
        to_range.start.checked_add(
            &self
                .checked_sub(&from_range.start)?
                .checked_mul(&to_range.end.checked_sub(&to_range.start)?)?
                .checked_div(&from_range.end.checked_sub(&from_range.start)?)?,
        )
    }
}
