pub mod chunk {
    enum OPCODE {
        OPRETURN,
    }
    struct Chunk {
        code: Vec<i32>,
    }
    impl Chunk {
        fn init_chunk(&mut self) -> Self {
            Chunk { code: Vec::new() }
        }
    }
}
