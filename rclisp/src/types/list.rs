use super::Object;

use std::fmt;
use std::borrow::Borrow;

const EMPTY_LIST: List = List::EndsWith(Object::Nil);

#[macro_export]
macro_rules! generate_symbol_list {
    ($x:expr) => ($crate::types::cons($crate::types::symbol($x), $crate::types::nil()));
    ($x:expr, $($y:expr),+) => ($crate::types::cons($crate::types::symbol($x), $crate::generate_symbol_list!($($y),+)));
}

// This thing is really funky to implement
#[derive(Clone, Debug, PartialEq)]
pub enum List {
    EndsWith(Object),
    Cons(Object, Box<List>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct IterMut {
    inner: Box<List>,
}

impl List {
    pub fn prepend(&self, l: Object) -> Self {
        Self::Cons(l, Box::new(self.to_owned()))
    }

    pub fn new_pair(l: Object, r: Object) -> Self {
        Self::Cons(l, Box::new(Self::EndsWith(r)))
    }

    pub fn is_end(&self) -> bool {
        matches!(self, Self::EndsWith(_))
    }

    pub fn len(&self) -> usize {
        match self {
            Self::EndsWith(Object::Nil) => 0,
            Self::EndsWith(_) => 1,
            Self::Cons(_, n) => 1 + n.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() < 1
    }

    pub fn car_ref(&self) -> &Object {
        if let Self::Cons(l, _) = self {
            l
        } else {
            &Object::Nil
        }
    }

    pub fn unpack(self) -> (Object, Object) {
        match self {
            Self::Cons(car, cdr) => {
                let cdr_ret: &List = cdr.borrow();
                (car, match cdr_ret {
                    List::EndsWith(Object::Nil) => Object::Nil,
                    List::EndsWith(o) => o.clone(),
                    List::Cons(_, _) => cdr_ret.clone().into(),
                })
            },
            Self::EndsWith(Object::Nil) => (Object::Nil, Object::Nil),
            Self::EndsWith(o) => (o, Object::Nil),
        }
    }

    pub fn car(self) -> Object {
        let (ret, _) = self.unpack();
        ret
    }

    pub fn cdr(self) -> Object {
        let (_, ret) = self.unpack();
        ret
    }
}

impl IterMut {
    fn new(list: List) -> Self {
        Self {
            inner: Box::new(list),
        }
    }
}

impl Iterator for IterMut {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        let (ret, inner) = match &self.inner.borrow() {
            List::EndsWith(Object::Nil) => (None, Box::new(EMPTY_LIST)),
            List::EndsWith(o) => (Some(o.to_owned()), Box::new(EMPTY_LIST)),
            List::Cons(o, r) => (Some(o.to_owned()), Box::clone(r)),
        };
        self.inner = inner;
        ret
    }
}

impl IntoIterator for List {
    type Item = Object;
    type IntoIter = IterMut;
    
    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self)
    }
}

impl From<List> for Object {
    fn from(l: List) -> Self {
        Object::List(Box::new(l))
    }
}

impl From<&List> for Object {
    fn from(l: &List) -> Self {
        Object::List(Box::new(l.to_owned()))
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut next = self;
        write!(f, "(")?;
        loop {
            match next {
                Self::Cons(l, r) => {
                    write!(f, "{}", l)?;
                    next = r;
                    if !next.is_end() {
                        write!(f, " ")?;
                    }
                }
                Self::EndsWith(o) => {
                    if o != &Object::Nil {
                        write!(f, " . {}", o)?;
                    }

                    write!(f, ")")?;
                    break;
                }
            }
        }
        Ok(())
    }
}

pub fn cons(l: Object, r: Object) -> Object {
    match r {
        Object::List(list) => list.prepend(l).into(),
        _ => List::new_pair(l, r).into(),
    }
}

#[cfg(test)]
mod test {
    use super::super::{symbol, nil, Object};
    use super::{cons, List};

