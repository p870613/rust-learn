use std::cell::RefCell;
use std::process;
use std::rc::Rc;

#[derive(Debug)]
struct List {
    size: u32,
    head: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct Node {
    val: i32,
    pre: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl List {
    fn new(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<List>>> {
        Some(Rc::new(RefCell::new(List {
            size: 1,
            head: {
                match node {
                    Some(node_Rc) => Some(Rc::clone(&node_Rc)),
                    None => {
                        process::exit(1);
                    }
                }
            },
        })))
    }

    fn push(&mut self, val: i32) {
        let mut ptr;
        self.size += 1;
        match self.head {
            Some(ref node) => ptr = Rc::clone(node),
            None => process::exit(1),
        }

        let mut tmp;
        loop {
            tmp = Rc::clone(&ptr);
            match tmp.borrow().next {
                Some(ref next) => {
                    ptr = Rc::clone(next);
                }
                None => break,
            }
        }

        let new_node = Node::new(val, Some(Rc::clone(&ptr)), None);
        match new_node {
            Some(ref node) => {
                ptr.borrow_mut().next = Some(Rc::clone(node));
            }
            None => {
                process::exit(1);
            }
        }
    }

    fn pop(&mut self) {
        let mut ptr;
        match self.head {
            Some(ref node) => ptr = Some(Rc::clone(node)),
            None => {
                println!("no node can pop");
                process::exit(1);
            }
        }
        let len = self.size;
        for _ in 0..(len - 2) {
            match ptr {
                Some(cur) => {
                    ptr = match cur.borrow().next {
                        Some(ref node) => Some(Rc::clone(node)),
                        None => None,
                    }
                }
                None => break,
            }
        }
        match ptr {
            Some(node) => {
                let del = node.borrow_mut().next.take();
            }
            None => println!("error"),
        }
    }

    fn insert(&mut self, val: i32, pos: u32) {
        if pos == 0 {
            match self.head {
                Some(ref node) => {
                    let new_node = Node::new(val, None, Some(Rc::clone(node)));
                    self.head = match new_node {
                        Some(ref node) => Some(Rc::clone(node)),
                        None => None,
                    };
                    self.size += 1;
                }
                None => {
                    self.head = Node::new(val, None, None);
                    self.size = 1;
                }
            }
        } else {
            let mut ptr;
            let mut pre;
            let mut next = None;
            match self.head {
                Some(ref node) => {
                    pre = Some(Rc::clone(node));
                    ptr = Some(Rc::clone(node));
                }
                None => {
                    process::exit(1);
                }
            }

            for _ in 0..(pos) {
                match ptr {
                    Some(cur) => {
                        pre = Some(Rc::clone(&cur));
                        match cur.borrow().next {
                            Some(ref node) => {
                                next = Some(Rc::clone(node));
                                ptr = Some(Rc::clone(node));
                            }
                            None => {
                                ptr = None;
                                next = None;
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
            //pre next

            match pre {
                Some(ref node_pre) => {
                    match next {
                        //mid
                        Some(ref node_next) => {
                            let new_node = Node::new(
                                val,
                                Some(Rc::clone(node_pre)),
                                Some(Rc::clone(node_next)),
                            );
                            match new_node {
                                Some(ref node) => {
                                    node_next.borrow_mut().pre = Some(Rc::clone(node));
                                    node_pre.borrow_mut().next = Some(Rc::clone(node));
                                }
                                None => {
                                    println!("new_node don't create");
                                    process::exit(1);
                                }
                            }
                            self.size += 1;
                        }
                        //tail
                        None => {
                            let new_node = Node::new(val, Some(Rc::clone(node_pre)), None);

                            match new_node {
                                Some(ref node) => {
                                    node_pre.borrow_mut().next = Some(Rc::clone(node));
                                }
                                None => {
                                    println!("new_node don't create");
                                    process::exit(1);
                                }
                            }
                            self.size += 1;
                        }
                    }
                }
                //head
                None => {
                    println!("no node");
                }
            }
        }
    }

    fn erase() {}

    fn traverse(&mut self) {
        let mut ptr;
        match self.head {
            Some(ref node) => ptr = Some(Rc::clone(node)),
            None => {
                println!(":4:");
                process::exit(1);
            }
        }

        loop {
            match ptr {
                Some(cur) => {
                    println!("{}", cur.borrow().val);
                    ptr = match cur.borrow().next {
                        Some(ref node) => Some(Rc::clone(node)),
                        None => None,
                    }
                }
                None => {
                    break;
                }
            }
        }
    }
}

impl Node {
    fn new(
        val: i32,
        pre: Option<Rc<RefCell<Node>>>,
        next: Option<Rc<RefCell<Node>>>,
    ) -> Option<Rc<RefCell<Node>>> {
        Some(Rc::new(RefCell::new(Node {
            val: val,
            pre: pre,
            next: next,
        })))
    }
}

fn main() {
    let head = List::new(Node::new(1, None, None));
    let mut tmp = None;
    let mut tmp2 = None;
    let mut tmp3 = None;
    match head {
        Some(ref node) => {
            node.borrow_mut().insert(2, 0);
            node.borrow_mut().push(3);
            node.borrow_mut().push(4);
            node.borrow_mut().insert(4, 1);
            tmp = Some(Rc::clone(node));
            tmp2 = Some(Rc::clone(node));

            tmp3 = Some(Rc::clone(node));
        }
        None => {
            process::exit(1);
        }
    }

    println!("\n");
    match tmp {
        Some(node) => node.borrow_mut().traverse(),
        None => {
            process::exit(1);
        }
    }
    match tmp2 {
        Some(node) => node.borrow_mut().pop(),
        None => {
            process::exit(1);
        }
    }
    println!("\n");
    match tmp3 {
        Some(node) => node.borrow_mut().traverse(),
        None => {
            process::exit(1);
        }
    }
}
