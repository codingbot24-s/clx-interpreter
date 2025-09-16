pub mod chunk {
    enum OPCODE {
        OPRETURN,
    }

    struct Chunk {
        code: Box<i32>,
        count: i32,
        capacity: i32,
    }
    impl Chunk {
        fn init_chunk(&mut self) {
            Chunk {
                code: Box::new(0),
                count: 0,
                capacity: 0,
            };
        }
        fn write_chunk(&mut self) {
            // is the current array already has the size for new byte if not grow the arr
            if self.count > self.capacity {
                // GROW THE CAP
                self.capacity = grow_capacity(&self.capacity);
                match reallocate(self.capacity){
                    Some(new_size) => self.code = new_size,
                    None => println!("memory not allocated")    
                }
            }
        }
    }

    pub fn grow_capacity(capacity: &i32) -> i32 {
        if *capacity < 8 { 8 } else { capacity * 2 }
    }
    // TODO: fn for grow array
    
    pub fn reallocate(new_size:i32) -> Option<Box<i32>> {
        if new_size == 0 {
            return None;
        } 
        let result = Box::new(new_size);       
        Some(result)
    }
}
