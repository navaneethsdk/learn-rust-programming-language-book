fn main() {
    // Constants aren’t just immutable by default—they’re always immutable. You aren’t allowed to use mut with constants
    const THREE: u32 = 3;

    let x = 5;
    // the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.
    let x = x + 1;
    {
        let x = x * THREE;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    /*
    DATA TYPES:
        Scalar data types:
            Integer:
                8-bit	i8	u8
                16-bit	i16	u16
                32-bit	i32	u32
                64-bit	i64	u64
                128-bit	i128	u128
                arch	isize	usize
            Floating-Point Types:
                f32 and f64
            The Boolean Type
                true/false
            The Character Type

        Compound data types:
            Tuple
                A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
            Array
                Every element of an array must have the same type - fixed number of elements
                A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.

    */

    // Compound data type examples

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    // access a tuple element directly by using a period
    let five_hundred = tup.0;
    let six_point_four = tup.1;

    // array
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // ou can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets
    let a = [3; 5];
    // access using indexing
    let first = a[0];
    let second = a[1];
}
