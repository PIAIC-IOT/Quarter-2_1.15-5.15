#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T 
}

impl Point<f32>{
    fn hello(&self)-> &f32{
        &self.y
    }
}

fn main() {

let p1 = Point{
    x: 10.7,
    y: 15.89
};

println!("{:?}",p1.hello());

}
