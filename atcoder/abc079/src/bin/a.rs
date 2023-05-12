use proconio::{input, marker::Bytes};

fn main() {
    input! { n: Bytes, }
    println!("{}", if n[0] == n[1] && n[1] == n[2] || n[1] == n[2] && n[2] == n[3] {
       "Yes" 
    } else {
        "No"
    })
}