    #[test]
    fn test_list_cons_nil_ended() {
        let list = cons(symbol("A"), nil());
        assert_eq!(
            list,
            Object::List(Box::new(List::Cons(
                Object::Symbol("A".into()),
                Box::new(List::EndsWith(Object::Nil))
            )))
        );
    }

    #[test]
    fn test_list_cons_dotted() {
        let list = cons(symbol("A"), symbol("B"));
        assert_eq!(
            list,
            Object::List(Box::new(List::Cons(
                Object::Symbol("A".into()),
                Box::new(List::EndsWith(Object::Symbol("B".into())))
            )))
        );
    }

    #[test]
    fn test_list_cons_another_list() {
        let list = cons(cons(symbol("A"), symbol("B")), symbol("C"));
        assert_eq!(
            list,
            Object::List(Box::new(List::Cons(
                Object::List(Box::new(List::Cons(
                    Object::Symbol("A".into()),
                    Box::new(List::EndsWith(Object::Symbol("B".into())))
                ))),
                Box::new(List::EndsWith(Object::Symbol("C".into())))
            )))
        );
    }

    #[test]
    fn test_list_display() {
        let list = cons(symbol("A"), nil());
        assert_eq!("(A)", list.to_string());

        let list = cons(nil(), nil());
        assert_eq!("(NIL)", list.to_string());

        let list = cons(symbol("A"), symbol("B"));
        assert_eq!("(A . B)", list.to_string());

        let list = cons(cons(symbol("A"), symbol("B")), symbol("C"));
        assert_eq!("((A . B) . C)", list.to_string());

        let list = cons(symbol("A"), cons(symbol("B"), cons(symbol("C"), nil())));
        assert_eq!("(A B C)", list.to_string());

        let list = cons(cons(symbol("A"), symbol("B")), cons(symbol("C"), symbol("D")));
        assert_eq!("((A . B) C . D)", list.to_string());
    }

    #[test]
    fn test_list_car() {
        if let Object::List(list) = cons(symbol("A"), symbol("B")) {
            assert_eq!(symbol("A"), list.car());
        }
        if let Object::List(list) = cons(symbol("A"), nil()) {
            assert_eq!(symbol("A"), list.car());
        }
        if let Object::List(list) = cons(symbol("A"), cons(symbol("B"), nil())) {
            assert_eq!(symbol("A"), list.car());
        }
    }

    #[test]
    fn test_list_cdr() {
        if let Object::List(list) = cons(symbol("A"), symbol("B")) {
            assert_eq!(symbol("B"), list.cdr());
        }
        if let Object::List(list) = cons(symbol("A"), nil()) {
            assert_eq!(nil(), list.cdr());
        }
        if let Object::List(list) = cons(symbol("A"), cons(symbol("B"), nil())) {
            assert_eq!(cons(symbol("B"), nil()), list.clone().cdr());
            if let Object::List(inner_list) = list.cdr() {
                assert_eq!(nil(), inner_list.cdr());
            }
        }
    }

    #[test]
    fn test_list_len() {
        fn get_len(l: Object) -> usize {
            if let Object::List(list) = l {
                list.len()
            } else {
                unreachable!()
            }
        }
        assert_eq!(get_len(cons(symbol("A"), nil())), 1);
        assert_eq!(get_len(cons(symbol("A"), symbol("B"))), 2);
        assert_eq!(get_len(cons(symbol("A"), cons(symbol("B"), nil()))), 2);
        assert_eq!(get_len(cons(symbol("A"), cons(symbol("B"), symbol("C")))), 3);
        assert_eq!(get_len(cons(symbol("A"), cons(symbol("B"), cons(symbol("C"), nil())))), 3);
        assert_eq!(get_len(cons(cons(symbol("A"), symbol("B")), cons(symbol("C"), cons(symbol("D"), nil())))), 3);
    }
}
