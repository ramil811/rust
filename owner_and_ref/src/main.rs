fn main() {
    ownership();
    borrowing();
}

fn ownership() {
    let s1 = String::from("hello");
    println!("s1 = {}", s1);
    let s2 = add_world_ownership(s1); // ownership of s1 is moved to add_world_ownership and s1 is no longer valid and add_world_ownership returns ownership of s1 to s2
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1); // this line will not compile because s1 has been moved to s2
}

fn borrowing() {
    let mut s1 = String::from("hello"); // mut is required to make s1 mutable
    println!("s1 = {}", s1);
    add_world_borrow(&mut s1); // &mut s1 is a mutable reference to s1
    println!("s1 = {}", s1);
}

fn add_world_ownership(mut s: String) -> String { // s is now owned by this function and is mutable
    // let mut s = s; // s is now mutable
    s.push_str(", world!");
    s
}

fn add_world_borrow(s: &mut String) { // s is a mutable reference to a String
    s.push_str(", world!");
}