use yadc::implement;

#[implement(Debug)]
struct Foo;

#[implement(Debug)]
struct Bar(u8, Foo);

#[implement(Debug)]
struct Baz {
    foo: u8,
    bar: Foo,
}

#[implement(Debug)]
enum Qux {
    Foo,
    Bar(u8, Foo),
    Baz { foo: u8, bar: Foo },
}

#[implement(Debug)]
struct Alice();

#[implement(Debug)]
struct Bob {}

#[implement(Debug)]
struct Generic<A, B: Copy> {
    a: A,
    b: B,
}

/// Test that the default behaviour matches `derive(Debug)`.
#[test]
fn debug_default() {
    assert_eq!(format!("{:?}", Foo), "Foo");
    assert_eq!(format!("{:?}", Bar(42, Foo)), "Bar(42, Foo)");
    assert_eq!(
        format!("{:?}", Baz { foo: 42, bar: Foo }),
        "Baz { foo: 42, bar: Foo }"
    );

    assert_eq!(format!("{:?}", Qux::Foo), "Foo");
    assert_eq!(format!("{:?}", Qux::Bar(42, Foo)), "Bar(42, Foo)");
    assert_eq!(
        format!("{:?}", Qux::Baz { foo: 42, bar: Foo }),
        "Baz { foo: 42, bar: Foo }"
    );

    assert_eq!(format!("{:?}", Alice()), "Alice");
    assert_eq!(format!("{:?}", Bob {}), "Bob");

    assert_eq!(
        format!("{:?}", Generic { a: 42, b: 3 }),
        "Generic { a: 42, b: 3 }"
    )
}
