use std::ops;

use super::Domain;

impl<T: Clone + ops::Add<Output = T>> ops::Add<Box<Domain<T>>> for Box<Domain<T>> {
    type Output = Box<Domain<T>>;

    fn add(self, rhs: Box<Domain<T>>) -> Self::Output {
        Box::new(*self + *rhs)
    }
}

impl<T: Clone + ops::Add<Output = T>> ops::Add<Domain<T>> for Box<Domain<T>> {
    type Output = Box<Domain<T>>;

    fn add(self, rhs: Domain<T>) -> Self::Output {
        Box::new(*self + rhs)
    }
}

impl<T: Clone + ops::Add<Output = T>> ops::Add<Domain<T>> for Domain<T> {
    type Output = Domain<T>;

    fn add(self, rhs: Domain<T>) -> Self::Output {
        match (self, rhs) {
            (Domain::Range(l1, r1), Domain::Range(l2, r2)) => Domain::Range(l1 + l2, r1 + r2),
            (Domain::Value(v1), Domain::Value(v2)) => Domain::Value(v1 + v2),

            (Domain::Range(l, r), Domain::Value(v)) | (Domain::Value(v), Domain::Range(l, r)) => {
                let v = Domain::Value(v);
                Domain::Range(l + v.clone(), r + v)
            } /*
              (Domain::Enum(a), Domain::Enum(b)) => {
                  let r = a.join(b);
                  for i in r {

                  }
                  todo!()
              },

              (r @ Domain::Range(_, _), Domain::Enum(e)) |
              (Domain::Enum(e), r @ Domain::Range(_, _)) =>
                  Domain::Enum(e.into_iter().map(|x| x+r).collect()),

              (Domain::Enum(e), Domain::Value(v)) |
              (Domain::Value(v), Domain::Enum(e)) => {
                  let v = Domain::Value(v);
                  Domain::Enum(e.into_iter().map(|x| x+v).collect())
              },
               */
        }
    }
}
