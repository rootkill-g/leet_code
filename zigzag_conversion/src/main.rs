use handle_input::*;
use std::{io::{prelude::*, self, BufWriter}, cmp::min};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = BufWriter::new(stdout.lock());
    let loop_var: usize = scan.input();

    for _ in 0..loop_var {
        let instr: String = scan.input::<String>();
        let num_rows: usize = scan.input();
        let fin_string: String = convert(instr, num_rows);

        writeln!(out, "{fin_string}").ok();
    }
}

fn convert(s: String, num_rows: usize) -> String {
    if num_rows == 1 {
        s
    } else {
        let mut rows: Vec<String> = vec![String::new(); min(num_rows, s.len())];

        let mut curow: usize = 0;
        let mut going_down: bool = false;

        for c in s.chars() {
            rows[curow].push(c);

            if (curow == 0) || (curow == num_rows -1) {
                going_down = !going_down;
            }

            if going_down { curow += 1; } else { curow -= 1; }

        }
            let mut ret = String::new();

            for row in rows.iter() {
                ret += row;
            }

            ret
    }
}
