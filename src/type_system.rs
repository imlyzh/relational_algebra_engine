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

pub mod domain;
pub mod unify;

use std::collections::HashMap;

use crate::structs::Symbol;

// type check\infer and unify error
pub enum TypeError {
    TypeUnifyError(Type, Type),
    NameNotFoundError(Symbol),
    TableNotFoundError(TableName),
    FieldNotFoundError(Symbol),
}

// table info

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Env(pub HashMap<TableName, Lines>, pub HashMap<Symbol, Type>);

impl Env {
    pub fn get_table(&self, name: &TableName) -> Option<Lines> {
        self.0.get(name).cloned()
    }
    pub fn get_type(&self, name: &Symbol) -> Option<Type> {
        self.1.get(name).cloned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Optional(Optional),
    Record(Record),
    Simple(SimpleType),
    TableName(String),
    Table(Lines),
}

macro_rules! impl_is_type {
    ($s:ident, $t:ident) => {
        pub fn $s(&self) -> bool {
            if let Type::$t(_) = self {
                true
            } else {
                false
            }
        }
    };
}

impl Type {
    impl_is_type!(is_optional, Optional);
    impl_is_type!(is_record, Record);
    impl_is_type!(is_simple_type, Simple);
    impl_is_type!(is_table, Table);
    impl_is_type!(is_table_name, TableName);
}

// type or null

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Optional(pub Box<Type>);

// adhoc-union type

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Union(pub Vec<Type>);

// record(struct) type

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record(pub HashMap<Symbol, Type>);

// Reletation etc.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lines(pub Record);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableName(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleType {
    Int(Option<Domain<i64>>),
    Uint(Option<Domain<u64>>),
    Float(Option<Domain<u64>>),
    String(Vec<String>),
}

// refinement type
#[derive(Debug, Clone, PartialEq, Eq)]
enum Domain<T> {
    Range(Box<Domain<T>>, Box<Domain<T>>),
    // Enum(Vec<Domain<T>>),
    Value(T),
}
