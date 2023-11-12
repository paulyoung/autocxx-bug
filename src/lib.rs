use autocxx::prelude::*;

include_cpp! {
    #include "MyClass.h"
    safety!(unsafe_ffi)
    subclass!("MyClass", MySubclass)
}

pub struct MySubclass {}
