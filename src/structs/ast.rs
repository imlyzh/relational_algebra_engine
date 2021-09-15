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
pub enum Node {
    CrossProduct(Box<Node>, Box<Node>),                 // 笛卡尔积
    Union(Box<Node>, Box<Node>),                        // 并集
    Difference(Box<Node>, Box<Node>),                   // 差集
    Intersect(Box<Node>, Box<Node>),                    // 交集
    Selection(Box<Node>, Vec<FilterExpr>),              // 选择
    Projection(Box<Node>, Vec<Symbol>),                 // 投影
    Division(Box<Node>, Box<Node>),                     // 除
    Rename(Box<Node>, Vec<(Symbol, Symbol)>),           // 重命名
    InnerJoin(Box<Node>, Box<Node>, Vec<FilterExpr>),   // 内连接
    EquiJoin(Box<Node>, Box<Node>, String),             // 等值连接
    NatureJoin(Box<Node>, Box<Node>),                   // 自然连接
    LeftJoin(),                                         // 左连接 todo
    RightJoin(),                                        // 右连接 todo
    FullJoin(),                                         // 全连接 todo
    Reduce(ItemReduce),                                 // 聚合
    Table(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemReduce {
    Count(Box<Node>),
    Sum(Box<Node>, Symbol),
    Avg(Box<Node>, Symbol),
    Max(Box<Node>, Symbol),
    Min(Box<Node>, Symbol),
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
    In(Box<Expr>, Box<Node>),
}