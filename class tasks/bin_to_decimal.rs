// CLASS TASK DONE BY MAHEEN SALEH

fn main(){

    let mut val:f32 = 0.0;
    let b = "1011";
    let a = "";
    let mut  p:f32 = (b.len()-1) as f32;
    let mut val = 0.0;


    for i in b.chars(){

      let bit = (i.to_string()).parse::<f32>().expect("please provide valid input");
      // println!(" bit {}",bit+0.0);


      //find 2 pow index
      let prod = 2.0_f32.powf(p);
      // println!(" prod {}",prod);
      p-=1.0;

      val += bit * prod;
    }
    println!(" val {}",val);


}