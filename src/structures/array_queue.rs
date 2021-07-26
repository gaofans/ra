use std::fmt::{Display, Formatter};
#[derive(Debug)]
pub struct ArrayQueue<T>{
    max_size:usize,
    size:usize,
    queue:Vec<T>
}

impl <T> ArrayQueue<T> {
    fn new(max_size:usize) -> ArrayQueue<T>{
        ArrayQueue{ max_size,size:0,queue:Vec::with_capacity(max_size)}
    }

    fn add(&mut self,t:T) -> Result<&mut Self,String>{
        if self.max_size == (self.size) {
            return Err(String::from("队列已满"));
        }
        self.queue.push(t);
        self.size += 1;
        Ok(self)
    }
    fn take(&mut self) -> Result<T,String>{
        if self.size == 0 {
            return Err(String::from("队列为空"));
        }
        let node = self.queue.remove(0);
        self.size -= 1;
        return Ok(node);
    }
}
impl <T> Display for ArrayQueue<T> where T:Display{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.size == 0 {
            write!(f,"{}","[]")
        }else{
            let x :Vec<String>= self.queue.iter().map(|ele| format!("{}",ele)).collect();
            write!(f,"{:?}",x)
        }

    }
}
#[cfg(test)]
mod test{
    use super::*;

    #[test]

    fn test() {
        let mut queue:ArrayQueue<i32> = ArrayQueue::new(10);
        for i in 0..11 {
            match queue.add(i) {
                Ok(q) => println!("{}",q),
                Err(msg) => println!("{}",msg)
            }
        }
        println!("{}",queue);
        for _ in 0..5 {
            println!("{:?}",queue.take());
        }
        println!("{}",queue);
        for i in 10..20 {
            queue.add(i);
        }
        println!("{}",queue);
        for _ in 0..11 {
            println!("{:?}",queue.take());
        }
        println!("{}",queue);
    }
}