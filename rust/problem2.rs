fn main() {
    let mut n_current = 1;
    let mut n_before = 0;

    let mut s = 0;
    while n_current < 4000000 {
        if n_current % 2 == 1 {
            s += n_current;
        }
        
        let n_temp = n_current;

        n_current = n_current + n_before;
        n_before = n_temp;
    }
 
    println!("Soma: {}", s)
}
