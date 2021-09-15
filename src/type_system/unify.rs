use std::collections::HashMap;

use crate::structs::Symbol;

use super::{Domain, Lines, Optional, Record, SimpleType, Type, TypeError};

pub trait Unify {
    type Output;
    fn unify(&self, r: &Self) -> Result<Self::Output, TypeError>;
}

impl Unify for Type {
    type Output = Self;
    fn unify(&self, r: &Self) -> Result<Self::Output, TypeError> {
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
    fn unify(&self, r: &Self) -> Result<Self::Output, TypeError> {
        self.0.unify(&r.0)?;
        Ok(self.clone())
    }
}

impl Unify for SimpleType {
    type Output = Self;
    fn unify(&self, r: &Self) -> Result<Self::Output, TypeError> {
        let r = match (self.clone(), r.clone()) {
            (SimpleType::Int(d), SimpleType::Int(None))
            | (SimpleType::Int(None), SimpleType::Int(d)) => SimpleType::Int(d.clone()),
            (SimpleType::Int(d1), SimpleType::Int(d2)) => {
                SimpleType::Int(Some(d1.unwrap().unify(&d2.unwrap())?))
            }
            (SimpleType::Uint(d), SimpleType::Uint(None))
            | (SimpleType::Uint(None), SimpleType::Uint(d)) => SimpleType::Uint(d.clone()),
            (SimpleType::Uint(d1), SimpleType::Uint(d2)) => {
                SimpleType::Uint(Some(d1.unwrap().unify(&d2.unwrap())?))
            }
            (SimpleType::Float(d), SimpleType::Float(None))
            | (SimpleType::Float(None), SimpleType::Float(d)) => SimpleType::Float(d.clone()),
            (SimpleType::Float(d1), SimpleType::Float(d2)) => {
                SimpleType::Float(Some(d1.unwrap().unify(&d2.unwrap())?))
            }
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

impl<T> Unify for Domain<T> {
    type Output = Self;
    fn unify(&self, r: &Self) -> Result<Self::Output, TypeError> {
        todo!()
    }
}

impl Unify for Lines {
    type Output = Self;
    fn unify(&self, r: &Self) -> Result<Self::Output, TypeError> {
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
        .ok_or_else(|| TypeError::FieldNotFoundError(k.clone()))?;
    l.unify(r).map(|x| (k.clone(), x))
}

impl Unify for Record {
    type Output = Self;
    fn unify(&self, r: &Self) -> Result<Self::Output, TypeError> {
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
