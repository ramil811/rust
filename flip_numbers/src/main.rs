fn main() {
    let mut a: i32 = 2;
    let mut b: i32 = 4;
    println!("a = {}, b = {}", a, b);
    // print memory addresses of a and b
    println!("a = {:p}, b = {:p}", &a, &b);
    flip_numbers(&mut a, &mut b);
    println!("a = {:p}, b = {:p}", &a, &b);
    println!("a = {}, b = {}", a, b);

    let mut c: i32 = 2;
    let mut d: i32 = 4;
    println!("c = {}, d = {}", c, d);
    // print memory addresses of c and d
    println!("c = {:p}, d = {:p}", &c, &d);
    flip_numbers_inbuilt(&mut c, &mut d);
    println!("c = {:p}, d = {:p}", &c, &d);
    println!("c = {}, d = {}", c, d);
}

// write a function to flip the values of two variables
fn flip_numbers(num1: &mut i32, num2: &mut i32) {
    println!("num1 = {:p}, num2 = {:p}", num1, num2);
    let c = *num1;
    *num1 = *num2;
    *num2 = c;
}

fn flip_numbers_inbuilt(num1: &mut i32, num2: &mut i32) {
    std::mem::swap(num1, num2);
}