#![allow(dead_code)]

struct NonDebug;

mod implemented {
    use yadc::implement;

    #[implement(Debug)]
    pub struct Foo {
        pub foo: u8,
        #[debug::skip]
        pub bar: super::NonDebug,
        pub qux: Box<u16>,
    }
}

mod derived {
    #[derive(Debug)]
    pub struct Foo {
        pub foo: u8,
        pub qux: Box<u16>,
    }
}

#[test]
fn debug_skip() {
    assert_eq!(
        format!(
            "{:?}",
            implemented::Foo {
                foo: 3,
                bar: NonDebug,
                qux: Box::new(42),
            }
        ),
        format!(
            "{:?}",
            derived::Foo {
                foo: 3,
                qux: Box::new(42),
            }
        )
    )
}
