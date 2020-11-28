
use std::ops;
use std::time::Instant;
use ramp::int::Int;
use primal::Primes;

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

