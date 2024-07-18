use std::{
    fmt::Debug,
    io::{self, BufWriter, Lines, StdinLock, StdoutLock, Write},
    str::FromStr,
};
use std::collections::VecDeque;

#[macro_export]
macro_rules! putln {
    ($io:expr $(, $($args:tt)*)?) => {
        writeln!($io.writer $(, $($args)*)?).expect("failed to write output")
    };
}

#[macro_export]
macro_rules! put {
    ($io:expr, $($args:tt)*) => {
        write!($io.writer, $($args)*).expect("failed to write output")
    };
}

fn main() {
    let mut io = Io::new();
    let n: usize = io.read();
    let m: usize = io.read();
    let mut prices = vec![(0, 0); m];
    for i in 0..m {
        prices[i] = (io.read(), io.read());
    }
    prices.sort();

    putln!(io, "{}", max(n, &prices));
}

fn max(n: usize, prices: &[(usize, usize)]) -> usize {
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        for &(length, price) in prices {
            if length > i {
                break;
            }
            dp[i] = dp[i].max(dp[i - length] + price);
        }
    }
    dp[n]
}
struct Io {
    line: String,
    offset: usize,
    lines: Lines<StdinLock<'static>>,
    writer: BufWriter<StdoutLock<'static>>,
}

#[allow(dead_code)]
impl Io {
    fn new() -> Self {
        Self {
            line: String::new(),
            offset: 0,
            lines: io::stdin().lines(),
            writer: BufWriter::new(io::stdout().lock()),
        }
    }

    fn next_token(&mut self) -> &str {
        loop {
            if let Some(trim_len) = self.line[self.offset..].find(|c: char| !c.is_whitespace()) {
                let trimmed = &self.line[self.offset + trim_len..];
                let len = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
                self.offset += trim_len + len;
                break &trimmed[..len];
            }

            self.line = self
                .lines
                .next()
                .expect("unexpected end-of-file")
                .expect("failed to read input");
            self.offset = 0;
        }
    }

    fn read<T: FromStr>(&mut self) -> T
    where
        T::Err: Debug,
    {
        self.next_token().parse().expect("failed to parse input")
    }

    fn collect<T: FromStr, C: FromIterator<T>>(&mut self, len: usize) -> C
    where
        T::Err: Debug,
    {
        (0..len).map(|_| self.read()).collect()
    }
}


