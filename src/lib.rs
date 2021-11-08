use std::ops::{Add, Deref, Div, Mul, Rem, Sub};

trait GenericAdd<TOther, TOutput> {
    fn generic_add(self, other: TOther) -> TOutput;
}

trait GenericSub<TOther, TOutput> {
    fn generic_sub(self, other: TOther) -> TOutput;
}

trait GenericMul<TOther, TOutput> {
    fn generic_mul(self, other: TOther) -> TOutput;
}

trait GenericDiv<TOther, TOutput> {
    fn generic_div(self, other: TOther) -> TOutput;
}

trait GenericRem<TOther, TOutput> {
    fn generic_rem(self, other: TOther) -> TOutput;
}

// checked

trait GenericCheckedAdd<TOther, TOutput> {
    fn generic_checked_add(self, other: TOther) -> Option<TOutput>;
}

trait GenericCheckedSub<TOther, TOutput> {
    fn generic_checked_sub(self, other: TOther) -> Option<TOutput>;
}

trait GenericCheckedMul<TOther, TOutput> {
    fn generic_checked_mul(self, other: TOther) -> Option<TOutput>;
}

trait GenericCheckedDiv<TOther, TOutput> {
    fn generic_checked_div(self, other: TOther) -> Option<TOutput>;
}

trait GenericCheckedRem<TOther, TOutput> {
    fn generic_checked_rem(self, other: TOther) -> Option<TOutput>;
}

