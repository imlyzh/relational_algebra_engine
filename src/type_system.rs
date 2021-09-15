/*
   Copyright (C) 2021-2021 imlyzh.

This file is part of RAE(Relational Algebra Engine).
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


use std::{collections::HashMap, ops};

use crate::structs::Symbol;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {

}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Optional(pub Box<Type>);


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Union(pub Vec<Type>);


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record(pub HashMap<Symbol, Type>);


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Special(pub String, pub Vec<Type>);


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleType {
    Null,
    Int(Domain<i64>),
    Uint(Domain<u64>),
    Float(Domain<u64>),
    String(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Domain<T> {
    Range(Box<Domain<T>>, Box<Domain<T>>),
    // Enum(Vec<Domain<T>>),
    Value(T),
}


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
            (Domain::Range(l1, r1), Domain::Range(l2, r2)) => Domain::Range(l1+l2, r1+r2),
            (Domain::Value(v1), Domain::Value(v2)) => Domain::Value(v1+v2),

            (Domain::Range(l, r), Domain::Value(v)) |
            (Domain::Value(v), Domain::Range(l, r)) => {
                let v = Domain::Value(v);
                Domain::Range(l+v.clone(), r+v)
            },

            /*
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