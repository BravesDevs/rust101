pub mod namehelpers {
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name = format!("{0} {1}", first, last);
        full_name
    }
}

pub mod txHelpers {

    type SingleLink = Option<Box<Node>>;
    #[derive(Clone)]
    struct Node {
        value: i32,
        next: SingleLink,
    }

    pub struct TransactionLog {
        head: SingleLink,
        tail: SingleLink,
        pub length: u64,
    }

    impl Node {
        fn new(value: i32) -> Node {
            Node {
                value: value,
                next: None,
            }
        }
    }

    impl TransactionLog {
        pub fn new() -> TransactionLog {
            TransactionLog {
                head: None,
                tail: None,
                length: 0,
            }
        }

        pub fn append(&mut self, value: i32) {
            let new_node = Box::new(Node::new(value));
            match self.tail.take() {
                Some(mut old) => old.next = Some(new_node.clone()),
                None => self.head = Some(new_node.clone()),
            }

            self.length += 1;
            self.tail = Some(new_node);
        }

        pub fn pop(&mut self) -> Option<i32> {
            self.head.take().map(|head| {
                let head = *head;
                self.head = head.next;
                if self.head.is_none() {
                    self.tail = None;
                }
                self.length -= 1;
                head.value
            })
        }

        pub fn print(&self) {
            let mut current = &self.head;
            loop {
                match current {
                    Some(node) => {
                        print!("{} -> ", node.value);
                        current = &node.next;
                    }
                    None => {
                        print!("None\n");
                        break;
                    }
                }
            }
        }

        pub fn get_length(&self) -> u64 {
            self.length
        }
    }
}
