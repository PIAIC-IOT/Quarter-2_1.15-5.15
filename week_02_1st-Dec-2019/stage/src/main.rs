// Making a closure with no arguments
// fn main() {
//     let x = || println!("Hello world");
//     x();
// }

//capturing values from environment
// fn main() {
//     let mut x = 1;
//     let mut f = || {x = x + 1;println!("x = {}",x)};
//     f();
//     f();
//     f();
// }

// another example of capturing values from environment
// fn main() {
//     let mut y = 1;
//     let mut z = 2;
//     let mut x = || {y = y + 1; z = z + 1};
//     x();
//     println!("y = {},z = {}",y,z);
// }

// another example of capturing values from environment
// fn main() {
//     let mut c = 1;
//     let mut inc = || {c = c + 1;};
//     inc();
//     println!("c = {}",c);
// }


// passing a closure as an argument to a function
// fn hello<T:Fn()>(x:T){
//     x();
// }
// fn main() {
//     let mut x = || println!("Hey this is naufil");
//     hello(x);
// }

// A closure expecting u32 argument and returns a u32 value is passed as an argument to a function
// fn hello<T:Fn(u32)->u32>(x:T)->u32{
//     x(2)
// }
// fn main() {
//     let mut x = |y| y + 1;
//     // println!("{}",x(2));
//     println!("{}",hello(x));
// }