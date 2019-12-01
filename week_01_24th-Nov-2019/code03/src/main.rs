fn main(){

    let s1 = "abcde";

    {
        let s2 = String::from("akjsfkjasdgka");

    }

    let y = largest_string(s1, s2.as_str());

    println!("{}",y);

}

fn largest_string<'a>(x : &'a str, y : &'a str)-> &'a str{

    if x.len()>y.len(){
        x
    }

    else{
        y
    }
}
