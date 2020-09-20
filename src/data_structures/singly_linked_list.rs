pub struct SLL<'a> {
    pub head: Option<&'a Node<'a>>,
}

#[allow(dead_code)]
impl SLL<'_> {
    fn new() -> Self {
        SLL { head: None }
    }
}

#[derive(Debug, PartialEq)]
pub struct Node<'a> {
    val: i32,
    pub next: Option<&'a Node<'a>>,
}

#[allow(dead_code)]
impl Node<'_> {
    fn new(val: i32) -> Self {
        Node { val, next: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list() {
        let sll = SLL::new();
        assert_eq!(sll.head, None);
    }

    #[test]
    fn single_node_list() {
        let mut sll = SLL::new();
        let node = Node::new(1);
        sll.head = Some(&node);
    }

    #[test]
    fn multiple_node_list() {
        let mut multi = SLL::new();
        let mut node = Node::new(1);
        multi.head = Some(&node);
        let node2 = Node::new(2);
        node.next = Some(&node2);
        assert_eq!(multi.head.unwrap(), Some(&1));
    }
}
