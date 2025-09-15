
pub mod chunk {
    enum OPCODE {
       OPRETURN 
    }

    struct Chunk {
        code:Box<u8>,
        count:i32,
        capacity:i32
    }
    impl Chunk{
        fn init_chunk(&mut self) {
            Chunk{
                code:Box::new(0),
                count:0,
                capacity:0,
            };
        }
        fn write_chunk(&mut self) {
            if self.count > self.capacity {
                let old_capacity = self.capacity;
                self.capacity = grow_capacity(&old_capacity); 
            }
        }   
    }

    pub fn grow_capacity (capacity: &i32) -> i32 {
        if *capacity < 8 {
            8
        }else {
            capacity * 2
        }
    }
}