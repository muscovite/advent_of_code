// From nrxus
// https://github.com/nrxus/advent-of-code/blob/master/common/src/macros.rs
#[macro_export]
macro_rules! read_main {
    () => {
        fn main() {
            use std::io::Read;

            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            println!("{}", solve(input.as_str()));
        }
    };
}
