//! fgh
#![deny(missing_docs)]
#![deny(unused_qualifications)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![deny(unused_import_braces)]
#![deny(unstable_features)]
#![deny(unsafe_code)]
#![deny(arithmetic_overflow)]

//#![deny(warnings)]
//#![no_std]

///d1
pub mod foo_module {
    #[derive(Debug)]
    ///doc2
    pub struct Foo {
        inner: u32,
    }
///doc
    pub struct FooBuilder {
        a: u32,
        b: u32,
    }
//doc
    impl FooBuilder {
        ///doc
        pub fn new(starter: u32) -> Self {
            Self {
                a: starter,
                b: starter,
            }
        }
        ///doc
        pub fn double_a(self) -> Self {
            Self {
                a: self.a * 2,
                b: self.b,
            }
        }
///doc
        pub fn into_foo(self) -> Foo {
            Foo {
                inner: self.a + self.b,
            }
        }
    }
}

///doc
fn main() {
    let x = foo_module::FooBuilder::new(10)
        .double_a()
        .into_foo();

    println!("{:#?}", x);
}
