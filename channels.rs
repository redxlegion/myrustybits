use std::env;
use std::thread;
use std::sync::mpsc;

fn main() {
    let args: Vec<_> = env::args().collect();
    let n: u64 = args[1].parse::<u64>().unwrap();
    let m: u64 = args[2].parse::<u64>().unwrap();


    let (tx, rx) = mpsc::channel();

    for i in n..m {
        let tx = tx.clone();

        thread::spawn(move || {
            let answer = i * i;

            tx.send(answer).unwrap();
        });
    }

    for _ in n..m {
        println!("{}", rx.recv().unwrap());
    }
}
