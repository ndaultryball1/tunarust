#[no_mangle]
pub extern "C" fn twice(x: i32) -> i32 {
    x * 2
}

#[repr(C)]
pub struct Foo {
    bar: i32,
    tab: f64,
}

impl Foo {
    pub fn addition(&self) -> f64 {
        self.bar as f64 + self.tab
    }
}

#[no_mangle]
pub extern "C" fn add_wrapper(foo_ptr: *const Foo) -> f64 {
    let foo_ex = unsafe {
        assert!(!foo_ptr.is_null());
        & *foo_ptr
    };
    foo_ex.addition()
}