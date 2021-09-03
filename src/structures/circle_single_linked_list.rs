use std::mem;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::option::Option::None;

#[derive(Debug)]
pub struct CircleSingleLinkedList<E>{
    size:usize,
    head:Option<Rc<RefCell<Node<E>>>>
}
#[derive(Debug)]
struct Node<E>{
    next:Option<Rc<RefCell<Node<E>>>>,
    value:E
}


impl <E> CircleSingleLinkedList<E>{
    pub fn new() -> CircleSingleLinkedList<E>{
        CircleSingleLinkedList{
            size:0,
            head:None
        }
    }

    pub fn add(&mut self,value:E){
        self.size+=1;
        let new_node:Node<E> = Node{
            value,
            next:None
        };
        match &mut self.head {
            None => {
                self.head = Option::from(Rc::new(RefCell::new(new_node)));
                self.head.as_ref().unwrap().borrow_mut().next = Some(self.head.as_ref().unwrap().clone());
            },
            Some(_) => {
                let next = mem::replace(&mut self.head,Option::from(Rc::new(RefCell::new(new_node))));
                self.head.as_ref().unwrap().borrow_mut().next = next;
                let mut temp = self.head.as_ref().unwrap().clone();
                for _ in 0..(self.size-1) {
                    unsafe {
                        temp = (*temp.as_ptr()).next.as_ref().unwrap().clone();
                    }
                }
                temp.borrow_mut().next = Some(self.head.as_ref().unwrap().clone());
            }
        }

    }

    pub fn joseph(&mut self, mut start:usize, count:usize) -> Vec<&E>{

        let mut vec = Vec::with_capacity(self.size);
        if self.size == 0 {
            return vec;
        }
        start = start % self.size;
        let mut temp = &self.head;
        for _ in 0..start {
            unsafe {
                temp = &(*(temp.as_ref().unwrap().as_ptr())).next;
            }
        }
        loop {
            if self.size == 0 {
                break;
            }
            for _ in 0..count {
                unsafe {
                    temp = &(*(temp.as_ref().unwrap().as_ptr())).next;
                }
            }
            let next = mem::replace(&mut temp.as_ref().unwrap().borrow_mut().next, None);
            self.size -= 1;
            unsafe {
                vec.push(&(*(next.as_ref().unwrap().as_ptr())).value);
                if self.size != 1 {
                    let nn = mem::replace(&mut next.as_ref().unwrap().borrow_mut().next, None);
                    temp.as_ref().unwrap().borrow_mut().next = nn;
                }else {
                    self.size -= 1;
                    vec.push(&(*(temp.as_ref().unwrap().as_ptr())).value);
                }
            }

        }

        vec
    }
}
impl <T> Display for CircleSingleLinkedList<T> where T:Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = Vec::with_capacity(self.size);
        let mut temp = &self.head;
        for _ in 0..self.size {
            unsafe {
                let value = &(*(temp.as_ref().unwrap().as_ptr())).value;
                result.push(format!("{}",value));
                temp = &(*(temp.as_ref().unwrap().as_ptr())).next;
            }
        }
        write!(f, "{:?}", result)
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut list = CircleSingleLinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);
        list.add(4);
        list.add(5);
        list.add(6);
        list.add(7);
        println!("{}",list);
        println!("{:?}",list.joseph(22,12));
    }


}