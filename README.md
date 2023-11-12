Minimal repro of an autocxx bug.

```
cargo:warning=In file included from /Users/py/projects/paulyoung/autocxx-bug/main/target/debug/build/autocxx-bug-94a6f68b4aecae68/out/autocxx-build-dir/cxx/gen0.cxx:2:

cargo:warning=/Users/py/projects/paulyoung/autocxx-bug/main/target/debug/build/autocxx-bug-94a6f68b4aecae68/out/autocxx-build-dir/include/autocxxgen_ffi.h:43:148: error: no matching function for call to 'operator new'

cargo:warning=inline void new_synthetic_const_copy_ctor_0x42aaa771a89cc55d_autocxx_wrapper_0x42aaa771a89cc55d(MyClass* autocxx_gen_this, const MyClass& arg1)  { new (autocxx_gen_this) MyClass(arg1); }

cargo:warning=                                                                                                                                                   ^   ~~~~~~~~~~~~~~~~~~

cargo:warning=include/MyClass.h:12:15: note: candidate function not viable: requires single argument 'size', but 2 arguments were provided

cargo:warning=        static void* operator new( size_t size );

cargo:warning=                     ^
```
