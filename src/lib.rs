use inner::{BoolyOrder, Pow2};

mod inner {
    macro_rules! impl_pow2 {
        ($(($x:expr, $y:expr)),*) => {
            $(
                impl Pow2 for BoolyOrder<$y> {
                    const VALUE: usize = $x;
                }
            )*
        };
    }

    pub trait Pow2 {
        const VALUE: usize;
    }

    pub struct BoolyOrder<const N: usize> {}

    impl_pow2!(
        (0, 1),
        (1, 2),
        (2, 4),
        (3, 8),
        (4, 16),
        (5, 32),
        (6, 64),
        (7, 128),
        (8, 256)
    );
}

pub struct Booleanomial<const N: usize>
where
    BoolyOrder<N>: Pow2,
{
    coeffs: Box<[i32; N]>,
}

impl<const N: usize> Booleanomial<N>
where
    BoolyOrder<N>: Pow2,
{
    pub fn new_false() -> Self {
        Self {
            coeffs: Box::new([0; N]),
        }
    }

    pub fn new(z: usize) -> Self {
        let mut ret = Self::new_false();
        ret.coeffs[1 << z] = 1;
        ret
    }

    fn mul(&self, other: &Self) -> Self {
        let mut ret = Self::new_false();
        for i in 0..N {
            for j in 0..N {
                ret.coeffs[i | j] += self.coeffs[i] * other.coeffs[j]
            }
        }
        ret
    }

    pub fn not(&self) -> Self {
        let mut ret = Self::new_false();
        ret.coeffs[0] = 1 - self.coeffs[0];
        for i in 1..N {
            ret.coeffs[i] = -self.coeffs[i]
        }
        ret
    }

    pub fn and(&self, other: &Self) -> Self {
        self.mul(other)
    }

    pub fn or(&self, other: &Self) -> Self {
        let mut ret = self.mul(other);
        for i in 0..N {
            ret.coeffs[i] = -ret.coeffs[i] + self.coeffs[i] + other.coeffs[i]
        }
        ret
    }

    pub fn xor(&self, other: &Self) -> Self {
        let mut ret = self.mul(other);
        for i in 0..N {
            ret.coeffs[i] = -2 * ret.coeffs[i] + self.coeffs[i] + other.coeffs[i]
        }
        ret
    }
}

impl<const N: usize> std::fmt::Display for Booleanomial<N>
where
    BoolyOrder<N>: Pow2,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut leading = true;
        if self.coeffs[0] != 0 {
            leading = false;
            write!(f, "{}", self.coeffs[0])?;
        }
        for i in 1..N {
            if self.coeffs[i] == 0 {
                continue;
            }
            let (sign, mag) = (self.coeffs[i] < 0, self.coeffs[i].abs());
            if leading {
                leading = false;
                if sign {
                    write!(f, "-")?
                }
            } else if !leading {
                if sign {
                    write!(f, " - ")?
                } else {
                    write!(f, " + ")?
                }
            }
            if mag != 1 {
                write!(f, "{}", mag)?;
            }
            for j in 0..BoolyOrder::<N>::VALUE {
                if (i >> j) & 1 != 0 {
                    let c = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
                    write!(f, "{}", c[j])?
                }
            }
        }
        if leading {
            write!(f, "0")?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Booleanomial;

    #[test]
    fn order_test() {
        let a = Booleanomial::<2>::new(0);
        assert_eq!(a.to_string(), "a");

        let a = Booleanomial::<4>::new(0);
        let b = Booleanomial::<4>::new(1);
        assert_eq!(a.to_string(), "a");
        assert_eq!(b.to_string(), "b");

        let a = Booleanomial::<8>::new(0);
        let b = Booleanomial::<8>::new(1);
        let c = Booleanomial::<8>::new(2);
        assert_eq!(a.to_string(), "a");
        assert_eq!(b.to_string(), "b");
        assert_eq!(c.to_string(), "c")
    }

    #[test]
    fn basic_op_test() {
        let a = Booleanomial::<4>::new(0);
        let b = Booleanomial::<4>::new(1);
        assert_eq!(a.not().to_string(), "1 - a");
        assert_eq!(a.and(&b).to_string(), "ab");
        assert_eq!(a.or(&b).to_string(), "a + b - ab");
        assert_eq!(a.xor(&b).to_string(), "a + b - 2ab")
    }

    #[test]
    fn complex_op_test() {
        let a = Booleanomial::<8>::new(0);
        let b = Booleanomial::<8>::new(1);
        let c = Booleanomial::<8>::new(2);
        assert_eq!(
            a.xor(&b).xor(&c).to_string(),
            "a + b - 2ab + c - 2ac - 2bc + 4abc"
        );
        assert_eq!(
            a.and(&b).or(&c.and(&a.xor(&b))).to_string(),
            "ab + ac + bc - 2abc"
        )
    }
}
