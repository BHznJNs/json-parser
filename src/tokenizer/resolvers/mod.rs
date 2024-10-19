mod number;
mod string;
mod indentifier;

type ResolvedPair<T> = (String, T);

pub use number::resolve_number;
pub use string::resolve_string;
pub use indentifier::resolve_identifier;
