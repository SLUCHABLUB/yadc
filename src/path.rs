use crate::punctuated;
use crate::util::{new_identifier, token};
use proc_macro2::{Ident, Span};
use syn::{Path, PathSegment};

pub fn new<const N: usize>(segments: [&str; N]) -> Path {
    Path {
        leading_colon: None,
        segments: segments
            .into_iter()
            .map(|name| PathSegment::from(Ident::new(name, Span::call_site())))
            .collect(),
    }
}

pub fn core<const N: usize>(segments: [&str; N]) -> Path {
    let string = segments;
    let mut segments = punctuated![PathSegment::from(new_identifier("core"))];

    for string in string {
        segments.push(PathSegment::from(new_identifier(string)));
    }

    Path {
        leading_colon: Some(token![::]),
        segments,
    }
}

pub fn into_identifier(mut path: Path) -> Option<Ident> {
    if path.leading_colon.is_some() || path.segments.len() != 1 {
        return None;
    }

    let segment = path.segments.pop()?.into_value();

    if !segment.arguments.is_none() {
        return None;
    }

    Some(segment.ident)
}

/// Splits off the first identifier of the path.
/// The leading colon and the arguments are ignored.
///
/// # Errors
///
/// If the path is empty
pub fn split_off_first(path: Path) -> Option<(Ident, Path)> {
    let mut iterator = path.segments.into_pairs();

    let first = iterator.next()?.into_value().ident;

    let segments = iterator.collect();

    Some((
        first,
        Path {
            leading_colon: None,
            segments,
        },
    ))
}
