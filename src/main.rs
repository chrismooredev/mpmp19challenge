
use std::time::{Duration, Instant};
use primal::Primes;

fn main() {
    let timer = Instant::now();

    let mut iter = Primes::all()
        .scan(0 as u128, |state, n| -> Option<u128> { // return the collective sums
            // add a squared prime to a running sum
            if let Some(n) = (n as u128).checked_pow(2) {
                *state += n as u128;
            } else {
                // switch back to bigints?
                eprintln!("overflow on n = {}", n);
                return None;
            }
            Some(*state)
        })
        .enumerate() // get the prime's number
        .filter(|(i, n)| *n % (*i as u128 + 1) == 0); // filter only the ones we want

    while let Some((i, _)) = iter.next() {
        let mut t = Instant::now() - timer;
        if t.as_secs() > 1 {
            t -= Duration::from_nanos((t.subsec_nanos() % 1_000_000) as u64);
        }
        println!("[{:.3}] {}",
            humantime::format_duration(t),
            i+1
        );
    }
}

