fn main() {
    let mut a: i32 = 2;
    let mut b: i32 = 4;
    println!("a = {}, b = {}", a, b);
    // print memory addresses of a and b
    println!("a = {:p}, b = {:p}", &a, &b);
    flip_numbers(&mut a, &mut b);
    println!("a = {:p}, b = {:p}", &a, &b);
    println!("a = {}, b = {}", a, b);
}

// write a function to flip the values of two variables
fn flip_numbers(num1: &mut i32, num2: &mut i32) {
    println!("num1 = {:p}, num2 = {:p}", num1, num2);
    let c = *num1;
    *num1 = *num2;
    *num2 = c;
}