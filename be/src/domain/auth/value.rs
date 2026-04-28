use email_address::EmailAddress;
use nutype::nutype;

#[nutype(
    sanitize(trim),
    validate(
        len_char_min = 3,
        len_char_max = 32,
        predicate = |value| value.chars().all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '-'),
    ),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref, Display),
)]
pub struct UserName(String);

#[nutype(
    sanitize(trim, lowercase),
    validate(predicate = |value| EmailAddress::is_valid(value)),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref, Display),
)]
pub struct UserEmail(String);

#[nutype(
    validate(greater = 0),
    derive(
        Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsRef, Deref, Display
    )
)]
pub struct IsoNumericCode(i32);