macro_rules! impl_generics {
    // Lhs == Rhs
    (add; $t: ty) => {
        impl GenericAdd<$t, $t> for $t {
            #[inline]
            fn generic_add(self, other: $t) -> $t {
                self.add(other)
            }
        }

        impl GenericSub<$t, $t> for $t {
            #[inline]
            fn generic_sub(self, other: $t) -> $t {
                self.sub(other)
            }
        }

        impl GenericMul<$t, $t> for $t {
            #[inline]
            fn generic_mul(self, other: $t) -> $t {
                self.mul(other)
            }
        }

        impl GenericDiv<$t, $t> for $t {
            #[inline]
            fn generic_div(self, other: $t) -> $t {
                self.div(other)
            }
        }

        impl GenericRem<$t, $t> for $t {
            #[inline]
            fn generic_rem(self, other: $t) -> $t {
                self.rem(other)
            }
        }

        // checked

        impl GenericCheckedAdd<$t, $t> for $t {
            #[inline]
            fn generic_checked_add(self, other: $t) -> Option<$t> {
                self.checked_add(other)
            }
        }

        impl GenericCheckedSub<$t, $t> for $t {
            #[inline]
            fn generic_checked_sub(self, other: $t) -> Option<$t> {
                self.checked_sub(other)
            }
        }

        impl GenericCheckedMul<$t, $t> for $t {
            #[inline]
            fn generic_checked_mul(self, other: $t) -> Option<$t> {
                self.checked_mul(other)
            }
        }

        impl GenericCheckedDiv<$t, $t> for $t {
            #[inline]
            fn generic_checked_div(self, other: $t) -> Option<$t> {
                self.checked_div(other)
            }
        }

        impl GenericCheckedRem<$t, $t> for $t {
            #[inline]
            fn generic_checked_rem(self, other: $t) -> Option<$t> {
                self.checked_rem(other)
            }
        }
    };

    // Lhs > Rhs
    (add_l; $tl: ty, $tr: ty) => {
        impl GenericAdd<$tr, $tl> for $tl {
            #[inline]
            fn generic_add(self, other: $tr) -> $tl {
                self.add(other as $tl)
            }
        }

        impl GenericSub<$tr, $tl> for $tl {
            #[inline]
            fn generic_sub(self, other: $tr) -> $tl {
                self.sub(other as $tl)
            }
        }

        impl GenericMul<$tr, $tl> for $tl {
            #[inline]
            fn generic_mul(self, other: $tr) -> $tl {
                self.mul(other as $tl)
            }
        }

        impl GenericDiv<$tr, $tl> for $tl {
            #[inline]
            fn generic_div(self, other: $tr) -> $tl {
                self.div(other as $tl)
            }
        }

        impl GenericRem<$tr, $tl> for $tl {
            #[inline]
            fn generic_rem(self, other: $tr) -> $tl {
                self.rem(other as $tl)
            }
        }

        // checked

        impl GenericCheckedAdd<$tr, $tl> for $tl {
            #[inline]
            fn generic_checked_add(self, other: $tr) -> Option<$tl> {
                self.checked_add(other as $tl)
            }
        }

        impl GenericCheckedSub<$tr, $tl> for $tl {
            #[inline]
            fn generic_checked_sub(self, other: $tr) -> Option<$tl> {
                self.checked_sub(other as $tl)
            }
        }

        impl GenericCheckedMul<$tr, $tl> for $tl {
            #[inline]
            fn generic_checked_mul(self, other: $tr) -> Option<$tl> {
                self.checked_mul(other as $tl)
            }
        }

        impl GenericCheckedDiv<$tr, $tl> for $tl {
            #[inline]
            fn generic_checked_div(self, other: $tr) -> Option<$tl> {
                self.checked_div(other as $tl)
            }
        }

        impl GenericCheckedRem<$tr, $tl> for $tl {
            #[inline]
            fn generic_checked_rem(self, other: $tr) -> Option<$tl> {
                self.checked_rem(other as $tl)
            }
        }
    };

    // Lhs < Rhs
    (add_r; $tl: ty, $tr: ty) => {
        impl GenericAdd<$tr, $tr> for $tl {
            #[inline]
            fn generic_add(self, other: $tr) -> $tr {
                (self as $tr).add(other)
            }
        }

        impl GenericSub<$tr, $tr> for $tl {
            #[inline]
            fn generic_sub(self, other: $tr) -> $tr {
                (self as $tr).sub(other)
            }
        }

        impl GenericMul<$tr, $tr> for $tl {
            #[inline]
            fn generic_mul(self, other: $tr) -> $tr {
                (self as $tr).mul(other)
            }
        }

        impl GenericDiv<$tr, $tr> for $tl {
            #[inline]
            fn generic_div(self, other: $tr) -> $tr {
                (self as $tr).div(other)
            }
        }

        impl GenericRem<$tr, $tr> for $tl {
            #[inline]
            fn generic_rem(self, other: $tr) -> $tr {
                (self as $tr).rem(other)
            }
        }

        // checked

        impl GenericCheckedAdd<$tr, $tr> for $tl {
            #[inline]
            fn generic_checked_add(self, other: $tr) -> Option<$tr> {
                (self as $tr).checked_add(other)
            }
        }

        impl GenericCheckedSub<$tr, $tr> for $tl {
            #[inline]
            fn generic_checked_sub(self, other: $tr) -> Option<$tr> {
                (self as $tr).checked_sub(other)
            }
        }

        impl GenericCheckedMul<$tr, $tr> for $tl {
            #[inline]
            fn generic_checked_mul(self, other: $tr) -> Option<$tr> {
                (self as $tr).checked_mul(other)
            }
        }

        impl GenericCheckedDiv<$tr, $tr> for $tl {
            #[inline]
            fn generic_checked_div(self, other: $tr) -> Option<$tr> {
                (self as $tr).checked_div(other)
            }
        }

        impl GenericCheckedRem<$tr, $tr> for $tl {
            #[inline]
            fn generic_checked_rem(self, other: $tr) -> Option<$tr> {
                (self as $tr).checked_rem(other)
            }
        }
    };
}

