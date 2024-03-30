use std::ffi::c_void;

#[derive(Debug)]
struct Node {
    name: String,
    size: u32,
    count: u32,
}

impl Node {
    pub fn new(name: String) -> Node {
        Node { name, size: 0, count: 0 }
    }
    fn size(self,size: u32) -> Node {
        Node { name: self.name, size, count:self.count }
    }
    fn count(self,count: u32) -> Node {
        Node { name: self.name, size:self.size, count }
    }
    fn to_string(&self)->String{
        "name: ".to_owned() + &*self.name + &*" size: ".to_owned() + &*self.size.to_string() +
            &*" count: ".to_owned() + &*self.count.to_string()
    }
    fn grow(&mut self)-> (){
        self.size+=1
    }
    fn inc(&mut self)-> (){
        self.count+=1
    }
}

fn main() {
    let mut node = Node::new("nome".to_string()).size(2).count(10);
    println!("{}", node.to_string());
    node.grow();
    node.inc();
    println!("{}", node.to_string());
}