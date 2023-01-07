use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct ListNode {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(k: i32, v: i32) -> Self {
        ListNode {
            key: k,
            value: v,
            prev: None,
            next: None,
        }
    }
}

struct LRUCache {
    map: HashMap<i32, Rc<RefCell<ListNode>>>,
    capacity: usize,
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            map: HashMap::new(),
            capacity: capacity as usize,
            head: None,
            tail: None,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        //判断key是否存在
        if let Some(node) = self.map.get(&key) {
        //拷贝node，将其放至队头，删除原结点
            let node = Rc::clone(node);
            self.delete(&node);
            self.update(&node);
            //利用borrow获得value，拷贝到新的变量使之能成功返回
            let value = node.borrow().value;
            value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        //Hash表查找key值是否存在
        if self.map.contains_key(&key) {
            let node_old = Rc::clone(self.map.get(&key).unwrap());
            self.delete(&node_old);
        } else {
            //容量已满，删除队头
            if self.map.len() == self.capacity {
                self.remove_first();
            }
        }
        let node_insert = Rc::new(RefCell::new(ListNode::new(key, value)));
        self.update(&node_insert);
    }

    fn delete(&mut self, node: &Rc<RefCell<ListNode>>) {
        //prev,next有四种组合情况，利用match进行处理
        match (
            Rc::clone(node).borrow().prev.as_ref(), // prev
            Rc::clone(node).borrow().next.as_ref(), // next
        ) {
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
            (None, Some(next)) => {
                self.head = Some(Rc::clone(next));
                next.borrow_mut().prev = None;
            }
            (Some(prev), None) => {
                self.tail = Some(Rc::clone(prev));
                prev.borrow_mut().next = None;
            }
            (Some(prev), Some(next)) => {
                next.borrow_mut().prev = Some(Rc::clone(prev));
                prev.borrow_mut().next = Some(Rc::clone(next));
            }
        }

        let key = node.borrow().key;
        self.map.remove(&key);
    }

    fn update(&mut self, node: &Rc<RefCell<ListNode>>) {
        let node = Rc::clone(node);
        //用borrow_mut()来实现读写
        let mut node_borrow = node.borrow_mut();
        if let Some(n) = self.tail.take() {
            //tail不空
            n.borrow_mut().next = Some(Rc::clone(&node));

            node_borrow.prev = Some(n);
            node_borrow.next = None;

            self.tail = Some(Rc::clone(&node));
        } else {
            //tail为空
            node_borrow.prev = None;
            node_borrow.next = None;

            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node)); 
        }

        let key = node_borrow.key;
        self.map.insert(key, Rc::clone(&node));
    }

    fn remove_first(&mut self) {
        if let Some(node) = self.head.take() {
            //从Hash表中删除key
            let key = node.borrow().key;
            self.map.remove(&key);

            //更新链表
            match node.borrow().next.as_ref() {
                Some(next) => {
                    let next = Rc::clone(next);
                    next.borrow_mut().prev = None;
                    self.head = Some(next);
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main_case() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        //由于超过容量，2应该被删除，get返回-1
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        //1也被删除，返回-1
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
        cache.put(4,5);
        //考察是否实现了覆写
        assert_eq!(cache.get(4), 5);
    }
}
