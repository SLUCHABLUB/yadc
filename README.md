# YADC

> NOTE: the crate is **not** finished

Yet another derive-crate.

There are many crates
that provide functionality similar to that of the [std derive macro].
They all strive to further this functionality in their own ways.
However, I often find myself needing to use multiple of them to cover all of my needs.

Some notable crates are:

- [`derive_more`]: Supports a lot of traits. 
- [`educe`]: Adds more control over already derivable traits. 
- [`derive-where`]: Adds control over trait bounds.

They all have pros yet cons; therefore, I decided to make this crate.
I do not claim that it is perfect, since perfection is subjective, but it is sufficient for all of my purposes.

## Principles

These are the design philosophies which I have set for `yadc`.
If you do not agree with them, feel free to make an issue with a rationale, create a fork, or create a new crate.

- There should not be a custom derive-macros with the same name as those from the standard library (like in [`derive_more`]).
  - Importing such macros is annoying.
  - They may be confused for the ones in the standard library. 
- There should not be a need for a custom derive macro (like in [`educe`]).
  - It does not derive a trait and is, as shown in `yadc`, unnecessary.
- Attribute macros should avoid using strings (like in [`derive-where`]).
  - Unless they represent strings, it is unnecessary and hinders readability.
  - This includes parsing expressions from strings, as it is ambiguous whether the string or the content is the expression.
- Traits can only be implemented for algebraic types, i.e. structs and enums.

The goal is to achieve equivalent functionality as the aforementioned crates,
unless their features violate the aforementioned principles.

## Supported Traits


| Traits with planned support |
|:---------------------------:|
|        `Borrow(Mut)`        |
|           `Copy`            |
|           `Clone`           |
|            `Eq`             |
|            `Ord`            |
|         `PartialEq`         |
|        `PartialOrd`         |
|           `As***`           |
|           `From`            |
|          `TryFrom`          |
|          `Default`          |
|           `Error`           |
|       `Display`-like*       |
|      `Sum` & `Product`      |
|        `Add` & `Sub`        |
|  `AddAssign` & `SubAssign`  |
|          `BitAnd`           |
|       `BitAndAssign`        |
|           `BitOr`           |
|        `BitOrAssign`        |
|          `BitXor`           |
|       `BitXorAssign`        |
|    `Deref` & `DerefMut`     |
|            `Div`            |
|         `DivAssign`         |
|            `Mul`            |
|         `MulAssign`         |
|        `Neg` & `Not`        |
|            `Rem`            |
|         `RemAssign`         |
|            `Shl`            |
|         `ShlAssign`         |
|            `Shr`            |
|         `ShrAssign`         |
|          `Random`           |
|          `FromStr`          |

If any trait is missing, please open an issue.

### Hash

Hash currently matches `derive` in features.

### Debug

Hash currently surpasses `derive` in features.
Fields may be skipped with `#[debug::skip]`.
A variant can be debugged as non-exhaustive if marked with `#[debug::non_exhaustive]`.

## Config

TODO

[std derive macro]: https://doc.rust-lang.org/reference/attributes/derive.html
[`derive_more`]: https://crates.io/crates/derive_more
[`educe`]: https://crates.io/crates/educe
[`derive-where`]: https://crates.io/crates/derive-where
