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

use super::*;
use crate::{
    structs::Loc,
    type_system::{Env, Type, TypeError},
};

pub trait TypeInfer {
    fn type_infer(&self, env: Env) -> Result<(Env, Type), Loc<TypeError>>;
}

pub trait TypeCheck {
    fn type_check(&self, env: Env) -> Result<Env, Loc<TypeError>>;
}

impl TypeInfer for LocNode {
    fn type_infer(&self, env: Env) -> Result<(Env, Type), Loc<TypeError>> {
        match &self.0 {
            Node::CrossProduct(r1, r2) => {
                todo!()
            }
            Node::Union(r1, r2) => {
                todo!()
            }
            Node::Difference(r1, r2) => {
                todo!()
            }
            Node::Intersect(r1, r2) => {
                todo!()
            }
            Node::Selection(r, f) => {
                todo!()
            }
            Node::Projection(r, names) => {
                todo!()
            }
            Node::Division(r1, r2) => {
                todo!()
            }
            Node::InnerJoin(r1, r2, f) => {
                todo!()
            }
            Node::EquiJoin(r1, r2, name) => {
                todo!()
            }
            Node::NatureJoin(r1, r2) => {
                let (env, r1t) = r1.as_ref().type_infer(env)?;
                let (env, r2t) = r2.as_ref().type_infer(env)?;
                // r1t.unify(r2t)?
                todo!()
            }
            Node::LeftJoin() => todo!(),
            Node::RightJoin() => todo!(),
            Node::FullJoin() => todo!(),
            Node::Reduce(reduce) => reduce.type_infer(env),
            Node::Table(tname) => {
                let r = env.get_table(tname).ok_or_else(|| {
                    Loc(TypeError::TableNotFoundError(tname.clone()), self.1.clone())
                })?;
                Ok((env, Type::Table(r)))
            }
            Node::Rename(_, _) => unreachable!(),
        }
    }
}

impl TypeInfer for LocItemReduce {
    fn type_infer(&self, env: Env) -> Result<(Env, Type), Loc<TypeError>> {
        todo!()
    }
}
