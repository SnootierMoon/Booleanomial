use inner::{BoolyOrder, GoodBooly};

mod inner {
    macro_rules! impl_booly_order {
        ($(($x:expr, $y:expr)),*) => {
            $(
                impl GoodBooly for BoolyOrder<$y> {
                    const VALUE: usize = $x;
                }
            )*
        };
    }

    /// This trait is used with `BoolyOrder` to determine how many variables a `Booleanomial` has.
    pub trait GoodBooly {
        const VALUE: usize;
    }

    pub struct BoolyOrder<const N: usize> {}

    impl_booly_order!(
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

/// A Booleanomial of `N` terms.
///
/// Has `N` terms, and `log2(N)` variables. For example, a `Booleanomial<4>` has four terms and two
/// variables, so it can represent a XOR b, or -2ab + a + b.
///
/// A Booleanomial of `X` variables only needs `2^X` terms because powers of variables, such as
/// ab^2, simplify to ab due to the fact that b^2=b is true for all values of b (b=0 and b=1).
/// Therefore, every term can either have each variable only once or not at all.
pub struct Booleanomial<const N: usize>
where
    BoolyOrder<N>: GoodBooly,
{
    /// The coefficients of the N terms
    ///
    /// The kth bit of each index in coeffs is 1 if the term it corresponds to contains the kth
    /// variable. For example, if the term is 2abc, the index is 111b = 7, so coeffs\[7]=2. If the
    /// term is 3ac, the index = 101b = 5, so coeffs\[5] = 3.
    coeffs: Box<[i32; N]>,
}

impl<const N: usize> Booleanomial<N>
where
    BoolyOrder<N>: GoodBooly,
{
    /// Create a new `booleanomial` with a value of 0 (y = 0)
    pub fn new_false() -> Self {
        Self {
            coeffs: Box::new([0; N]),
        }
    }

    /// Create a new `booleanomial` which is 1 when the zth variable is 1 (y = z)
    pub fn new(z: usize) -> Self {
        let mut ret = Self::new_false();
        ret.coeffs[1 << z] = 1;
        ret
    }

    /// Compute the product of two booleanomials
    fn mul(&self, other: &Self) -> Self {
        let mut ret = Self::new_false();
        for i in 0..N {
            for j in 0..N {
                // Using the fact that the bits in the index correspond to which variables are in
                // the term, we can use the bitwise or of the two indices that we are multiplying
                // to find the destination index in `ret`. For example, for 3ab + 2bc = 6abc,
                // 011 | 110 = 111.
                ret.coeffs[i | j] += self.coeffs[i] * other.coeffs[j]
            }
        }
        ret
    }

    /// Calculate the opposite of this booleanomial (1 -> 0, 0 -> 1).
    pub fn not(&self) -> Self {
        // Implemented by doing X = 1 - X
        let mut ret = Self::new_false();
        // negate the constant term and add 1
        ret.coeffs[0] = 1 - self.coeffs[0];
        // negate all other terms
        for i in 1..N {
            ret.coeffs[i] = -self.coeffs[i]
        }
        ret
    }

    // Calculate the booleanomial that is 1 if both input booleanomials are 1.
    pub fn and(&self, other: &Self) -> Self {
        // a AND b = ab. 0(0) = 0, 0(1) = 0, 1(0) = 0, 1(1) = 1
        self.mul(other)
    }

    // Calculate the booleanomial that is 0 if both input booleanomials are 0.
    pub fn or(&self, other: &Self) -> Self {
        // a OR b = -ab + a + b
        let mut ret = self.mul(other);
        for i in 0..N {
            ret.coeffs[i] = -ret.coeffs[i] + self.coeffs[i] + other.coeffs[i]
        }
        ret
    }

    // Calculate the booleanomial that is 1 if exactly one input booleanomials is 1.
    pub fn xor(&self, other: &Self) -> Self {
        // a XOR b = -2ab + a + b
        let mut ret = self.mul(other);
        for i in 0..N {
            ret.coeffs[i] = -2 * ret.coeffs[i] + self.coeffs[i] + other.coeffs[i]
        }
        ret
    }
}

impl<const N: usize> std::fmt::Display for Booleanomial<N>
where
    BoolyOrder<N>: GoodBooly,
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
