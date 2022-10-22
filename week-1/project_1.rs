fn main () {
    let p = 520000.00;
    let r = 10.00;
    let n = 5.00;
    let radical = 1.00+r/100.00;
    // f32 is used for floating point numbers
    let radical = radical as f32;
    // powf is used to raise floating point numbers to a certain power
    let a = p * radical.powf(n);
    println!("Amount is equal to {}",a );
    let ci = a - p;
    println!("Compound interest is equal to {}",ci );
}    
    