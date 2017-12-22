
struct CircularQueue {
    buffer: [u8; 10],
    head: u8,
    tail: u8,
    empty: bool
}

impl CircularQueue {
    
    fn push(&mut self, val: u8) {
        let new_tail = (self.tail+1) % 10;
        if new_tail == self.head {
            panic!("Circular Queue Overflow");
        }
        self.tail = new_tail;
        self.buffer[new_tail] = val;
        self.empty = false;
    }

    fn pop(&mut self) -> u8 {
        if self.empty {
            panic!("Circulat Queue Underflow");
        }

        let ret_index = self.head;
        if self.head == self.tail {
            self.empty = true;
        } else {
            self.head = (self.head+1) % 10;
        }
        return self.buffer[ret_index];
    }
}

fn main() {
    println!("Hello, world!");
    let queue: CircularQueue;
}
