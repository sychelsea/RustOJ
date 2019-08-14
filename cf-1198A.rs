#![allow(unused_imports)]
use std::io::{self, Write};
use std::str;
use std::vec::Vec;
use std::cmp::min;


struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}
impl<R: io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self { reader, buf_str: Vec::new(), buf_iter: "".split_ascii_whitespace() }
    }
    fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader.read_until(b'\n', &mut self.buf_str).expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn mylog(x: i32) -> usize {
    let mut k: usize = 0;
    let mut temp: usize = 1; // 2^k
    while x as usize > temp {
        k += 1;
        temp *= 2
    }
    return k;
}

fn main() {
    let stdin = io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let n = scan.token::<usize>();
    let I = scan.token::<i32>();

    let mut a = Vec::new();
    for i in 0..n {
        let x = scan.token::<i32>();
        a.push(x);
    }
    a.sort();

    let mut m: i32 = 1;
    for i in 1..n {
        if a[i] != a[i-1] {
            m += 1;
        }
    }
    let mut pre_num: [i32; 400010] = [0; 400010];
    let mut suf_num: [i32; 400010] = [0; 400010];
    for i in 1..n {
        if a[i] == a[i-1] {
            pre_num[i] = pre_num[i-1];
        } else {
            pre_num[i] = pre_num[i-1] + 1;
        }
    }
    for i in 0..n {
        suf_num[i] = m - pre_num[i] - 1;
    }

    let mut ans: usize = n + 1;
    for i in 0..n {
        if i > 0 && a[i] == a[i-1] {
            continue;
        }
        let mut l: usize = i;
        let mut r: usize = n - 1;
        while l <= r {
            let mut mid: usize = (l+r)/2;
            if n * mylog(m - pre_num[i] - suf_num[mid]) <= 8 * I as usize {
                if a[l] == a[r] {
                    mid = r;
                }
                ans = min(ans, i + (n-mid-1));
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
    }
    println!("{}", ans);
}
