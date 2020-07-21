from cffi import FFI
ffibuilder = FFI()

ffibuilder.cdef("""
    int twice(int x);
""")

C = ffibuilder.dlopen("./target/debug/libpricing_interface.so")

if __name__ == "__main__":
    print(C.twice(9))