mod first;

#[cfg(test)]
mod tests {
    use crate::first::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(2);
        list.push(10);
        list.push(233);

        assert_eq!(list.pop(), Some(233));
        assert_eq!(list.pop(), Some(10));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(2));

        assert_eq!(list.pop(), None);
    }
}
