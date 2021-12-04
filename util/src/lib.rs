// From nrxus
// https://github.com/nrxus/advent-of-code/blob/master/common/src/macros.rs
#[macro_export]
macro_rules! read_main {
    () => {
        fn main() {
            use std::io::Read;

            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            let start = std::time::Instant::now();
            let solution = solve(input.as_str());
            let elapsed = start.elapsed();
            let seconds = elapsed.as_secs();
            let millis = elapsed.subsec_millis();
            let micros = elapsed.subsec_micros() - millis * 1000;
            println!(
                "{}\n\nElapsed: {}s {}.{:03}ms",
                solution, seconds, millis, micros
            )
        }
    };
}
