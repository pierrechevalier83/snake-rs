extern crate num;

/// Workaround strange behaviour of % operator in rust:
/// -1 % 10 returns -1 instead of 9!!!
pub fn modulo<T>(x: T, n: T) -> T where T: num::PrimInt {
    let m = x.rem(n);
	if m < T::zero() {
		m + n
	} else {
		m
	}
}

pub fn wrap_inc(x: &mut isize, n: isize) {
    *x = modulo(*x + 1, n);
}

pub fn wrap_dec(x: &mut isize, n: isize) {
    *x = modulo(*x - 1, n);
}
