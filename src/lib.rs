#![allow(dead_code)]

pub struct LinkedList {
    head: Option<Box<ListNode>>,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList{head: None}
    }

    pub fn append(& mut self, val: u32) {
        let new_node = Box::new(ListNode::new(val));

        if let Some(ref mut current_head) = self.head {
            let mut current = current_head;

            while let Some(ref mut next) = current.nxt {
                current = next;
            }

            current.nxt = Some(new_node);
        } else {

            self.head = Some(new_node);
        }
    }

    pub fn print_list(&self) {
        print!("List : ");
        
        if let Some(head) = self.head.as_ref() {
            let mut current = head;

            while let Some(next) = current.nxt.as_ref() {
                print!("{} -> ", current.val);
                current = next;
            }
            println!("{} .", current.val);

        } else {
            println!("Empty List");
        }
    }
}

struct ListNode {
    val: u32,
    nxt: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: u32) -> ListNode {
        ListNode{val, nxt: None}
    }
}
