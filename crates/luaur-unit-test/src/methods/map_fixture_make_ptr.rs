use crate::records::map_fixture::MapFixture;

impl MapFixture {
    pub fn make_ptr(&mut self) -> *mut core::ffi::c_int {
        self.ptrs.push(alloc::boxed::Box::new(0));
        let last = self.ptrs.last_mut().expect("vector is not empty");
        last.as_mut() as *mut core::ffi::c_int
    }
}
