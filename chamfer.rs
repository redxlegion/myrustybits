use std::io;

fn chamf(d: f64, a: f64, b: f64) -> f64 {
    ((a.to_radians().tan() * d) * 2.0) + b
}

fn seek(p: &str) -> f64 {
    let mut buffer = String::new();
    println!("\n{}", p);
    io::stdin().read_line(&mut buffer)
               .ok()
               .expect("Failed to read line!");
    buffer.trim().parse::<f64>()
                 .ok()
                 .expect("Not a float!")
}

fn main() {
    let x = seek("Enter Min Depth: ");

    let y = seek("Enter Min Angle: ");

    let z = seek("Enter Min Bore: ");

    let i = seek("Enter Max Depth: ");

    let j = seek("Enter Max Angle: ");

    let k = seek("Enter Max Bore: ");

    println!("\nMin Result:\n{} mm\n{} in", chamf(x, y, z), chamf(x, y, z)/25.4); 
    println!("\nMax Result:\n{} mm\n{} in", chamf(i, j, k), chamf(i, j, k)/25.4);
}
