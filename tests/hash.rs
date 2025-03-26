use std::hash::{DefaultHasher, Hash, Hasher};

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

macro_rules! implement_hash {
    ($($item:item)*) => {
        mod derived {
            $(
                #[derive(Hash)]
                $item
            )*
        }

        mod implemented {
            use yadc::implement;

            $(
                #[implement(Hash)]
                $item
            )*
        }
    };
}

implement_hash! {
    pub struct Foo;

    pub struct Bar(pub u8, pub Foo);

    pub struct Baz {
        pub foo: u8,
        pub bar: Foo,
    }

    pub enum Qux {
        Foo,
        Bar(u8, Foo),
        Baz { foo: u8, bar: Foo },
    }

    pub struct Alice();

    pub struct Bob {}

    pub struct Generic<'a, A, B: Copy> {
        pub a: &'a A,
        pub b: B,
    }
}

macro_rules! test_hash {
    ($expr:expr) => {
        assert_eq!(
            hash({
                use derived::*;
                &$expr
            }),
            hash({
                use implemented::*;
                &$expr
            }),
        )
    };
}

/// Test that `yadc::implement` generates the same hashes a `derive`
#[test]
fn hash_default() {
    test_hash!(Foo);
    test_hash!(Bar(42, Foo));
    test_hash!(Baz { foo: 42, bar: Foo });

    test_hash!(Qux::Foo);
    test_hash!(Qux::Bar(42, Foo));
    test_hash!(Qux::Baz { foo: 42, bar: Foo });

    test_hash!(Alice());
    test_hash!(Bob {});

    test_hash!(Generic { a: &42, b: 3 })
}
