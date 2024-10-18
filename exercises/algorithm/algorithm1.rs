/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
// use std::cmp::PartialOrd;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}


// 为什么new函数要单独拉出来，为什么指定实现Clone会编译失败
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }
}

impl<T: Clone> LinkedList<T> {
    // pub fn new() -> Self {
    //     Self {
    //         length: 0,
    //         start: None,
    //         end: None,
    //     }
    // }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
    where T:Ord,
	{
		//TODO
		
        let mut merged_l = Self::new();
        let mut a = list_a.start;
        let mut b = list_b.start;

        while a.is_some() && b.is_some() {
            let node_a = unsafe { a.unwrap().as_ref()};
            let node_b = unsafe { b.unwrap().as_ref()};

            if node_a.val <  node_b.val {
                merged_l.add(node_a.val.clone());
                a = unsafe { node_a.next};
            } else {
                merged_l.add(node_b.val.clone());
                b = unsafe { node_b.next};
            }
        }

        while a.is_some() {
            let node_a = unsafe { a.unwrap().as_ref()};
            merged_l.add(node_a.val.clone());
            a = unsafe { node_a.next};
        }

        while b.is_some() {
            let node_b = unsafe { b.unwrap().as_ref()};
            merged_l.add(node_b.val.clone());
            b = unsafe { node_b.next};
        }
        merged_l

        // let mut index_a: i32 = 0;
        // let mut index_b: i32 = 0;
        // while index_a < list_a.length as i32 && index_b < list_b.length as i32 {
        //     let t_a = *list_a.get(index_a).unwrap();
        //     let t_b = *list_b.get(index_b).unwrap();
        //     if t_a < t_b {
        //         result.add(t_a);
        //         index_a += 1;
        //     } else {
        //         result.add(t_b);
        //         index_b += 1;
        //     }
        //     result.length += 1;
        // }
        // if index_a == list_a.length {
        //     while index_b < list_b.length as i32 {
        //         result.add(list_b.get(index_b));
        //         index_b += 1;
        //         result.length += 1;
        //     }
        // }
        // if index_b == list_b.length {
        //     while index_a < list_a.length  as i32 {
        //         result.add(list_a.get(index_a));
        //         index_a += 1;
        //         result.length += 1;
        //     }
        // }
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}