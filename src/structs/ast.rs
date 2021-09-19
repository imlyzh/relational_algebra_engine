/*
   Copyright (C) 2021-2021 imlyzh.

This file is part of RAE(Relational Algebra Engine).
This file is AST(Abstract Syntax Tree) of RAE.
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

pub mod type_check;

use crate::type_system::TableName;

use super::{Loc, LocExpr, Symbol};

pub type LocNode = Loc<Node>;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    CrossProduct(Box<LocNode>, Box<LocNode>),    // 笛卡尔积
    Union(Box<LocNode>, Box<LocNode>),           // 并集
    Difference(Box<LocNode>, Box<LocNode>),      // 差集
    Intersect(Box<LocNode>, Box<LocNode>),       // 交集
    Selection(Box<LocNode>, Vec<LocFilterExpr>), // 选择
    Projection(Box<LocNode>, Vec<Symbol>),       // 投影
    Division(Box<LocNode>, Box<LocNode>),        // 除
    Rename(Box<LocNode>, Vec<(Symbol, Symbol)>), // 重命名
    InnerJoin(Box<LocNode>, Box<LocNode>, Vec<FilterExpr>), // 内连接
    EquiJoin(Box<LocNode>, Box<LocNode>, Vec<String>), // 等值连接
    NatureJoin(Box<LocNode>, Box<LocNode>),      // 自然连接
    LeftJoin(),                                  // 左连接 todo
    RightJoin(),                                 // 右连接 todo
    FullJoin(),                                  // 全连接 todo
    Reduce(LocItemReduce),                       // 聚合
    Table(TableName),
}

pub type LocItemReduce = Loc<ItemReduce>;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemReduce {
    Count(Box<LocNode>),
    Sum(Box<LocNode>, Symbol),
    Avg(Box<LocNode>, Symbol),
    Max(Box<LocNode>, Symbol),
    Min(Box<LocNode>, Symbol),
}

pub type LocFilterExpr = Loc<FilterExpr>;

#[derive(Debug, Clone, PartialEq)]
pub enum FilterExpr {
    And(Vec<Box<LocCompExpr>>),
    Or(Vec<Box<LocCompExpr>>),
    Not(Box<LocCompExpr>),
    Comp(Box<LocCompExpr>),
    Range(u64, u64),
    GetItem(u64),
    GetFirst,
    GetLast,
}

pub type LocCompExpr = Loc<CompExpr>;

#[derive(Debug, Clone, PartialEq)]
pub enum CompExpr {
    Eq(Box<LocExpr>, Box<LocExpr>),
    Lt(Box<LocExpr>, Box<LocExpr>),
    Gt(Box<LocExpr>, Box<LocExpr>),
    In(Box<LocExpr>, Box<LocExpr>),
}
