use proconio::input;

fn main() {
    input! {
    n: usize,
    k: usize,
    mut h: [usize; n],
    }

    if n <= k {
        println!("{}", 0);
    } else {
        h.sort();
        let mut count = 0;
        for i in 0..(n - k) {
            count += h[i];
        }
        println!("{}", count);
    }
}
