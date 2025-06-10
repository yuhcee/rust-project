fn main() {
    // In Rust, you can define variables with specific types.

    // Why do you have to specify types of numbers you have to use in rust?
    // This is because you have to commit as little as possible that you can to optimise the space your program takes up in your machine's memory.

    // A signed i8 integer ranges from -128 to 127. If the value exceeds this range, it will not compile.
    let x: i8 = 1;

    // When you do not specify the type, Rust infers it as i32.
    // If you want to use a different type, you must specify it explicitly.
    let y: i32 = 5;

    // The u32 type is an unsigned 32-bit integer, which means it can only hold non-negative values.
    let z: u32 = 10;

    let f: f32 = 10000.01;

    println!("x: {}, y: {}, z : {}, f: {}", x, y, z, f)
}
