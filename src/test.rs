
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

    // Read input
    let n: usize = io.read();
    let m: usize = io.read();
    let q: usize = io.read();

    let mut heights: Vec<i64> = io.collect(n);

    // Create difference array
    let mut diff = vec![0; n + 1];
    for i in 0..n {
        diff[i] += heights[i];
        diff[i + 1] -= heights[i];
    }

    // Process watering operations
    for _ in 0..m {
        let l: usize = io.read();
        let r: usize = io.read();
        let d: i64 = io.read();

        diff[l - 1] += d;
        diff[r] -= d;
    }

    // Calculate prefix sum
    let mut prefix_sum = vec![0; n + 1];
    let mut current_sum = 0;
    for i in 0..n {
        current_sum += diff[i];
        prefix_sum[i + 1] = current_sum;
    }

    // Process queries
    for _ in 0..q {
        let l: usize = io.read();
        let r: usize = io.read();

        let total_height = prefix_sum[r] - prefix_sum[l - 1];
        putln!(io, "{}", total_height);
    }
}
struct Io {
    line: String,
    offset: usize,
    lines: Lines<StdinLock<'static>>,
    writer: BufWriter<StdoutLock<'static>>,
}

#[allow(dead_code)]
impl crate::Io {
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


