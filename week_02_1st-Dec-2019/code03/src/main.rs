
fn main()
{
    let y:&str;

    {
        let x = "sdsad";

        let s: &'static str = "I have a static lifetime.";


        let y = &s;


    }

    
    println!("r: {}", y);

    
}
