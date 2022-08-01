use std::fmt;

use funty::{Floating, Signed};

pub trait IntT: Signed + Copy + ToString + fmt::Debug {}
impl<T: Signed + Copy + ToString + fmt::Debug> IntT for T {}

pub trait FloatT: Floating + Copy + ToString + fmt::Debug {}
impl<T: Floating + Copy + ToString + fmt::Debug> FloatT for T {}
