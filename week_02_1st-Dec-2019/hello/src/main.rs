struct Post {
    author: String,
    topic: String
}
pub trait Summary {
    fn summarize(&self)->String;
}
pub trait new_notification {
    fn alert(&self)->String;
}
impl new_notification for Post {
    fn alert(&self)->String {
        format!("New notification is here")
    }
}
impl Summary for Post {
    fn summarize(&self)->String{
        format!("The author is {} and topic is {}",self.author,self.topic)
    }
}
fn notify(item:T)
    where T: Summary + new_notification 
    {
        println!("{}. {}",item.alert(),item.summarize());
}
fn main() {
    let post_1 = Post {
        author: String::from("Naufil"),
        topic: String::from("Class")
    };
    notify(post_1);
}
