use std::cell::RefCell;
use std::io;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    data: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}
impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            data,
            left: None,
            right: None,
        }))
    }
}

struct LL<T: std::fmt::Debug> {
    front: Option<Rc<RefCell<Node<T>>>>,
    back: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T: std::fmt::Debug + std::fmt::Display> LL<T> {
    fn new() -> Self {
        LL {
            front: None,
            back: None,
            len: 0,
        }
    }

    fn push_back(&mut self, data: T) {
        let new_node = Node::new(data);
        if self.len == 0 {
            self.front = Some(new_node.clone());
            self.back = Some(new_node.clone());
        } else {
            if let Some(back_node) = self.back.take() {
                back_node.borrow_mut().right = Some(Rc::clone(&new_node));
                new_node.borrow_mut().left = Some(Rc::clone(&back_node));
                self.back = Some(new_node);
            }
        }
        self.len += 1;
    }

    fn pop_front(&mut self) {
        if self.len == 0 {
            return;
        }
        if self.len == 1 {
            self.front = None;
            self.back = None;
        } else {
            self.front = self.next(self.front.as_ref().unwrap().clone());
            self.front.as_ref().unwrap().borrow_mut().left = None;
        }
        self.len -= 1;
    }

    fn pop_back(&mut self) {
        if self.len == 0 {
            return;
        }
        if self.len == 1 {
            self.front = None;
            self.back = None;
        } else {
            self.back = self.prev(self.back.as_ref().unwrap().clone());
            self.back.as_ref().unwrap().borrow_mut().right = None;
        }
        self.len -= 1;
    }

    fn front(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.front.clone()
    }

    fn back(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.back.clone()
    }

    fn next(&self, node: Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
        node.borrow().right.clone()
    }

    fn prev(&self, node: Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
        node.borrow().left.clone()
    }

    fn insert_after(&mut self, data: T, node: Rc<RefCell<Node<T>>>) {
        let right = node.borrow_mut().right.clone();
        if right.is_none() {
            self.push_back(data);
            return;
        }
        let mut new_node = Node::new(data);

        right.unwrap().borrow_mut().left = Some(new_node.clone());
        new_node.borrow_mut().right = node.borrow_mut().right.clone();
        node.borrow_mut().right = Some(new_node.clone());
        new_node.borrow_mut().left = Some(node.clone());
        self.len += 1;
    }

    fn remove(&mut self, node: Rc<RefCell<Node<T>>>) {
        if node.borrow().left.is_none() {
            self.pop_front();
            return;
        }
        if node.borrow().right.is_none() {
            self.pop_back();
            return;
        }
        let prev = node.borrow().left.clone().unwrap();
        let next = node.borrow().right.clone().unwrap();
        prev.borrow_mut().right = Some(next.clone());
        next.borrow_mut().left = Some(prev.clone());
        self.len -= 1;
    }

    // fn len(&self) -> usize {
    //     self.len
    // }
}

#[derive(Debug)]
struct Block {
    id: u32,
    len: u32,
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.id == u32::MAX {
            for _ in 0..self.len {
                write!(f, ".")?;
            }
        } else {
            for _ in 0..self.len {
                write!(f, "{}", self.id)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for LL<Block> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut node = self.front().clone();
        while let Some(b) = node {
            write!(f, "{}", b.borrow().data)?;
            node = self.next(b);
        }
        Ok(())
    }
}

fn main() {
    let mut ll = LL::<Block>::new();
    // let mut ll = LinkedList::<Block>::new();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("failed to read");
    let s = s.trim();
    let ch: Vec<u32> = s
        .chars()
        .map(|s| s.to_digit(10).ok_or("failed to parse"))
        .collect::<Result<_, _>>()
        .unwrap();
    let mut id = 0;
    let mut pos = 0u32;
    for i in (0..ch.len()).step_by(2) {
        ll.push_back(Block { id, len: ch[i] });
        if i + 1 == ch.len() {
            break;
        }
        ll.push_back(Block {
            id: u32::MAX,
            len: ch[i + 1],
        });
        id += 1;
    }

    // println!("{}", ll);
    let mut back = ll.back();
    while let Some(b) = back {
        if b.borrow().data.id == u32::MAX {
            back = ll.prev(b.clone());
            continue;
        }
        let mut front = ll.front();
        let mut found = false;
        while let Some(f) = front {
            if Rc::ptr_eq(&b, &f) {
                break;
            }
            let f_borrow = f.borrow();
            let b_borrow = b.borrow();
            if f_borrow.data.id != u32::MAX {
                drop(f_borrow);
                front = ll.next(f);
                continue;
            }
            let b_len = b_borrow.data.len;
            let f_len = f_borrow.data.len;
            let b_id = b_borrow.data.id;
            drop(b_borrow);
            drop(f_borrow);

            if f_len >= b_len {
                //append after node with b_len (id b_id)
                //append after node with f_len - b_len (id MAX)
                //remove node at f
                if f_len > b_len {
                    ll.insert_after(
                        Block {
                            id: u32::MAX,
                            len: f_len - b_len,
                        },
                        f.clone(),
                    );
                }
                ll.insert_after(
                    Block {
                        id: b_id,
                        len: b_len,
                    },
                    f.clone(),
                );
                ll.remove(f);
                ll.insert_after(
                    Block {
                        id: u32::MAX,
                        len: b_len,
                    },
                    b.clone(),
                );
                found = true;
                break;
            }
            front = ll.next(f);
        }
        back = ll.prev(b.clone());
        if found {
            ll.remove(b);
        }
    }
    let mut sum = 0usize;
    let mut i = 0u32;
    let mut node = ll.front();
    while let Some(n) = node {
        let b = &n.borrow().data;
        if b.id != u32::MAX {
            for _ in 0..b.len {
                sum += (b.id * i) as usize;
                i += 1;
            }
        } else {
            i += b.len;
        }
        node = ll.next(n.clone());
    }
    println!("{:?}", sum);
}
