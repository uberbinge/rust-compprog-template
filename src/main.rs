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

    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let a: usize = io.read();
        let b: usize = io.read();
        graph[a].push(b);
    }

    let mut visited = vec![false; n + 1];
    let mut stack = Vec::new();

    for i in 1..=n {
        if !visited[i] {
            dfs(i, &mut visited, &mut stack, &graph);
        }
    }

    while let Some(topic) = stack.pop() {
        print!("{} ", topic);
    }
}

fn dfs(node: usize, visited: &mut Vec<bool>, stack: &mut Vec<usize>, graph: &Vec<Vec<usize>>) {
    visited[node] = true;
    for &neighbor in &graph[node] {
        if !visited[neighbor] {
            dfs(neighbor, visited, stack, graph);
        }
    }
    stack.push(node);
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


