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

pub mod ast;
pub mod plan;
pub mod plan_group;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    offset: usize,
    line: usize,
    col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loc<T>(pub T, pub Pos);

pub type LocExpr = Loc<Expr>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Add(Box<LocExpr>, Box<LocExpr>),
    Sub(Box<LocExpr>, Box<LocExpr>),
    Mul(Box<LocExpr>, Box<LocExpr>),
    Div(Box<LocExpr>, Box<LocExpr>),
    Mod(Box<LocExpr>, Box<LocExpr>),
    And(Box<LocExpr>, Box<LocExpr>),
    Or(Box<LocExpr>, Box<LocExpr>),
    Not(Box<LocExpr>),
    Value(LocValue),
}

pub type LocValue = Loc<Value>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Float(f64),
    String(String),
    Symbol(Symbol),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(pub String, pub Option<String>);
