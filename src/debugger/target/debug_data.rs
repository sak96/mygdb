extern crate addr2line;

use addr2line::{
    gimli::{EndianRcSlice, RunTimeEndian},
    object, Context, Location,
};

pub struct DebugData {
    ctx: Context<EndianRcSlice<RunTimeEndian>>,
}

impl DebugData {
    pub fn new(target: &str) -> Self {
        let file = std::fs::File::open(target).unwrap();
        let map = unsafe { memmap::Mmap::map(&file).unwrap() };
        let object_file = &object::File::parse(&*map).unwrap();
        let ctx = Context::new(object_file).unwrap();
        Self { ctx }
    }

    pub fn find_location(&self, addr: u64) -> Location {
        use std::convert::TryInto;
        self.ctx
            .find_location(addr.try_into().unwrap())
            .unwrap()
            .unwrap_or(Location {
                line: None,
                file: None,
                column: None,
            })
    }
}
