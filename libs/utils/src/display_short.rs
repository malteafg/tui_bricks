use std::fmt::{self, Display, Formatter};

pub trait DisplayShort {
    fn fmt_short(&self, f: &mut Formatter<'_>) -> fmt::Result;
}

pub struct Short<'a, T: DisplayShort + ?Sized>(pub &'a T);

impl<'a, T: DisplayShort + ?Sized> Display for Short<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt_short(f)
    }
}

pub trait DisplayShortExt: DisplayShort {
    fn short(&self) -> Short<'_, Self> {
        Short(self)
    }
}

impl<T: DisplayShort> DisplayShortExt for T {}
