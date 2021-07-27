use std::mem::take;

#[derive(Debug)]
pub struct Queue<T:Copy>{
    head:usize,
    tail:usize,
    max_size:usize,
    array:Vec<T>
}

impl <T:Copy> Queue<T>{

    pub fn new(max_size:usize) -> Queue<T>{
        Queue{
            head:0,
            tail:0,
            max_size,
            array:Vec::with_capacity(max_size+1)
        }
    }

    pub fn size(&self) -> usize{
        let offset = self.tail as isize - self.head as isize;
        return if offset >= 0 {
            offset as usize
        } else {
            return (offset + (self.max_size + 1) as isize) as usize;
        }
    }

    pub fn add(&mut self,t:T) -> Result<(),&str>{
        if self.size() == self.max_size {
           return Err("队列已满");
        }
        if self.array.len() == self.tail {
            self.array.push(t);
        }else{
            self.array[self.tail] = t;
        }
        self.tail = (self.tail + 1) % (self.max_size + 1);
        Ok(())
    }

    pub fn take(&mut self) -> Result<T,&str>{
        if self.size() == 0 {
            return Err("队列为空");
        }
        let t = self.array.get(self.head).unwrap();
        self.head = (self.head + 1) % (self.max_size + 1);
        Ok(*t)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]

    fn test() {
        let mut q:Queue<usize> = Queue::new(10);
        for i in 0..11 {
            match q.add(i) {
                Err(s) => println!("{}",s),
                _ => println!("{:?}",q)
            }

        }
        for _ in 0..3 {
            match q.take() {
                Err(s) => println!("{}",s),
                Ok(t) => println!("{}",t)
            }
        }
        for i in 11..16 {
            match q.add(i) {
                Err(s) => println!("{}",s),
                _ => println!("{:?}",q)
            }
        }
        for _ in 0..8 {
            match q.take() {
                Err(s) => println!("{}",s),
                Ok(t) => println!("{}",t)
            }
        }
        for i in 16..24 {
            match q.add(i) {
                Err(s) => println!("{}",s),
                _ => println!("{:?}",q)
            }
        }
        for _ in 0..12 {
            match q.take() {
                Err(s) => println!("{}",s),
                Ok(t) => println!("{}",t)
            }
        }
    }
}