impl_generics!(add_l; i32, i8);
impl_generics!(add_l; i32, i16);
impl_generics!(add; i32);
impl_generics!(add_r; i32, i64);
impl_generics!(add_r; i32, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_add_i32() {
        assert_eq!(1i32.generic_add(1i8), 2i32);
        assert_eq!(1i32.generic_add(1i16), 2i32);
        assert_eq!(1i32.generic_add(1i32), 2i32);
        assert_eq!(1i32.generic_add(1i64), 2i64);
        assert_eq!(1i32.generic_add(1i128), 2i128);
    }

    #[test]
    fn generic_sub_i32() {
        assert_eq!(1i32.generic_sub(1i8), 0i32);
        assert_eq!(1i32.generic_sub(1i16), 0i32);
        assert_eq!(1i32.generic_sub(1i32), 0i32);
        assert_eq!(1i32.generic_sub(1i64), 0i64);
        assert_eq!(1i32.generic_sub(1i128), 0i128);
    }

    #[test]
    fn generic_mul_i32() {
        assert_eq!(3i32.generic_mul(2i8), 6i32);
        assert_eq!(3i32.generic_mul(2i16), 6i32);
        assert_eq!(3i32.generic_mul(2i32), 6i32);
        assert_eq!(3i32.generic_mul(2i64), 6i64);
        assert_eq!(3i32.generic_mul(2i128), 6i128);
    }

    #[test]
    fn generic_div_i32() {
        assert_eq!(7i32.generic_div(3i8), 2i32);
        assert_eq!(7i32.generic_div(3i16), 2i32);
        assert_eq!(7i32.generic_div(3i32), 2i32);
        assert_eq!(7i32.generic_div(3i64), 2i64);
        assert_eq!(7i32.generic_div(3i128), 2i128);
    }

    #[test]
    fn generic_rem_i32() {
        assert_eq!(7i32.generic_rem(3i8), 1i32);
        assert_eq!(7i32.generic_rem(3i16), 1i32);
        assert_eq!(7i32.generic_rem(3i32), 1i32);
        assert_eq!(7i32.generic_rem(3i64), 1i64);
        assert_eq!(7i32.generic_rem(3i128), 1i128);
    }

    //
    #[test]
    fn generic_checked_add_i32() {
        assert_eq!(1i32.generic_checked_add(1i8), Some(2i32));
        assert_eq!(1i32.generic_checked_add(1i16), Some(2i32));
        assert_eq!(1i32.generic_checked_add(1i32), Some(2i32));
        assert_eq!(1i32.generic_checked_add(1i64), Some(2i64));
        assert_eq!(1i32.generic_checked_add(1i128), Some(2i128));
    }

    #[test]
    fn generic_checked_sub_i32() {
        assert_eq!(1i32.generic_checked_sub(1i8), Some(0i32));
        assert_eq!(1i32.generic_checked_sub(1i16), Some(0i32));
        assert_eq!(1i32.generic_checked_sub(1i32), Some(0i32));
        assert_eq!(1i32.generic_checked_sub(1i64), Some(0i64));
        assert_eq!(1i32.generic_checked_sub(1i128), Some(0i128));
    }

    #[test]
    fn generic_checked_mul_i32() {
        assert_eq!(3i32.generic_checked_mul(2i8), Some(6i32));
        assert_eq!(3i32.generic_checked_mul(2i16), Some(6i32));
        assert_eq!(3i32.generic_checked_mul(2i32), Some(6i32));
        assert_eq!(3i32.generic_checked_mul(2i64), Some(6i64));
        assert_eq!(3i32.generic_checked_mul(2i128), Some(6i128));
    }

    #[test]
    fn generic_checked_div_i32() {
        assert_eq!(7i32.generic_checked_div(3i8), Some(2i32));
        assert_eq!(7i32.generic_checked_div(3i16), Some(2i32));
        assert_eq!(7i32.generic_checked_div(3i32), Some(2i32));
        assert_eq!(7i32.generic_checked_div(3i64), Some(2i64));
        assert_eq!(7i32.generic_checked_div(3i128), Some(2i128));
    }

    #[test]
    fn generic_checked_rem_i32() {
        assert_eq!(7i32.generic_checked_rem(3i8), Some(1i32));
        assert_eq!(7i32.generic_checked_rem(3i16), Some(1i32));
        assert_eq!(7i32.generic_checked_rem(3i32), Some(1i32));
        assert_eq!(7i32.generic_checked_rem(3i64), Some(1i64));
        assert_eq!(7i32.generic_checked_rem(3i128), Some(1i128));
    }
}
