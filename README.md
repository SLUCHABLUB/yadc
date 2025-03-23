# YADC
Yet another derive-crate.

There are many crates
that provide functionality similar to that of the [std derive macro].
They all strive to further this functionality in their own ways.
However, I often find myself needing to use multiple of them to cover all of my needs.

The crates that I have used include:

- [`derive_more`]: Supports a lot of traits. 
- [`educe`]: Adds more control over already derivable traits. 
- [`derive-where`]: Adds control over trait bounds.

They all have pros yet cons; therefore, I decided to make this crate.
I do not claim that it is perfect, since perfection is subjective, but it is sufficient for all of my purposes.

## Principles

These are the design philosophies which I have set for `yadc`.
If you do not agree with them, feel free to make an issue with a rationale, create a fork, or create a new crate.

- There should not be a custom derive-macros with the same name as those from std (like in [`derive_more`]).
  - Importing such macros is annoying.
- There should not be a need to derive a custom trait (like in [`educe`]).
- Attribute macros should avoid using strings (like in [`derive-where`]).
  - Unless they represent strings, it is unnecessary and hinders readability.
  - This includes parsing expressions from strings, as it is ambiguous whether the string or the content is the expression.

The goal is to achieve equivalent functionality as the aforementioned crates,
unless their features violate the aforementioned principles.

## Supported Traits

- `Borrow` & `BorrowMut`: WIP
- `Copy`: WIP
- `Clone`: WIP
- `Eq`: WIP
- `Ord`: WIP
- `PartialEq`: WIP 
- `PartialOrd`: WIP 
- `AsRef` & `AsMut`: WIP 
- `From`: WIP
- `TryFrom`: WIP
- `Default`: WIP
- `Error`: WIP
- `Binary`, `Debug`, `Display`, `LowerExp`, `LowerHex`, `Octal`, `Pointer`, `UpperExp` & `UpperHex`: WIP
- `Hash`: WIP
- `Product` & `Sum`: WIP
- `Copy`: WIP
- `Add` & `Sub`: WIP
- `AddAssign` & `SubAssign`: WIP
- `BitAnd`: WIP
- `BitAndAssign`: WIP
- `BitOr`: WIP
- `BitOrAssign`: WIP
- `BitXor`: WIP
- `BitXorAssign`: WIP
- `Deref` & `DerefMut`: WIP
- `Div`: WIP
- `DivAssign`: WIP
- `Mul`: WIP
- `MulAssign`: WIP
- `Neg` & `Not`: WIP
- `Rem`: WIP
- `RemAssign`: WIP
- `Shl`: WIP
- `ShlAssign`: WIP
- `Shr`: WIP
- `ShrAssign`: WIP
- `Random`: WIP
- `FromStr`: WIP

If any trait is missing from this list, please open an issue.

[std derive macro]: https://doc.rust-lang.org/reference/attributes/derive.html
[`derive_more`]: https://crates.io/crates/derive_more
[`educe`]: https://crates.io/crates/educe
[`derive-where`]: https://crates.io/crates/derive-where
