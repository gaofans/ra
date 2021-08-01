use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::mem;
use std::option::Option::{None, Some};
use std::fmt::{Display, Formatter, Debug};
use std::borrow::Borrow;

#[derive(Debug,Default)]
struct Node<E>{
    value:E,
    pre:Option<Weak<RefCell<Node<E>>>>,
    next:Option<Rc<RefCell<Node<E>>>>
}

#[derive(Debug)]
pub struct LinkedList<E>{
    size:usize,
    start:Option<Rc<RefCell<Node<E>>>>,
    end:Option<Rc<RefCell<Node<E>>>>
}

impl <E> LinkedList<E> where E:Default{
    pub fn new() -> LinkedList<E>{
        LinkedList{
            size:0,
            start:None,
            end:None
        }
    }

    pub fn add_last(&mut self,value:E){
        self.size+=1;
        let new_node = Node{
            pre:None,
            next:None,
            value
        };
        let old_end = mem::replace(&mut self.end, Some(Rc::new(RefCell::new(new_node))));
        match old_end {
            Some(node) => {
                node.borrow_mut().next = Some(self.end.as_ref().unwrap().clone());
                self.end.as_ref().unwrap().borrow_mut().pre = Some(Rc::downgrade(&node.clone()));
            },
            None => {
                self.start = Some(self.end.as_ref().unwrap().clone());
            }
        }
    }

    pub fn add_first(&mut self,value:E){
        self.size+=1;
        let new_node = Node{
            pre:None,
            next:None,
            value
        };
        let old_start = mem::replace(&mut self.start, Some(Rc::new(RefCell::new(new_node))));
        match old_start {
            Some(node) => {
                self.start.as_ref().unwrap().borrow_mut().next = Some(node.clone());
                node.borrow_mut().pre = Some(Rc::downgrade(&self.start.as_ref().unwrap().clone()));
            },
            None => {
                self.end = Some(self.start.as_ref().unwrap().clone());
            }
        }
    }

    pub fn get(&self, index:usize) -> Option<&E> {
        if self.size == 0 || index >= self.size{
            return None;
        }
        let mut count = 0;
        for value in self.iter() {
            if count == index {
                return Some(value);
            }
            count += 1;
        }
        None
    }

    pub fn remove(&mut self,index:usize) -> Option<E>{
        if self.size == 0 || index >= self.size{
            return None;
        }
        self.size -= 1;
        let mut temp = &self.start;
        let mut count = 0;
        while let Some(node) = temp {
            if count == index {
                break;
            }
            unsafe {
                temp = &(*node.as_ptr()).next;
            }
            count += 1;
        }
        let mut node = temp.as_ref().unwrap().take();
        let pre = mem::replace(&mut node.pre,Option::None);
        let next = mem::replace(&mut node.next,Option::None);
        if let Some (node) = pre {
            node.upgrade().unwrap().borrow_mut().next = next;
            if let Some(next_node) = &node.upgrade().unwrap().borrow_mut().next {
                next_node.borrow_mut().pre = Some(node);
            }else {
                self.end = Some(node.upgrade().unwrap().clone());
            }
        }else {
            if let Some(next_node) = next {
                unsafe { (*next_node.as_ptr()).pre = None; }
                self.start = Some(next_node.clone());
            }else {
                self.start = None;
                self.end = None;
            }
        }
        return Some(node.value);
    }

    pub fn iter(&self) -> Iter<E>{
        Iter{next:self.start.as_ref()}
    }
}

impl <T> Display for LinkedList<T> where T:Display+Default {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = Vec::with_capacity(self.size);
        self.iter().for_each(|value|{
            result.push(format!("{}",value));
        });
        write!(f, "{:?}", result)
    }
}
pub struct Iter<'a,T>{
    next:Option<&'a Rc<RefCell<Node<T>>>>
}
impl <'a,T> Iterator for Iter<'a,T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node|{
            unsafe {
                self.next = (*(node.as_ptr())).next.as_ref();
                (*(node.as_ptr())).value.borrow()
            }
        })
    }
}
#[allow(unused_must_use)]
impl <T> Drop for LinkedList<T>  {
    fn drop(&mut self) {
        let mut temp = mem::replace(&mut self.start,None);
        while let Some(node) = temp{
            temp = mem::replace(&mut node.as_ref().borrow_mut().next,None);
        }
        mem::replace(&mut self.end,None);
    }
}

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn test_add() {

        let mut linked_list = LinkedList::new();
        linked_list.add_last(1);
        linked_list.add_last(2);
        linked_list.add_last(3);
        linked_list.add_last(4);
        linked_list.add_last(5);
        linked_list.add_first(6);
        linked_list.add_first(7);
        linked_list.add_first(8);
        linked_list.add_first(9);
        linked_list.add_first(10);
        println!("{}",linked_list);
    }

    #[test]
    fn test_get() {
        let mut linked_list = LinkedList::new();
        linked_list.add_last(1);
        linked_list.add_last(2);
        linked_list.add_last(3);
        linked_list.add_last(4);
        linked_list.add_last(5);
        linked_list.add_first(6);
        linked_list.add_first(7);
        linked_list.add_first(8);
        linked_list.add_first(9);
        linked_list.add_first(10);
        assert_eq!(9,*linked_list.get(1).unwrap())
    }
    #[test]
    fn test_remove() {
        let mut linked_list = LinkedList::new();
        linked_list.add_last(1);
        linked_list.add_last(2);
        linked_list.add_last(3);
        linked_list.add_last(4);
        linked_list.add_last(5);
        linked_list.add_first(6);
        linked_list.add_first(7);
        linked_list.add_first(8);
        linked_list.add_first(9);
        linked_list.add_first(10);
        assert_eq!(2,linked_list.remove(1).unwrap())
    }
    #[test]
    fn test_drop() {
        let mut linked_list = LinkedList::new();
        linked_list.add_last(1);
        linked_list.add_last(2);
        linked_list.add_last(3);
        linked_list.add_last(4);
        linked_list.add_last(5);
        linked_list.add_first(6);
        linked_list.add_first(7);
        linked_list.add_first(8);
        linked_list.add_first(9);
        linked_list.add_first(10);
        mem::drop(linked_list);
    }
}
