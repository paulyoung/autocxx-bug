use autocxx::prelude::*;

include_cpp! {
    #include "MyClass.h"
    safety!(unsafe_ffi)
    generate!("MyClass")
}
