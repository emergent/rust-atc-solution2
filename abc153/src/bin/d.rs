use proconio::input;

fn main() {
    input! {
    mut h:u64
    }

    let mut ans: u64 = 1;
    let mut x = 1;
    while h > 1 {
        h /= 2;
        x = x * 2;
        ans += x;
    }
    println!("{}", ans)
}
