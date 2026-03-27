/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T: Ord + Clone>Node<T> {
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

impl<T: Ord + Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone>LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

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
	{
		//TODO
        //code of "get" edv
        // let mut list_a = list_a;
        // let mut list_b = list_b;
        // let mut a = list_a.length;
        // let mut b = list_b.length;
        // let mut list_c = LinkedList::<T>::new();
        // list_c.length = &a + &b;
        // while a != 0 && b != 0 {
        //     let la = list_a.length-a;
        //     let lb = list_b.length-b;
        //     let na = list_a.get(la as i32).unwrap();
        //     let nb = list_b.get(lb as i32).unwrap(); 
        //     if *na >= *nb {
        //         list_c.add(nb.clone());
        //         b-=1;
        //     }else {
        //         list_c.add(na.clone());
        //         a-=1;
        //     }
        // }  
        // if b != 0{
        //     for _ in 0..b{
        //             let lb = list_b.length-b;
        //                 list_c.add(list_b.get(lb as i32).unwrap().clone());
        //             b-=1;}
        // }else {
        //         for _ in 0..a{
        //         let la = list_a.length-a;
        //         list_c.add(list_a.get(la as i32).unwrap().clone());
        //             a-=1;
        //     }
        //     }
        // list_c     
        let mut list_c = LinkedList::<T>::new();
       
        let mut ptr  = None;
        let mut list_a = list_a;
        let mut list_b = list_b;
        let mut current_a = list_a.start;
        let mut current_b = list_b.start;
        let mut current_c = list_c.start;    
        let mut con_a = 0;
        let mut con_b = 0;
        loop{
            if con_a == list_a.length||con_b == list_b.length {
                if con_a==list_a.length{
                    unsafe {(*(current_c.unwrap()).as_ptr()).next = current_b;}                     
                }else{
                    unsafe { (*(current_c.unwrap()).as_ptr()).next = current_a;}                  
                }
                break;
            }
            else {
                unsafe {
                    let a_num =(*(current_a.unwrap()).as_ptr()).val.clone();
                    let b_num =(*(current_b.unwrap()).as_ptr()).val.clone();
                    if a_num <= b_num{
                        if current_c.is_none(){
                            ptr = current_a;
                        }else{
                            (*(current_c.unwrap()).as_ptr()).next = current_a;
                            
                        }current_c = current_a;
                        current_a = (*(current_a.unwrap()).as_ptr()).next;
                        con_a +=1;
                    }else{
                        if current_c.is_none(){
                            ptr = current_b;
                        }else{
                            (*(current_c.unwrap()).as_ptr()).next = current_b;
                        }
                        current_c = current_b;
                        current_b = (*(current_b.unwrap()).as_ptr()).next;
                        con_b +=1;
                    }                  
                }                
            }
        }
        list_c.length = list_a.length + list_b.length;
        list_c.start = ptr;
        list_c
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