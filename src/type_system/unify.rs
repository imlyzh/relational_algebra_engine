/*
   Copyright (C) 2021-2021 imlyzh.

This file is part of RAE(Relational Algebra Engine).
This file is Type Unify of RAE.
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

use std::collections::HashMap;
use std::cmp;

use crate::structs::Symbol;

use super::{Domain, Lines, Optional, Record, SimpleType, Type, TypeError};

pub trait Unify {
    type Output;
    type Error;
    fn unify(&self, r: &Self) -> Result<Self::Output, Self::Error>;
}

impl Unify for Type {
    type Output = Self;
    type Error = TypeError;
    fn unify(&self, r: &Self) -> Result<Self::Output, Self::Error> {
        match (self, r) {
            (Type::Optional(t1), Type::Optional(t2)) => t1.unify(t2).map(Type::Optional),
            (Type::Optional(Optional(t1)), t2) => t1.unify(t2),
            (Type::Record(t1), Type::Record(t2)) => t1.unify(t2).map(Type::Record),
            (Type::Simple(t1), Type::Simple(t2)) => t1.unify(t2).map(Type::Simple),
            (Type::Table(t1), Type::Table(t2)) => t1.unify(t2).map(Type::Table),
            (Type::TableName(t1), Type::TableName(t2)) => {
                if t1 == t2 {
                    Ok(self.clone())
                } else {
                    Err(TypeError::TypeUnifyError(self.clone(), r.clone()))
                }
            }
            (Type::TableName(_t1), Type::Table(_t2)) | (Type::Table(_t2), Type::TableName(_t1)) => {
                unreachable!()
            }
            _ => Err(TypeError::TypeUnifyError(self.clone(), r.clone())),
        }
    }
}

impl Unify for Optional {
    type Output = Self;
    type Error = TypeError;
    fn unify(&self, r: &Self) -> Result<Self::Output, Self::Error> {
        self.0.unify(&r.0)?;
        Ok(self.clone())
    }
}

// error_domain_to_type

macro_rules! impl_edt {
    ($id:ident, $tp:ident, $en:ident) => {
        fn $id((l, r): (Domain<$tp>, Domain<$tp>)) -> TypeError {
            TypeError::TypeUnifyError(
                Type::Simple(SimpleType::$en(Some(l))),
                Type::Simple(SimpleType::$en(Some(r)))
            )
        }
    };
}

impl_edt!(edti, i64, Int);
impl_edt!(edtu, u64, Uint);
impl_edt!(edtf, f64, Float);


impl Unify for SimpleType {
    type Output = Self;
    type Error = TypeError;
    fn unify(&self, r: &Self) -> Result<Self::Output, Self::Error> {
        let r = match (self.clone(), r.clone()) {
            (SimpleType::Int(d), SimpleType::Int(None))
            | (SimpleType::Int(None), SimpleType::Int(d)) => SimpleType::Int(d.clone()),
            (SimpleType::Int(d1), SimpleType::Int(d2)) =>
                SimpleType::Int(Some(d1.unwrap().unify(&d2.unwrap()).map_err(edti)?)),
            (SimpleType::Uint(d), SimpleType::Uint(None))
            | (SimpleType::Uint(None), SimpleType::Uint(d)) => SimpleType::Uint(d.clone()),
            (SimpleType::Uint(d1), SimpleType::Uint(d2)) =>
                SimpleType::Uint(Some(d1.unwrap().unify(&d2.unwrap()).map_err(edtu)?)),
            (SimpleType::Float(d), SimpleType::Float(None))
            | (SimpleType::Float(None), SimpleType::Float(d)) => SimpleType::Float(d.clone()),
            (SimpleType::Float(d1), SimpleType::Float(d2)) =>
                SimpleType::Float(Some(d1.unwrap().unify(&d2.unwrap()).map_err(edtf)?)),
            (SimpleType::String(d1), SimpleType::String(d2)) => {
                if d1.len() == 0 {
                    SimpleType::String(d1.clone())
                } else {
                    if d2.iter().all(|x| d2.contains(x)) {
                        return Ok(SimpleType::String(d1.clone()));
                    }
                    return Err(TypeError::TypeUnifyError(
                        Type::Simple(self.clone()),
                        Type::Simple(r.clone()),
                    ));
                }
            }
            _ => {
                return Err(TypeError::TypeUnifyError(
                    Type::Simple(self.clone()),
                    Type::Simple(r.clone()),
                ))
            }
        };
        Ok(r)
    }
}

impl<T: Clone + cmp::PartialOrd + cmp::PartialOrd> Unify for Domain<T> {
    type Output = Self;
    type Error = (Domain<T>, Domain<T>);
    fn unify(&self, r: &Self) -> Result<Self::Output, Self::Error> {
        let ret = match (self, r) {
            (Domain::Range(l1, r1), Domain::Range(l2, r2)) =>
                if l1 <= l2 && r1 >= r2 {
                    self
                } else {
                    return Err((self.clone(), r.clone()));
                },
            (Domain::Range(l, r), Domain::Value(_)) => todo!(),
            (Domain::Value(_), Domain::Range(_, _)) => todo!(),
            (Domain::Value(_), Domain::Value(_)) => todo!(),
        };
        Ok(ret.clone())
    }
}

impl Unify for Lines {
    type Output = Self;
    type Error = TypeError;
    fn unify(&self, r: &Self) -> Result<Self::Output, Self::Error> {
        self.0.unify(&r.0).map(Lines)
    }
}

#[inline]
fn merge_double_map_from_key(
    k: &Symbol,
    l: &HashMap<Symbol, Type>,
    r: &HashMap<Symbol, Type>,
) -> Result<(Symbol, Type), TypeError> {
    let l = l.get(k).unwrap();
    let r = r
        .get(k)
        .ok_or_else(|| TypeError::FieldNotFound(k.clone()))?;
    l.unify(r).map(|x| (k.clone(), x))
}

impl Unify for Record {
    type Output = Self;
    type Error = TypeError;
    fn unify(&self, r: &Self) -> Result<Self::Output, Self::Error> {
        let nullables: Vec<_> = self.0.iter().filter(|(_, v)| v.is_optional()).collect();
        if self.0.len() == r.0.len() {
            let r: Result<HashMap<Symbol, Type>, _> = self
                .0
                .keys()
                .map(|k| merge_double_map_from_key(k, &self.0, &r.0))
                .collect();
            let r = r?;
            Ok(Record(r))
        } else if self.0.len() - nullables.len() == r.0.len() {
            let nonnulls: Result<HashMap<Symbol, Type>, _> = self
                .0
                .iter()
                .filter(|(_, v)| !v.is_optional())
                .map(|(k, _)| k)
                .map(|k| merge_double_map_from_key(k, &self.0, &r.0))
                .collect();
            let mut ret = nonnulls?;
            let nullables: Result<HashMap<Symbol, Type>, _> = nullables
                .into_iter()
                .map(|(k, _)| k)
                .map(|k| {
                    let l = self.0.get(k).unwrap();
                    if let Some(r) = r.0.get(k) {
                        Ok((k.clone(), l.unify(r)?))
                    } else {
                        Ok((k.clone(), l.clone()))
                    }
                })
                .collect();
            let nullables = nullables?;
            ret.extend(nullables);
            Ok(Record(ret))
        } else {
            Err(TypeError::TypeUnifyError(
                Type::Record(self.clone()),
                Type::Record(r.clone()),
            ))
        }
    }
}
