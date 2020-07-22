from cffi import FFI
from pytest import fixture

def rlib(ffi):
    return ffi.dlopen("./target/debug/libpricing_interface.so")

def test_simple_fn():
    ffi = FFI()
    ffi.cdef("""
        int twice(int x);
    """)

    C = rlib(ffi)
    assert C.twice(9) == 18

def test_struct():
    ffi = FFI()

    ffi.cdef("""
    typedef struct {
        int bar;
        float tab;
    } Foo;"""
    )

    C = rlib(ffi)

    foo_ex = ffi.new("Foo *", [5, 6.0])

    assert C.add_wrapper(foo_ex) == 11.0

    