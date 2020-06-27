use proconio::input;

fn main() {
    input! {
    h: u32,
    n: u32,
    a: [u32; n],
    }

    if h <= a.into_iter().sum() {
        println!("Yes");
    } else {
        println!("No");
    }
}
