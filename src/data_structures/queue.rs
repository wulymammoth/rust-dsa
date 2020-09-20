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
    fn enqueue_and_dequeue() {
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
