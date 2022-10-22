fn main() {
    let p = 210000.00;
    let r = 5.00;
    let n = 3.00;
    let radical = 1.00 - r / 100.00;
    let radical = radical as f32;
    let a = p * radical.powf(n);
    println!("Value of TV is equal to {}",a );
}