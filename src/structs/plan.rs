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

use super::{Expr, Symbol};

#[derive(Debug, Clone, PartialEq)]
pub enum Plan {
    Product(Box<Plan>, Box<Plan>),         // 笛卡尔积
    Union(Box<Plan>, Box<Plan>),           // 并集
    Difference(Box<Plan>, Box<Plan>),      // 差集
    Intersect(Box<Plan>, Box<Plan>),       // 交集
    Selection(Box<Plan>, Box<FilterExpr>), // 选择
    Projection(Box<Plan>, Vec<Symbol>),    // 投影
    Division(Box<Plan>, Box<Plan>),        // 除
    Reduce(ItemReduce),                    // 聚合
    Table(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemReduce {
    Count(Box<Plan>),
    Sum(Box<Plan>, Symbol),
    Avg(Box<Plan>, Symbol),
    Max(Box<Plan>, Symbol),
    Min(Box<Plan>, Symbol),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterExpr {
    And(Vec<Box<CompExpr>>),
    Or(Vec<Box<CompExpr>>),
    Not(Box<CompExpr>),
    Comp(Box<CompExpr>),
    Range(u64, u64),
    GetItem(u64),
    GetFirst,
    GetLast,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompExpr {
    Eq(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    In(Box<Expr>, Box<Plan>),
}
