use bounded_integer::bounded_integer;

bounded_integer! {
    pub struct Word12(-999_999_999_999, 999_999_999_999);
}

bounded_integer! {
    pub struct Word11(-99_999_999_999, 99_999_999_999);
}

bounded_integer! {
    pub struct Switch16(0, 15);
}