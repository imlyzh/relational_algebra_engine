use super::plan;
use super::plan::Plan;
use super::{Expr, Symbol};

#[derive(Debug, Clone, PartialEq)]
struct PlanGroup {
    oper_item: OperItem,
    selection: Vec<FilterExpr>,
    projection: Vec<Symbol>,
    item_reduce: Option<ReduceOperator>,
}

#[derive(Debug, Clone, PartialEq)]
enum OperItem {
    CartesianProduct(Box<PlanGroup>, Box<PlanGroup>),
    Difference(Box<PlanGroup>, Box<PlanGroup>),
    Intersect(Box<PlanGroup>, Box<PlanGroup>),
    Division(Box<PlanGroup>, Box<PlanGroup>),
    Union(Box<PlanGroup>, Box<PlanGroup>),
    Table(String),
}

#[derive(Debug, Clone, PartialEq)]
enum ReduceOperator {
    Sum,
    Avg,
    Count,
    Max,
    Min,
}

#[derive(Debug, Clone, PartialEq)]
enum FilterExpr {
    Or(Vec<Box<CompExpr>>),
    Not(Box<CompExpr>),
    Comp(Box<CompExpr>),
    Range(u64, u64),
    GetItem(u64),
    GetFirst,
    GetLast,
}

#[derive(Debug, Clone, PartialEq)]
enum CompExpr {
    Eq(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    In(Box<Expr>, Box<PlanGroup>),
}

impl From<Box<Plan>> for Box<PlanGroup> {
    fn from(i: Box<Plan>) -> Self {
        Box::new(i.into())
    }
}

impl From<Box<Plan>> for PlanGroup {
    fn from(i: Box<Plan>) -> Self {
        (*i).into()
    }
}

impl From<Plan> for PlanGroup {
    fn from(i: Plan) -> Self {
        let mut selection: Vec<FilterExpr> = vec![];
        let mut projection: Vec<Symbol> = vec![];
        let mut item_reduce: Option<ReduceOperator> = None;
        let oper_item: OperItem = load_plan(i, &mut selection, &mut projection, &mut item_reduce);
        selection.reverse();
        projection.reverse();
        Self {
            oper_item,
            selection,
            projection,
            item_reduce,
        }
    }
}

fn load_plan(
    i: Plan,
    selection: &mut Vec<FilterExpr>,
    projection: &mut Vec<Symbol>,
    item_reduce: &mut Option<ReduceOperator>,
) -> OperItem {
    match i {
        Plan::CartesianProduct(a, b) => OperItem::CartesianProduct(a.into(), b.into()),
        Plan::Difference(a, b) => OperItem::Difference(a.into(), b.into()),
        Plan::Intersect(a, b) => OperItem::Intersect(a.into(), b.into()),
        Plan::Division(a, b) => OperItem::Division(a.into(), b.into()),
        Plan::Union(a, b) => OperItem::Union(a.into(), b.into()),
        Plan::Table(t) => OperItem::Table(t),
        Plan::Selection(a, b) => {
            if let plan::FilterExpr::And(v) = *b {
                let iter = v.into_iter().map(|x| FilterExpr::Comp(x.into()));
                selection.extend(iter);
            } else {
                selection.push(b.into());
            }
            load_plan(*a, selection, projection, item_reduce)
        }
        Plan::Projection(a, mut b) => {
            projection.append(&mut b);
            load_plan(*a, selection, projection, item_reduce)
        }
        Plan::Reduce(a) => {
            if item_reduce.is_some() {
                panic!("Type check does not exist?"); // 要么是有人乱搞，要么是没过类型检查
            }
            match a {
                plan::ItemReduce::Count(a) => {
                    item_reduce.replace(ReduceOperator::Count);
                    load_plan(*a, selection, projection, item_reduce)
                }
                plan::ItemReduce::Sum(a, s) => {
                    item_reduce.replace(ReduceOperator::Sum);
                    projection.push(s);
                    load_plan(*a, selection, projection, item_reduce)
                }
                plan::ItemReduce::Avg(a, s) => {
                    item_reduce.replace(ReduceOperator::Avg);
                    projection.push(s);
                    load_plan(*a, selection, projection, item_reduce)
                }
                plan::ItemReduce::Max(a, s) => {
                    item_reduce.replace(ReduceOperator::Max);
                    projection.push(s);
                    load_plan(*a, selection, projection, item_reduce)
                }
                plan::ItemReduce::Min(a, s) => {
                    item_reduce.replace(ReduceOperator::Min);
                    projection.push(s);
                    load_plan(*a, selection, projection, item_reduce)
                }
            }
        }
    }
}

impl From<Box<plan::FilterExpr>> for Box<FilterExpr> {
    fn from(i: Box<plan::FilterExpr>) -> Self {
        Box::new(i.into())
    }
}

impl From<Box<plan::FilterExpr>> for FilterExpr {
    fn from(i: Box<plan::FilterExpr>) -> Self {
        (*i).into()
    }
}

impl From<plan::FilterExpr> for FilterExpr {
    fn from(i: plan::FilterExpr) -> Self {
        match i {
            plan::FilterExpr::And(_) => unreachable!(),
            plan::FilterExpr::Or(a) => FilterExpr::Or(a.into_iter().map(|x| x.into()).collect()),
            plan::FilterExpr::Not(a) => FilterExpr::Not(a.into()),
            plan::FilterExpr::Comp(a) => FilterExpr::Comp(a.into()),
            plan::FilterExpr::Range(a, b) => FilterExpr::Range(a, b),
            plan::FilterExpr::GetItem(s) => FilterExpr::GetItem(s),
            plan::FilterExpr::GetFirst => FilterExpr::GetFirst,
            plan::FilterExpr::GetLast => FilterExpr::GetLast,
        }
    }
}

impl From<Box<plan::CompExpr>> for CompExpr {
    fn from(i: Box<plan::CompExpr>) -> Self {
        (*i).into()
    }
}

impl From<Box<plan::CompExpr>> for Box<CompExpr> {
    fn from(i: Box<plan::CompExpr>) -> Self {
        Box::new(i.into())
    }
}

impl From<plan::CompExpr> for CompExpr {
    fn from(i: plan::CompExpr) -> Self {
        match i {
            plan::CompExpr::Eq(a, b) => CompExpr::Eq(a, b),
            plan::CompExpr::Lt(a, b) => CompExpr::Lt(a, b),
            plan::CompExpr::Gt(a, b) => CompExpr::Gt(a, b),
            plan::CompExpr::In(a, b) => CompExpr::In(a, b.into()),
        }
    }
}
