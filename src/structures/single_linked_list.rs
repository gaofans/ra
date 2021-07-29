///有序的单向链表
pub struct SingleLinkedList<T> where T:Eq + Ord{
    size:usize,
    head:Link<T>
}
enum Link<T> where T:Eq + Ord{
    Empty,
    More(Box<Node<T>>)
}
struct Node<T> where T:Eq + Ord{
    data:T,
    next:Link<T>
}

use std::mem;
use std::fmt::{Display, Formatter};

impl <T> SingleLinkedList<T> where T:Eq + Ord{

    pub fn new() -> Self{
        SingleLinkedList{
            size: 0,
            head: Link::Empty
        }
    }

    ///添加
    pub fn add(&mut self,data:T){
        self.size += 1;
        //首先判断head的大小
        if let Link::More(node) = &mut self.head{
            if node.data > data {
                let old_head = mem::replace(&mut self.head,Link::Empty);
                self.head = Link::More(Box::new(Node{ data, next:old_head }));
                return;
            }
        }
        //遍历寻找合适的位置
        let mut temp = &mut self.head;
        loop {
            match temp {
                Link::Empty => {
                    self.head = Link::More(Box::new(Node{ data, next: Link::Empty }));
                    return;
                },
                Link::More(node) => {
                    if let Link::More(next_node) = &node.next{
                        if next_node.data > data {
                            let old_next = mem::replace(&mut node.next,Link::Empty);
                            node.next = Link::More(Box::new(Node { data, next: old_next }));
                            break;
                        }else{
                            temp = &mut node.next;
                        }
                    }else {
                        node.next = Link::More(Box::new(Node { data, next: Link::Empty }));
                        break;
                    }
                }
            }
        }

    }

    ///移除
    pub fn remove(&mut self, index:usize) -> Result<T,&str>{
        if self.size <= index {
            Err("下标越界")
        }else{
            if index == 0 {
                let link = mem::replace(&mut self.head, Link::Empty);
                if let Link::More(node) = link{
                    self.head = node.next;
                    self.size -= 1;
                    return Ok(node.data);
                }
            }

            let mut temp = &mut self.head;
            let mut count = 0;
            while let Link::More(node) = temp {
                if count == index - 1 {
                    let next_link = mem::replace(&mut node.next,Link::Empty);
                    if let Link::More(next_node) = next_link {
                        node.next = next_node.next;
                        self.size -= 1;
                        return Ok(next_node.data);
                    }
                }
                temp = &mut node.next;
                count += 1;
            }
            Err("")
        }
    }

    ///查询
    pub fn get(&self,index:usize) -> Result<&T,&str>{
        if self.size <= index {
            Err("下标越界")
        } else {
            let mut temp = &self.head;
            let mut count = 0;
            while let Link::More(node) = temp{
                if count == index {
                    return Ok(&node.data);
                }
                count+=1;
                temp = &node.next;
            }
            panic!("未知错误");
        }
    }

    ///反转
    pub fn reverse(&mut self){
        if self.size == 0 {
            return;
        }
        let mut temp = mem::replace(&mut self.head, Link::Empty);
        let mut next;
        while let Link::More(node) = temp {
            next = mem::replace(&mut self.head, Link::Empty);
            self.head = Link::More(Box::new(Node{ data: node.data, next }));
            temp = node.next;
        }
    }
}
impl <T> Display for SingleLinkedList<T> where T:Eq + Ord + Display{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = Vec::with_capacity(self.size);
        let mut temp = &self.head;
        while let Link::More(node) = temp{
            result.push(format!("{}",node.data));
            temp = &node.next;
        }
        write!(f, "{:?}", result)
    }
}
impl <T> Drop for SingleLinkedList<T> where T:Eq + Ord {
    fn drop(&mut self) {
        let mut temp = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut node) = temp {
            temp = mem::replace(&mut node.next,Link::Empty);
        }
    }
}
#[cfg(test)]
mod test{
    use super::*;
    use crate::util::random_array::random_array;

    #[test]
    fn test_add() {

        let mut linked_list = SingleLinkedList::new();
        random_array(100, 1000).iter().for_each(|ele| {
            linked_list.add(*ele);
        });
        println!("{}",linked_list);
    }

    #[test]
    fn test_take() {
        let mut linked_list = SingleLinkedList::new();
        linked_list.add(1);
        linked_list.add(2);
        linked_list.add(3);
        linked_list.add(4);
        linked_list.add(5);
        linked_list.add(6);
        assert_eq!(1,linked_list.remove(0).unwrap());
        assert_eq!(6,linked_list.remove(4).unwrap());
    }

    #[test]
    fn test_get() {
        let mut linked_list = SingleLinkedList::new();
        linked_list.add(1);
        linked_list.add(2);
        linked_list.add(3);
        linked_list.add(4);
        linked_list.add(5);
        linked_list.add(6);
        assert_eq!(1,*linked_list.get(0).unwrap());
        assert_eq!(2,*linked_list.get(1).unwrap());
        assert_eq!(3,*linked_list.get(2).unwrap());
        assert_eq!(4,*linked_list.get(3).unwrap());
        assert_eq!(5,*linked_list.get(4).unwrap());
        assert_eq!(6,*linked_list.get(5).unwrap());
    }

    #[test]
    fn test_reverse() {
        let mut linked_list = SingleLinkedList::new();
        linked_list.add(1);
        linked_list.add(2);
        linked_list.add(3);
        linked_list.add(4);
        linked_list.add(5);
        linked_list.add(6);
        println!("{}",linked_list);
        linked_list.reverse();
        println!("{}",linked_list);
    }
}