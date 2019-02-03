macro_rules! avg {
    ($($t:expr),*) => (sum!($($t),*)/count!($($t),*));
}

macro_rules! count {
    ($h:expr) => (1);
    ($h:expr, $($t:expr),*) =>
        (1 + count!($($t),*));
}

macro_rules! sum {
    ($h:expr) => ($h);
    ($h:expr, $($t:expr),*) =>
        ($h + sum!($($t),*));
}

fn main() {
    let i = avg![1,2,4,6,8,5,3,4,7,4,5,7,8,9,10];
    println!("{}", i);
}
