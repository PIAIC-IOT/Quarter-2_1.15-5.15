// #[derive(Debug)]
// struct Paitaint<T,U,A> {
//     age:T,
//     name:U,
//     salary:A,

// }
// fn main() {
//     let Paitaint_1 = Paitaint {
//         age : 25,
//         name : String::from("MAK"),
//         salary : 1000,
//     };
//     println!("{:?}",Paitaint_1);
// // }
// struct Point<T> {
//     x: T,
//     y:T
// }
// impl<T> Point<T> {
//     fn x(&self)->&T{
//         &self.x
//     }
// }
// fn main() {
//     let point_1 = Point {
//         x: 45.5,
//         y:32.6
//     };
//     println!("{}",point_1.x());
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {

//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c'};

//     let p3 = p2.mixup(p1);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }
// wool
// awaz
// awaz kis tarah ki hai
// #[derive(Debug)]
// struct Sheep {
//     name: String
// }
// struct Dog {
//     name: String
// }
// struct Human {
//     name: String
// }
// // #[derive(Debug)]
// pub trait Animal {
//     fn wool(&self)->bool;
//     fn awaz(&self)->bool;
//     fn awaz_kis_tarah_ki(&self)->String;
// }
// // fn exporter_ukasha(x: impl Animal,y:impl Animal)->String{
// //     format!("I would love to export this animal")
// // }
// pub fn exporter_ukasha_T<T: Animal>(item1: T, item2: T) ->String{
//     format!("I would love to export these animals")
// }
// // Trait Bound Syntax
// // #[derive(Debug)]
// impl Animal for Sheep {
//     fn wool(&self)->bool{
//         true
//     }
//     fn awaz(&self)->bool{
//         true
//     }
//     fn awaz_kis_tarah_ki(&self)->String{
//         "baaaaaaaa".to_string()
//     }
// }
// impl Animal for Dog {
//     fn wool(&self)->bool{
//         false
//     }
//     fn awaz(&self)->bool{
//         true
//     }
//     fn awaz_kis_tarah_ki(&self)->String{
//         "bhow".to_string()
//     }
// }
// fn Summary()->impl Animal {
//     Sheep {
//         name: String::from("My name is sheep :)")
//     }
// }
// fn main() {
//     println!("{:?}",Summary());
//     let sheep_1 = Sheep {
//         name: "Sheela".to_string()
//     };
//     let sheep_2 = Sheep {
//         name: "Hola".to_string()
//     };
//     println!("{}",sheep_1.wool());
//     println!("{}",sheep_1.awaz());
//     println!("{}",sheep_1.awaz_kis_tarah_ki());
//     let dog_1 = Dog {
//         name: "Bull dog".to_string()
//     };
//     let dog_2 = Dog {
//         name: "sdjskhfud".to_string()
//     };
//     println!("{}",dog_1.wool());
//     println!("{}",dog_1.awaz());
//     println!("{}",dog_1.awaz_kis_tarah_ki());
//     // println!("{}",exporter_ukasha(dog_1));
//     let human_1 = Human {
//         name: String::from("Naufil")
//     };
//     println!("{}",exporter_ukasha_T(sheep_1,sheep_2));
//     println!("{}",exporter_ukasha_T(dog_1,dog_2));

// }



// struct
// trait
// impl trait for struct
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}








