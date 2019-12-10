// Revision
// Code belongs to Syed Uzair Ahmed
// Github: https://github.com/SyedUzairAhmed03/Assiment-2/blob/master/assiment-2-Q2/closures_properties.rs
// fn main() {
//     let x = || println!("hello world");
//     x();
// }

// fn main() {
//     let x = |x:u32| x+1;
//     let y = 3;
//     println!("The function returns: {}",x(y)); 
// }

// fn main() {
//     let mut c = 1;
//     let mut x = || {c = c+1;};
//     x();
//     println!("The new value of c is: {}",c); // should print 2
// }

// Write a function which accepts a closure, and in the funciton body, it calls the closure. The closure accepts no argument and returns nothing. What should the closure be about? Be creative!
// passing a closure as an argument to a function
// fn print_message<T:Fn()>(x:T){
//     x();
//  }
//  fn main() {
//      let mut Print = || println!("This closure accepts no argument and returns nothing.It only print the message");
//      print_message(Print);
//  }

// // Write a function which expects a closure 
// as an argument and in the funciton body, 
// it calls the closure. The closure expects 
// u32 argument and returns the u32 value. 
// The closure adds 1 to the argument and returns it.
// fn hello<T:Fn(u32)->u32>(x:T)->u32{
//     x(5)
// }
// fn main() {
//     let mut add_1 = |y| y + 1;
//     println!("{}",hello(add_1));
// }

//class start
// Practice questions

// fn main() {
//     let x = 1;
//     let y = || x + 1;
//     println!("{}",y());
//     println!("{}",x);
// }


// fn main() {
//     let x = 1;
//     let y = || {x = x + 1; x};
//     println!("{}",y());
//     println!("{}",x);
// }


// fn main() {
//     let mut x = 1;
//     let y = || {x = x + 1; x};
//     println!("{}",y());
//     println!("{}",x);
// }


// fn main() {
//     let mut x = 1;
//     let mut y = || {x = x + 1; x};
//     println!("{}",y());
//     println!("{}",x);
// }


// fn main() {
//     let mut x = 1;
//     let y = || x = x + 1;
//     println!("{}",y());
//     println!("{}",x);
// }


// fn main() {
//     let mut x = 1;
//     let y = || x = 1 + 1; 
//     y();
//     println!("{}",x);
// }

// fn main() {
//     let mut x = 1;
//     let mut y = || x = x + 1;
//     y();
//     println!("{}",x);
// }


// Example 1
// fn main() {
//     let x = String::from("Hello");
//     let y = || x.push_str("hey"); // will give error
//     println!("{}",x);
// }

// Example 2
// fn main() {
//     let mut x = String::from("Hello");
//     let y = || x.push_str("hey"); // works perfectly
//     println!("{}",x);
// }

// Example 3
// fn hey<T:FnMut()>(y:T){ // error[E0596]: cannot borrow `y` as mutable, as it is not declared as mutable
//     y();
// }
// fn main() {
//     let mut x = String::from("Hello");
//     let z = || x.push_str(" hey"); 
//     // println!("{}",x);
//     hey(z);
//     println!("{}",x);
// }


// Example 4
// fn hey<T:FnMut()>(mut y:T){
//     y();
// }
// fn main() {
//     let mut x = String::from("Hello");
//     let z = || x.push_str(" hey"); 
//     // println!("{}",x);
//     hey(z);
//     println!("{}",x);
// }

// Example 5
// #![allow(unused)]
// fn main() {
// fn consume_with_relish<F>(func: F)
//     where F: FnOnce() -> String
// {
//     // `func` consumes its captured variables, so it cannot be run more
//     // than once.
//     println!("Consumed: {}", func());

//     println!("Delicious!");

//     // Attempting to invoke `func()` again will throw a `use of moved
//     // value` error for `func`.
// }

// let x = String::from("x");
// let consume_and_return_x = || x;
// consume_with_relish(consume_and_return_x);

// // `consume_and_return_x` can no longer be invoked at this point
// }


// Example 6
// #![allow(unused)]
// fn main() {
// fn consume_with_relish<F:FnOnce()->String>(func: F)
//     // where F: FnOnce() -> String
// {
//     // `func` consumes its captured variables, so it cannot be run more
//     // than once.
//     println!("Consumed: {}", func());

//     println!("Delicious!");
//     // println!("Consumed: {}", func());
//     // Attempting to invoke `func()` again will throw a `use of moved
//     // value` error for `func`.
// }

// let x = String::from("x");
// let consume_and_return_x = || x;
// consume_with_relish(consume_and_return_x);
// consume_with_relish(consume_and_return_x);

// // `consume_and_return_x` can no longer be invoked at this point
// }

//  Example 7
// #![allow(unused)]
// fn main() {
// fn do_twice<F:FnMut()>(mut func: F)
// {
//     func();
//     func();
// }
// let mut x: usize = 1;
// {
//     let add_two_to_x = || x += 2;
//     do_twice(add_two_to_x);
// }
// println!("{}",x);
// }



// x.meonw() // method
// x::meonw() // associated function
