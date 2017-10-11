#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

fn is_contains<T:PartialEq>(x: &[T], y: &[T]) -> bool {
    if x.len() < y.len() {
        return false;
    }

    if x.starts_with(y) {
        return true;
    }

    is_contains(&x[1..], y)
}

pub fn sublist<T:PartialEq>(x: &[T], y: &[T]) -> Comparison {
    if x == y {
        return Comparison::Equal
    } else if is_contains(x, y) {
        return Comparison::Superlist
    } else if is_contains(y, x) {
        return Comparison::Sublist
    }
    Comparison::Unequal
}
