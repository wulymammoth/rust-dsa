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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
    }

    #[test]
    fn pop() {
        let mut stack = Stack::new();
        stack.push(5);
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), None);
    }
}
