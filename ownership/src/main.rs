fn main() {
    //Ownership rules:
    // - Each value in Rust has an owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.

    // In rust the memory is automatically returned once the variable that owns it goes out of scope
    // The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
    // taking ownership and then returning ownership this way with every function is a bit tedious
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
    // Because it does not own it, the value it points to will not be dropped when the reference stops being used.
    // We call the action of creating a reference borrowing.
    let s1 = String::from("hello");
    let len = calculate_length_by_passing_ref(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // First we change s to be mut.
    // Then we create a mutable reference with &mut s where we call the change function, and update the function signature to accept a mutable reference with some_string: &mut String.
    // This makes it very clear that the change function will mutate the value it borrows.
    // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
    // We also cannot have a mutable reference while we have an immutable one to the same value.
    let mut s = String::from("hello");
    change(&mut s);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it i
fn calculate_length_by_passing_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
