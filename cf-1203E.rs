#![allow(unused_imports)]
use std::io::{self, Write};
use std::str;


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


fn main() {
    let (stdin, stdout) = (io::stdin(),  io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    //let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i32>();
    let mut x: usize;
    let mut a: [i32; 150002] = [0; 150002];

    for i in 0..n {
        x = scan.token::<usize>();
        a[x] += 1;
    }

    for i in 1..150001 {
        if a[i] > 0 {
            if i > 1 && a[i-1] == 0 {
                a[i-1] += 1;
                a[i] -= 1;
            }
            if a[i] > 1 {
                a[i+1] += 1;
                a[i] -= 1;
            }
        }
    }

    let mut ans: i32 = 0;
    for i in 1..150002 {
        ans += (a[i] > 0) as i32;
    }
    println!("{}", ans);
    //writeln!(out, "{}", ans).ok();
}
