pub fn format_signed<T>(value: T) -> String
where
    T: std::fmt::Display + PartialOrd + From<i8> + Copy,
{
    if value >= T::from(0) {
        format!("+{}", value)
    } else {
        value.to_string()
    }
}