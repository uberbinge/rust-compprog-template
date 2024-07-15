use std::{
    fmt::Debug,
    io::{self, BufWriter, Lines, StdinLock, StdoutLock, Write},
    str::FromStr,
    cmp::{min, max},
};

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
    let probs: Vec<usize> = io.collect(n);

    let result = solve(n, &probs);
    putln!(io, "{}", result);
}
fn solve(n: usize, probs: &[usize]) -> usize {
    let total_sum: usize = probs.iter().sum();
    let mut min_difference = usize::MAX;

    let mut sum1 = 0;
    for i in 1..n-1 {
        sum1 += probs[i-1];

        let mut sum2 = 0;
        for j in (1..=n-i).rev() {
            sum2 += probs[n-j];

            let sum3 = total_sum - sum1 - sum2;

            let max_sum = max(max(sum1, sum2), sum3);
            let min_sum = min(min(sum1, sum2), sum3);

            min_difference = min(min_difference, max_sum - min_sum);
        }
    }

    min_difference
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


