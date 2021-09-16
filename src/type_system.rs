/*
   Copyright (C) 2021-2021 imlyzh.

This file is part of RAE(Relational Algebra Engine).
This file is Type System of RAE.
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

pub mod domain;
pub mod unify;

use std::collections::HashMap;

use crate::structs::Symbol;

// type check\infer and unify error
pub enum TypeError {
    IsNotTable,
    InValidProjectionNames,
    NameNotFound(Symbol),
    FieldNotFound(Symbol),
    TableNotFound(TableName),
    TypeUnifyError(Type, Type),
    DoubleTableIsNotStyleLike(Record, Record),
}

// table info

#[derive(Debug, Clone, PartialEq)]
pub struct Env(
    pub HashMap<TableName, Lines>,
    // pub HashMap<Symbol, Type>
);

impl Env {
    pub fn get_table(&self, name: &TableName) -> Option<&Lines> {
        self.0.get(name)
    }
    /*
    pub fn get_type(&self, name: &Symbol) -> Option<Type> {
        self.1.get(name).cloned()
    }
    //  */
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Optional(Optional),
    Record(Record),
    Simple(SimpleType),
    TableName(TableName),
    Table(Lines),
}

macro_rules! impl_is_type {
    ($s:ident, $en:ident) => {
        pub fn $s(&self) -> bool {
            if let Type::$en(_) = self {
                true
            } else {
                false
            }
        }
    };
}

macro_rules! impl_get_type {
    ($s:ident, $en:ident, $t:ident) => {
        pub fn $s(&self) -> Option<&$t> {
            if let Type::$en(r) = self {
                Some(r)
            } else {
                None
            }
        }
    };
}

impl Type {
    impl_is_type!(is_table, Table);
    impl_is_type!(is_record, Record);
    impl_is_type!(is_optional, Optional);
    impl_is_type!(is_simple_type, Simple);
    impl_is_type!(is_table_name, TableName);

    impl_get_type!(get_table, Table, Lines);
    impl_get_type!(get_record, Record, Record);
    impl_get_type!(get_optional, Optional, Optional);
    impl_get_type!(get_simple_type, Simple, SimpleType);
    impl_get_type!(get_table_name, TableName, TableName);

    pub fn get_table_from_env<'a>(&'a self, env: &'a Env) -> Option<&'a Lines> {
        if let Type::Table(r) = self {
            Some(r)
        } else if let Type::TableName(x) = self {
            env.get_table(x)
        } else {
            None
        }
    }
}

// type or null

#[derive(Debug, Clone, PartialEq)]
pub struct Optional(pub Box<Type>);

// adhoc-union type

#[derive(Debug, Clone, PartialEq)]
pub struct Union(pub Vec<Type>);

// record(struct) type

#[derive(Debug, Clone)]
pub struct Record(pub HashMap<Symbol, Type>);

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() == other.0.len() {
            self.0.keys().all(|k| {
                if let Some(v) = other.0.get(k) {
                    self.0.get(k).unwrap() == v
                } else {
                    false
                }
            })
        } else {
            false
        }
    }
}

// Reletation etc.

#[derive(Debug, Clone, PartialEq)]
pub struct Lines(pub Record);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableName(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleType {
    Int(Option<Domain<i64>>),
    Uint(Option<Domain<u64>>),
    Float(Option<Domain<f64>>),
    String(Vec<String>),
}

// refinement type
#[derive(Debug, Clone, PartialEq, Eq)]
enum Domain<T> {
    Range(T, T),
    // Enum(Vec<Domain<T>>),
    Value(T),
}
