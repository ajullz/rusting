use std::collections::VecDeque;

#[derive(Debug)]
struct Link<T: PartialOrd + Copy>(Option<Box<Node<T>>>);

#[derive(Debug)]
struct Node<T: PartialOrd + Copy> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: PartialOrd  + Copy> Node<T> {
    pub fn boxed(elem: T) -> Box<Self> {
        Box::new(Node {
            elem,
            left: Link(None),
            right: Link(None),
        })
    }
}

#[derive(Debug)]
pub struct BST<T: std::cmp::PartialOrd + Copy> {
    root: Link<T>,
}

#[derive(Debug)]
pub struct IntoIter<T: PartialOrd + Copy> {
    next: VecDeque<T>,
}

impl<T: std::cmp::PartialOrd  + Copy> BST<T> {
    pub fn new() -> Self {
        BST { root: Link(None) }
    }

    pub fn insert(&mut self, elem: T) -> bool {
        self.root.insert(elem)
    }

    pub fn search(&self, elem: T) -> bool {
        self.root.search(elem)
    }
}

impl<T: PartialOrd  + Copy> Link<T> {
    fn insert(&mut self, elem: T) -> bool {
        match self {
            Link(None) => {
                *self = Link(Some(Node::boxed(elem)));
                return true;
            },
            Link(Some(ref mut node)) => {
                if elem == node.elem {
                    return false;
                }
                else if elem < node.elem {
                    node.left.insert(elem)
                }
                else {
                    node.right.insert(elem)
                }
            }
        }
    }
    
    fn search(&self, elem: T) -> bool {
        match self {
            Link(None) => false,
            Link(Some(ref node)) => {
                if elem == node.elem {
                    true
                }
                else if elem < node.elem {
                    node.left.search(elem)
                }
                else {
                    node.right.search(elem)
                }
            }
        }
    }

    fn inorder(&self) -> VecDeque<T> {
        if let Link(Some(node)) = self {
            let mut left = node.left.inorder();
            left.push_back(node.elem);
            let mut right = node.right.inorder();
            left.append(&mut right);
            return left;
        }
        VecDeque::new()
    }
}

impl<T: PartialOrd  + Copy> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            next: self.root.inorder(),
        }
    }
}

impl<T: PartialOrd  + Copy> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.pop_front()
    }
}

#[cfg(test)]
mod test {
    use super::BST;
    #[test]
    fn basics() {
        let mut bst = BST::<i32>::new();
        assert_eq!(true, bst.insert(10));
        assert_eq!(false, bst.insert(10));
        assert_eq!(true, bst.insert(20));
        assert_eq!(true, bst.insert(30));
        assert_eq!(true, bst.insert(40));
        assert_eq!(true, bst.search(40));
        assert_eq!(false, bst.search(50));
    }

    #[test]
    fn into_iter() {
        let mut bst = BST::<i32>::new();
        bst.insert(8);
        bst.insert(5);
        bst.insert(11);
        bst.insert(4);
        bst.insert(6);
        bst.insert(9);
        bst.insert(12);
        let expected = vec![4, 5, 6, 8, 9, 11, 12];
        for (expected, val) in expected.into_iter().zip(bst.into_iter()) {
            assert_eq!(expected, val);
        }
    }
}