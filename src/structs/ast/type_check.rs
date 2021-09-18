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

impl TypeInfer for LocNode {
    fn type_infer(&self, env: &Env) -> Result<Type, Loc<TypeError>> {
        match &self.0 {
            Node::CrossProduct(r1, r2) => {
                let Record(r1t, name1) = get_node_table_type(r1, env)?;
                let Record(r2t, name2) = get_node_table_type(r2, env)?;
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
                        ].into_iter()
                    })
                    .collect();
                r.extend(intersect);
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
                let Record(r2t, name1) = get_node_table_type(r2, env)?;
                todo!()
            }
            Node::EquiJoin(r1, r2, k) => {
                let r1t = get_node_table_type(r1, env)?;
                let r2t = get_node_table_type(r2, env)?;
                if !(r1t.0.contains_key(k) && r2t.0.contains_key(k)) {
                    return Err(Loc(TypeError::FieldNotFound(k.clone()), r1.1.clone()));
                }
                let name_set: HashSet<Symbol> = r1t.0.keys().chain(r2t.0.keys()).cloned().collect();
                // if name_set.contains()
                todo!()
            }
            Node::NatureJoin(r1, r2) => {
                let r1t = get_node_table_type(r1, env)?;
                let r2t = get_node_table_type(r2, env)?;
                /*
                let t = r1t.unify(&r2t).map_err(|x| Loc(x, self.1))?;
                if let Type::Table(t) = t {
                    todo!()
                } else {
                    Err(Loc(TypeError::IsNotTable(t), self.1))
                }
                */
                todo!()
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
