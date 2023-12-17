use std::fs::File;
use std::io::BufRead;
use std::io::Write;

fn main() {
    let f = File::open("../values.txt").expect("couldn't find input file");
    let mut new_f = File::create("../output.txt").expect("couldn't find input file");
    let buffer = std::io::BufReader::new(f).lines();
    let mut next_rank = 2;

    buffer.for_each(|l| match l {
        Ok(s) => {
            let mut values = s.split("values");
            let pre = values.next().unwrap();
            let suf = values.next().unwrap();
            println!("{}", pre);
            println!("{}", suf);
            // fix the rank
            let v: Vec<&str> = suf.splitn(3, ",").collect();
            // get the rank substring
            let mut rank = v.get(1).unwrap().to_string();
            println!("{}", rank);
            if rank.eq("null") {
                rank = next_rank.to_string();
            }
            next_rank += 1;

            rank = rank.replace(".", "");
            rank = rank.replace(" ", "");
            let rank = format!("{}{}{}", ",", rank, ",");
            println!("{}", rank);
            let suf = format!("{}{}{}", v.get(0).unwrap(), rank, v.get(2).unwrap());

            let output = format!("{}{}{}", pre.to_owned(), "values", suf);

            // write into output file
            writeln!(new_f, "{}", output).expect("write failed");
        }
        Err(_) => {}
    });

    println!("done cleaning");
}
