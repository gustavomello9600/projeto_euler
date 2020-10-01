fn main() {
    let s = String::from("12345");
    let vec = s.chars().collect::<Vec<char>>();
    println!("{:?}", s.chars().collect::<Vec<char>>());

    for c in &vec[1..4] {
        println!("{:?}", c);
    }

    let n = 3;
    let m: u64 = n as u64;
}
