use std::env;

fn gcd(mut m: u64, mut n: u64) -> u64 {
   while m != 0 {
       let old_m = m;
       m = n % m;
       n = old_m;
   }
   n
}

fn totient(mut a: u64) -> u64 {
    let mut y = 0;
    let o = a;
    while a != 0 {
        if gcd(o, a) == 1 {
            y += 1;
        }
        a -= 1;
    }
    y
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let n: u64 = args[1].parse::<u64>().unwrap();
    let m: u64 = args[2].parse::<u64>().unwrap();

    let x = (n..m).collect::<Vec<u64>>();

    for i in &x {
        let o = totient(*i);
        println!("{:?}, {:?}", i, o);
    }
    
}
