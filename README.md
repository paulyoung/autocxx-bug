Minimal repro of an autocxx bug.

```
cargo:warning=In file included from /Users/py/projects/paulyoung/autocxx-bug/main/target/debug/build/autocxx-bug-94a6f68b4aecae68/out/autocxx-build-dir/cxx/gen0.cxx:2:

cargo:warning=/Users/py/projects/paulyoung/autocxx-bug/main/target/debug/build/autocxx-bug-94a6f68b4aecae68/out/autocxx-build-dir/include/autocxxgen_ffi.h:57:144: error: no matching function for call to 'operator new'

cargo:warning=inline void MySubclassCpp_new_autocxx_autocxx_wrapper_0xc63676881d849a16(MySubclassCpp* autocxx_gen_this, rust::Box<MySubclassHolder> arg1)  { new (autocxx_gen_this) MySubclassCpp(std::move(arg1)); }

cargo:warning=                                                                                                                                               ^   ~~~~~~~~~~~~~~~~~~

cargo:warning=include/MyClass.h:12:15: note: candidate function not viable: requires single argument 'size', but 2 arguments were provided

cargo:warning=        static void* operator new( size_t size );

cargo:warning=                     ^
```
