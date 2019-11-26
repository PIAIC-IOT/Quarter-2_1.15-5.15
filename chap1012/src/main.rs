#[derive(Debug)]
struct Point<T,U>{
    x: T,
    y: U
}

impl<T,U> Point<T,U>{

    fn mixup<V,W>(self, other:Point<V,W>)-> Point<T,V>{

        Point{
            x: self.x,
            y: other.x
        }
    }
}


fn main() {

    let p1 = Point{
        x: 3,
        y: 'a'
    };

    let p2 = Point{
        x: 6.74,
        y: "a string"
    };

    let p3 = p2.mixup(p1);

    println!("{:?}",p3);

}

// p1 = (3,'a')
// p2 = (6.74,"a string")


// p3 = (3,"a string")