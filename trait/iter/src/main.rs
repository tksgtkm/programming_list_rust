use std::iter::Iterator;

#[derive(Debug, Clone)]
enum List<T> {
    Node {data: T, next: Box<List<T>>},
    Nil,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List::Nil
    }

    fn cons(self, data: T) -> List<T> {
        List::Node {
            data,
            next: Box::new(self),
        }
    }

    fn iter<'a>(&'a self) -> ListIter<'a, T> {
        ListIter {elm: self}
    }
}

struct ListIter<'a, T> {
    elm: &'a List<T>,
}

