
use std::ops;
use std::time::Instant;
use std::convert::identity;
use num_bigint::BigUint;
use num_traits::Zero;
use ramp::int::Int;
use primal::Primes;

/// A possibly stack-based big-integer
#[derive(Clone)]
enum Number {
    Local(usize),
    Heap(Int)
}
impl Number {
    fn small(n: usize) -> Number {
        Number::Local(n)
    }
    fn big(i: Int) -> Number {
        Number::Heap(i)
    }

    fn square(self) -> Number {
        use Number::{Local, Heap};
        match self {
            Local(n) => {
                match n.checked_pow(2) {
                    Some(n) => Local(n),
                    None => Heap(Int::from(n).dsquare())
                }
            },
            Heap(h) => {
                Heap(h.dsquare())
            }
        }
    }
    fn multiple19(&self) -> bool {
        use Number::{Local, Heap};
        match self {
            Local(n) => n % 19 == 0,
            Heap(h) => h % 19 == 0,
        }
    }
    fn promote(&mut self) -> &mut Int {
        if let Number::Local(n) = self {
            *self = Number::Heap(Int::from(*n));
        }
        if let Number::Heap(h) = self {
            return h;
        } else {
            unreachable!()
        }
    }
}
impl ops::AddAssign<&Number> for &mut Number {
    fn add_assign(&mut self, other: &Number) {
        use Number::{Local, Heap};

        match self {
            Heap(h) => match other {
                Heap(oh) => { *h += oh; },
                Local(on) => { *h += *on; },
            },
            Local(_) => {
                let h = self.promote();
                match other {
                    Heap(oh) => { *h += oh; },
                    Local(on) => { *h += *on; },
                }
            }
        }
    }
}
impl ops::Rem<usize> for &Number {
    type Output = usize;
    fn rem(self, modulus: usize) -> usize {
        use Number::{Local, Heap};
        match self {
            Local(n) => n % modulus,
            Heap(h) => (&(h % modulus)).into()
        }
    }
}

/*
impl ops::AddAssign<&Number> for Int {
    fn add_assign(&mut self, other: &Number) {
        match other {
            Number::Local(n) => *self += *n,
            Number::Heap(h) => *self += h,
        }
    }
}*/

/*
impl ops::Add<&Number> for Number {
    type Output = Number;

    /// Attempts to keep items as both locals if possible - otherwise promotes to bigints
    fn add(self, other: &Number) -> Self {
        use Number::{Local, Heap};

        match (self, other) {
            (Heap(h1), Heap(h2)) => { Number::Heap(h1 + h2) },
            (Heap(h), Local(n)) => { Number::Heap(h + *n) },
            (Local(n), Heap(h)) => { Number::Heap(h + n) },
            (Local(n1), Local(n2)) => {
                if let Some(s) = n1.checked_add(*n2) {
                    Number::Local(s)
                } else {
                    Number::Heap(Int::from(n1) + *n2)
                }
            },
        }
    }
}
*/

fn main() {
    let timer = Instant::now();

    let mut iter = Primes::all()
        .scan(Int::from(0 as usize), |state, n| -> Option<Int> { // return the collective sums
            // add a squared prime to a running sum
            if let Some(n) = n.checked_pow(2) {
                *state += n;
            } else {
                // more expensive - allocates memory
                *state += Int::from(n).dsquare();
            }
            Some(state.clone())
        })
        .enumerate() // get the prime's number
        .filter(|(i, n)| n % (i+1) == 0); // filter only the ones we want

    while let Some((i, _)) = iter.next() {
        println!("[{: >8}ms] {}",
            Instant::now().duration_since(timer).as_millis(),
            i+1
        );
    }
}

