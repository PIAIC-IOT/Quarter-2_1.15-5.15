fn main() {

    let a:i32 = 5;      //       000000000....0000000000101

    let b:i32 = 1; 

    let result = a >> b;    // 0000000.....000000000000

    println!("{:b}",result); 
    println!("{}",result); 

}
