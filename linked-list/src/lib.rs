use std::fmt;

struct Node {
    data: String,
    next: Option<Box<Node>>,
}

pub struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, data: String) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<String> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.data)
    }

    pub fn len(&mut self) -> usize {
        self.size
    }

    pub fn is_empty(&mut self) -> bool {
        self.head.is_none()
    }

    pub fn peek(&self) -> Option<&String> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn iter(&self) -> Iter {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut { next: self.head.as_deref_mut() }
    }

}

// Implement the drop trait so that nodes are deallocated iteratively
// instead of recursively (the default). Recursive cleanup could cause
// a stack overflow.
impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut current_node = self.head.take();
        while let Some(mut boxed_node) = current_node {  // boxing to heap allocate
            current_node = boxed_node.next.take();  // taking ownership
        }
    }
}

impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            write!(f, "{}", node.data)?;
            if node.next.is_some() {
                write!(f, ", ")?;
            }
            current = node.next.as_ref();
        }
        write!(f, "]")
    }
}

// Iterator for immutable borrowing
pub struct Iter<'a> {
    next: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a String;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

// Iterator for mutable borrowing
pub struct IterMut<'a> {
    next: Option<&'a mut Node>
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut list = LinkedList::new();
        list.push("three".to_string());
        list.push("two".to_string());
        list.push("one".to_string());

        assert_eq!(list.pop(), Some("one".to_string()));
        assert_eq!(list.pop(), Some("two".to_string()));
        assert_eq!(list.pop(), Some("three".to_string()));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_len() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0);
        list.push("hello".to_string());
        assert_eq!(list.len(), 1);
        list.push("world".to_string());
        assert_eq!(list.len(), 2);
        list.pop();
        assert_eq!(list.len(), 1);
        list.pop();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_is_empty() {
        let mut list = LinkedList::new();
        assert!(list.is_empty());
        list.push("test".to_string());
        assert!(!list.is_empty());
        list.pop();
        assert!(list.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek(), None);
        list.push("first".to_string());
        assert_eq!(list.peek(), Some(&"first".to_string()));
        list.push("second".to_string());
        assert_eq!(list.peek(), Some(&"second".to_string()));
    }

    #[test]
    fn test_display() {
        let mut list = LinkedList::new();
        assert_eq!(format!("{}", list), "[]");
        list.push("a".to_string());
        assert_eq!(format!("{}", list), "[a]");
        list.push("b".to_string());
        assert_eq!(format!("{}", list), "[b, a]");
        list.push("c".to_string());
        assert_eq!(format!("{}", list), "[c, b, a]");
    }

    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();
        list.push("c".to_string());
        list.push("b".to_string());
        list.push("a".to_string());

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&"a".to_string()));
        assert_eq!(iter.next(), Some(&"b".to_string()));
        assert_eq!(iter.next(), Some(&"c".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = LinkedList::new();
        list.push("c".to_string());
        list.push("b".to_string());
        list.push("a".to_string());

        for item in list.iter_mut() {
            item.push_str("!");
        }
        assert_eq!(format!("{}", list), "[a!, b!, c!]");
    }

}
