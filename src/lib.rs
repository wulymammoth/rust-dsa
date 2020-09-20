pub struct Stack<T> {
    data: Vec<T>,
}

#[allow(dead_code)]
impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.get(self.data.len() - 1)
    }
}

use std::collections::LinkedList;

pub struct Queue {
    data: LinkedList<i32>,
}

#[allow(dead_code)]
impl Queue {
    fn new() -> Self {
        Queue {
            data: LinkedList::new(),
        }
    }

    fn enqueue(&mut self, item: i32) {
        &self.data.push_back(item);
    }

    fn dequeue(&mut self) -> Option<i32> {
        self.data.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
    }

    #[test]
    fn test_stack_pop() {
        let mut stack = Stack::new();
        stack.push(5);
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_queue() {
        let mut q = Queue::new();
        for i in 1..11 {
            q.enqueue(i);
        }
        for j in 1..12 {
            if j <= 10 {
                assert_eq!(q.dequeue(), Some(j));
            } else {
                assert_eq!(q.dequeue(), None);
            }
        }
    }
}
