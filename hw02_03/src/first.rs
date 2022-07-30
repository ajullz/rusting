#[derive(Debug)]
pub struct BST {
    root: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

impl BST {
    pub fn new() -> Self {
        BST { root: Link::Empty }
    }

    pub fn insert(&mut self, elem: i32) -> bool {
        self.root.insert(elem)
    }

    pub fn search(&self, elem: i32) -> bool {
        self.root.search(elem)
    }
}

impl Link {
    fn new(elem: i32) -> Self {
        let new_node = Box::new(Node {
            elem: elem,
            left: Link::Empty,
            right: Link::Empty,
        });
        Link::More(new_node)
    }

    pub fn insert(&mut self, elem: i32) -> bool {
        match self {
            Link::Empty => {
                *self = Link::new(elem);
                true
            },
            Link::More(ref mut node) => {
                if elem == node.elem {
                    false
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
    
    pub fn search(&self, elem: i32) -> bool {
        match self {
            Link::Empty => false,
            Link::More(ref node) => {
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
}

#[cfg(test)]
mod test {
    use super::BST;
    #[test]
    fn basics() {
        let mut bst = BST::new();
        //println!("bst: {:?}", bst);
        assert_eq!(true, bst.insert(1));
        assert_eq!(false, bst.insert(1));
        assert_eq!(true, bst.insert(2));
        assert_eq!(true, bst.insert(3));
        assert_eq!(true, bst.insert(4));
        //println!("bst: {:?}", bst);
        assert_eq!(true, bst.search(4));
        assert_eq!(false, bst.search(5));
    }
}