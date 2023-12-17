use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let f = File::open("../values.txt").expect("couldn't find input file");
    let buffer = std::io::BufReader::new(f).lines();

    let mut new_f =
        BufWriter::new(File::create("../output.txt").expect("couldn't create output file"));

    let mut next_rank = 2;

    for l in buffer {
        if let Ok(s) = l {
            let mut values = s.split("values");
            let pre = values.next().unwrap();
            let suf = values.next().unwrap();

            let v: Vec<&str> = suf.splitn(3, ',').collect();
            let mut rank = v.get(1).unwrap().to_string();

            if rank == "null" {
                rank = next_rank.to_string();
            }
            next_rank += 1;

            rank.retain(|c| !c.is_whitespace() && c != '.');

            let suf = format!("{},{},{}", v[0], rank, v[2]);

            let output = format!("{}values{}", pre, suf);
            writeln!(new_f, "{}", output).expect("write failed");
        }
    }

    new_f.flush().expect("Failed to flush buffer");

    println!("done cleaning");
    let duration = start.elapsed();
    println!("Laufzeit: {:?}", duration);
}
