/*
   Copyright (C) 2021-2021 imlyzh.

This file is part of RAE(Relational Algebra Engine).
This file is AST type check/infer of RAE.
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

use std::collections::{HashMap, HashSet};

use super::*;
use crate::{
    structs::Loc,
    type_system::{Env, Lines, Record, Type, TypeError},
};

pub trait TypeInfer {
    fn type_infer(&self, env: &Env) -> Result<Type, Loc<TypeError>>;
}

pub trait TypeCheck {
    fn type_check(&self, env: &Env) -> Result<Env, Loc<TypeError>>;
}

#[inline]
fn get_node_table_type(r: &Box<Loc<Node>>, env: &Env) -> Result<Record, Loc<TypeError>> {
    let rt = r.as_ref().type_infer(env)?;
    let Lines(rt) = rt
        .get_table()
        .ok_or_else(|| Loc(TypeError::IsNotTable, r.as_ref().1.clone()))?;
    Ok(rt.clone())
}

#[inline]
fn get_double_node_to_cross_product(
    r1t: HashMap<Symbol, Type>,
    r2t: HashMap<Symbol, Type>,
    name1: &String,
    name2: &String,
) -> Result<HashMap<Symbol, Type>, Loc<TypeError>> {
    let record_intersect_keys: Vec<&Symbol> = r1t
        .keys()
        .flat_map(|x| r2t.keys().map(move |y| (x, y)))
        .filter(|(x, y)| x == y)
        .map(|(x, _)| x)
        .collect();
    let mut r: HashMap<Symbol, Type> = HashMap::new();
    r.extend(
        r1t.iter()
            .filter(|(k, _)| !record_intersect_keys.contains(k))
            .map(|(k, v)| (k.clone(), v.clone())),
    );
    r.extend(
        r2t.iter()
            .filter(|(k, _)| !record_intersect_keys.contains(k))
            .map(|(k, v)| (k.clone(), v.clone())),
    );
    let intersect: HashMap<Symbol, Type> = record_intersect_keys
        .into_iter()
        .flat_map(|k| {
            assert!(k.1.is_none());
            vec![
                (
                    Symbol(name1.clone(), Some(k.0.clone())),
                    r1t.get(k).unwrap().clone(),
                ),
                (
                    Symbol(name2.clone(), Some(k.0.clone())),
                    r2t.get(k).unwrap().clone(),
                ),
            ]
            .into_iter()
        })
        .collect();
    r.extend(intersect);
    Ok(r)
}

impl TypeInfer for LocNode {
    fn type_infer(&self, env: &Env) -> Result<Type, Loc<TypeError>> {
        match &self.0 {
            Node::CrossProduct(r1, r2) => {
                let Record(r1t, name1) = get_node_table_type(r1, env)?;
                let Record(r2t, name2) = get_node_table_type(r2, env)?;
                let r = get_double_node_to_cross_product(r1t, r2t, &name1, &name2)?;
                Ok(Type::Table(Lines(Record(
                    r,
                    format!("{}*{}", name1, name2),
                ))))
            }
            Node::Union(r1, r2) => {
                let r1t = get_node_table_type(r1, env)?;
                let r2t = get_node_table_type(r2, env)?;
                if r1t != r2t {
                    return Err(Loc(TypeError::DoubleTableIsNotStyleLike(r1t, r2t), self.1));
                }
                Ok(Type::Table(Lines(r1t)))
            }
            Node::Difference(r1, r2) => {
                let r1t = get_node_table_type(r1, env)?;
                let r2t = get_node_table_type(r2, env)?;
                if r1t != r2t {
                    return Err(Loc(TypeError::DoubleTableIsNotStyleLike(r1t, r2t), self.1));
                }
                Ok(Type::Table(Lines(r1t)))
            }
            Node::Intersect(r1, r2) => {
                let r1t = get_node_table_type(r1, env)?;
                let r2t = get_node_table_type(r2, env)?;
                if r1t != r2t {
                    return Err(Loc(TypeError::DoubleTableIsNotStyleLike(r1t, r2t), self.1));
                }
                Ok(Type::Table(Lines(r1t)))
            }
            Node::Selection(r, f) => {
                let rt = get_node_table_type(r, env)?;
                todo!("verify filter type");
                Ok(Type::Record(rt))
            }
            Node::Projection(r, names) => {
                let rt = get_node_table_type(r, env)?;
                if !names.iter().all(|name| rt.0.contains_key(name)) {
                    return Err(Loc(TypeError::InValidProjectionNames, self.1.clone()));
                }
                let r = names
                    .iter()
                    .map(|name| {
                        let r = rt.0.get_key_value(name).unwrap();
                        (r.0.clone(), r.1.clone())
                    })
                    .collect();
                Ok(Type::Table(Lines(Record(r, rt.1))))
            }
            Node::Division(r1, r2) => {
                let Record(mut r1t, name1) = get_node_table_type(r1, env)?;
                let Record(r2t, name2) = get_node_table_type(r2, env)?;
                r2t.keys().for_each(|k| {
                    r1t.remove(k);
                });
                Ok(Type::Table(Lines(Record(
                    r1t,
                    format!("{}/{}", name1, name2),
                ))))
            }
            Node::InnerJoin(r1, r2, f) => {
                let Record(r1t, name1) = get_node_table_type(r1, env)?;
                let Record(r2t, name2) = get_node_table_type(r2, env)?;
                let r = get_double_node_to_cross_product(r1t, r2t, &name1, &name2)?;
                todo!("verify filter type");
                Ok(Type::Table(Lines(Record(
                    r,
                    format!("{}><{}", name1, name2),
                ))))
            }
            Node::EquiJoin(r1, r2, ks) => {
                let Record(r1t, name1) = get_node_table_type(r1, env)?;
                let Record(r2t, name2) = get_node_table_type(r2, env)?;
                let r = get_double_node_to_cross_product(r1t, r2t, &name1, &name2)?;
                ks.into_iter().try_for_each(|k| {
                    let n1 = Symbol(name1.clone(), Some(k.clone()));
                    let n2 = Symbol(name1.clone(), Some(k.clone()));
                    let r1 = r.get(&n1).unwrap();
                    let r2 = r.get(&n2).unwrap();
                    if r1 == r2 {
                        Ok(())
                    } else {
                        Err(Loc(
                            TypeError::EquiJoinKeysTypeUnifyError(n1, r1.clone(), n2, r2.clone()),
                            self.1.clone(),
                        ))
                    }
                })?;
                Ok(Type::Table(Lines(Record(
                    r,
                    format!("{}*{}", name1, name2),
                ))))
            }
            Node::NatureJoin(r1, r2) => {
                let Record(r1t, name1) = get_node_table_type(r1, env)?;
                let Record(r2t, name2) = get_node_table_type(r2, env)?;
                let mut r = get_double_node_to_cross_product(r1t, r2t, &name1, &name2)?;
                let items = r.iter().filter(|(Symbol(_, k), _)| k.is_some());
                let deleted_items: Vec<Symbol> = items
                    .clone()
                    .flat_map(move |x| items.clone().map(move |y| (x, y)))
                    .filter(|((xk, xv), (yk, yv))| xk.1 == yk.1 && xv == yv)
                    .flat_map(|((xk, _), (yk, _))| vec![xk, yk])
                    .cloned()
                    .collect();
                deleted_items.into_iter().for_each(|k| {
                    r.remove(&k);
                });
                Ok(Type::Table(Lines(Record(
                    r,
                    format!("{}*{}", name1, name2),
                ))))
            }
            Node::LeftJoin() => todo!(),
            Node::RightJoin() => todo!(),
            Node::FullJoin() => todo!(),
            Node::Reduce(reduce) => reduce.type_infer(env),
            Node::Table(tname) => {
                let r = env
                    .get_table(tname)
                    .ok_or_else(|| Loc(TypeError::TableNotFound(tname.clone()), self.1.clone()))?;
                Ok(Type::Table(r.clone()))
            }
            Node::Rename(_, _) => unreachable!(),
        }
    }
}

impl TypeInfer for LocItemReduce {
    fn type_infer(&self, env: &Env) -> Result<Type, Loc<TypeError>> {
        todo!()
    }
}
