/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T:Clone + std::fmt::Debug>
{
	//TODO
    length:usize,
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T:Clone + std::fmt::Debug> MyStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
            length :0,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        self.q1.enqueue(elem);
        self.length +=1;
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        if self.length == 0{Err("Stack is empty")}
		else{
            let mut rnum=(*self.q1.peek().unwrap()).clone(); 
            
            while !self.q1.is_empty(){
                let num = self.q1.dequeue().unwrap();
                if self.q1.size()==0{rnum = num;}
                else{self.q2.enqueue(num);}    // print!("{},{:?}\n",self.q1.size(),self.q2.elements);          
            }
              while !self.q2.is_empty(){
                let num = self.q2.dequeue().unwrap();
                self.q1.enqueue(num);
            }
            self.length -= 1;
           
            return Ok(rnum);
        }
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        if self.length == 0{true}
        else{false}
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = MyStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}