use std::ops::Index;

#[derive(PartialEq, Default, Clone, Debug)]
struct Node<T: Clone> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(PartialEq, Default, Clone, Debug)]
struct List<T: Clone> {
    head: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T: Clone> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.clone(),
        });
        self.head = Some(new_node);
    }

    fn push_back(&mut self, _value: T) {}

    fn count(&self) -> usize {
        match &self.head {
            None => 0,
            Some(_head) => {
                let mut current: &Option<Box<Node<T>>> = &self.head;
                let mut result: usize = 0;
                while let Some(node) = current {
                    current = &node.next;
                    result += 1;
                }
                result
            }
        }
    }

    fn get(&self, i: usize) -> Option<&T> {
        if self.count() == 0 {
            return None;
        }
        match &self.head {
            None => None,
            Some(_head) => {
                let mut current: &Option<Box<Node<T>>> = &self.head;
                let mut idx: usize = 0;
                while let Some(node) = current {
                    if idx == i {
                        return Some(&node.value);
                    }
                    current = &node.next;
                    idx += 1;
                }
                panic!("tried to access out of bound index {}", i);
            }
        }
    }

    fn get_node(&self, i: usize) -> Option<&Node<T>> {
        if self.count() == 0 {
            return None;
        }
        match &self.head {
            None => None,
            Some(_head) => {
                let mut current: &Option<Box<Node<T>>> = &self.head;
                let mut idx: usize = 0;
                while let Some(node) = current {
                    if idx == i {
                        return Some(node);
                    }
                    current = &node.next;
                    idx += 1;
                }
                panic!("tried to access out of bound index {}", i);
            }
        }
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        if self.count() == 0 {
            return None;
        }
        match &mut self.head {
            None => None,
            Some(_head) => {
                let mut current: &mut Option<Box<Node<T>>> = &mut self.head;
                let mut idx: usize = 0;
                while let Some(node) = current {
                    if idx == i {
                        return Some(&mut node.value);
                    }
                    current = &mut node.next;
                    idx += 1;
                }
                panic!("tried to access out of bound index {}", i);
            }
        }
    }

    fn get_node_mut(&mut self, i: usize) -> Option<&mut Node<T>> {
        if self.count() == 0 {
            return None;
        }
        match &mut self.head {
            None => None,
            Some(_head) => {
                let mut current: &mut Option<Box<Node<T>>> = &mut self.head;
                let mut idx: usize = 0;
                while let Some(node) = current {
                    if idx == i {
                        return Some(node);
                    }
                    current = &mut node.next;
                    idx += 1;
                }
                panic!("tried to access out of bound index {}", i);
            }
        }
    }

    fn remove_front(&mut self) -> Option<T> {
        match &mut self.head {
            None => None,
            Some(old_head) => {
                let value = old_head.clone().value;
                self.head = old_head.next.clone();
                Some(value)
            }
        }
    }

    fn remove_back(&mut self) -> Option<T> {
        self.head.clone()?;
        let val: T;
        {
            let last_node = self.get_node_mut(self.count() - 1); // get penultimate node from list
            val = last_node?.value.clone();
        }
        let pen_node = self.get_node_mut(self.count() - 2);
        pen_node.unwrap().next = None;
        Some(val)
    }

    fn clear(&mut self) {
        self.head = None;
    }
}

impl<T: Clone> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_push_front() {
        let mut list: List<i32> = List::new();
        list.push_back(0);
        println!("{:?}", list);
    }

    #[test]
    fn default_list_is_empty() {
        let list: List<i32> = Default::default();
        assert_eq!(list.count(), 0);
    }

    #[test]
    fn cloned_list_are_equal() {
        let mut list = List::new();
        list.push_front(String::from("Hello"));
        list.push_front(String::from("World"));

        let cloned = list.clone();
        assert_eq!(cloned.count(), list.count());
        assert_eq!(&cloned[0], &cloned[0]);
        assert_eq!(&cloned[1], &cloned[1]);
    }

    #[test]
    fn test_remove() {
        let mut list: List<String> = List::new();
        list.push_front("toto".to_string());
        assert_eq!(list.count(), 1);
        list.remove_front();
        assert_eq!(list.count(), 0);
        list.push_front("toto2".to_string());
        list.push_front("toto1".to_string());
        list.remove_back();
        assert_eq!(list.count(), 1);
        assert_eq!(list[0], "toto1");
    }

    #[test]
    #[should_panic(expected = "tried to access out of bound index 10")]
    fn out_of_bound_access_panics() {
        let mut list: List<u32> = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list[10], 42);
    }
}
