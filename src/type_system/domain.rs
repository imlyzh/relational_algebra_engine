/*
   Copyright (C) 2021-2021 imlyzh.

This file is part of RAE(Relational Algebra Engine).
This file is Domain Type of RAE.
RAE is free software; you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free
Software Foundation; either version 3, or (at your option) any later
version.
RAE is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or
FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
for more details.
You should have received a copy of the GNU General Public License
along with RAE; see the file COPYING3.  If not see
<http://www.gnu.org/licenses/>.  */

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
