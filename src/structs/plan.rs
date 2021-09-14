use super::{Expr, Symbol};



#[derive(Debug, Clone, PartialEq)]
pub enum Plan {
    CartesianProduct(Box<Plan>, Box<Plan>), // 笛卡尔积
    Union(Box<Plan>, Box<Plan>),            // 并集
    Difference(Box<Plan>, Box<Plan>),       // 差集
    Intersect(Box<Plan>, Box<Plan>),        // 交集
    Selection(Box<Plan>, Box<FilterExpr>),  // 选择
    Projection(Box<Plan>, Vec<Symbol>),     // 投影
    Division(Box<Plan>, Box<Plan>),         // 除
    Reduce(ItemReduce),                     // 聚合
    Table(String)
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
