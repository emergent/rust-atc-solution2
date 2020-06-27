use proconio::input;

fn main() {
    input! {
    h: u32,
    a: u32,
    }

    println!("{}", (h + a - 1) / a);
}
