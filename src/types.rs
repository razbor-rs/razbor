use smol_str::SmolStr;

use crate::expr::Location;
use crate::path::TypeRow;
use crate::expr::RawExpr;
use crate::expr::Ranged;
use crate::path::PathTable;
use crate::path::RzPath;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Endianness {
    Le,
    Be,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpr {
    Top,
    Bottom,

    Bool,

    Int,
    U(usize, Endianness),
    S(usize, Endianness),

    Arr(Box<Ranged<TypeExpr>>, Ranged<Size>),
    Str(Ranged<Size>),

    And(Vec<Ranged<TypeExpr>>),
    Or(Vec<Ranged<TypeExpr>>),

    Prod(Vec<RzPath>),
    Ref(RzPath),

    Liq(Box<TypeExpr>, Vec<Ranged<Rel>>),
    Val(Value, Box<TypeExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    rels: Vec<Ranged<Rel>>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rel {
    Lt(Ranged<RelExpr>, Ranged<RelExpr>),
    Le(Ranged<RelExpr>, Ranged<RelExpr>),
    Eq(Ranged<RelExpr>, Ranged<RelExpr>),
    Ge(Ranged<RelExpr>, Ranged<RelExpr>),
    Gt(Ranged<RelExpr>, Ranged<RelExpr>),
}



#[derive(Debug, Clone, PartialEq)]
pub enum RelExpr {
    Hole,
    Sizeof,
    Integer(SmolStr),
    Ref(RzPath),
    Size(Size),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Integer(SmolStr),
}

fn is_rel_name(name: &str) -> bool {
    ["lt", "le", "eq", "ge", "gt"]
        .contains(&name)
}

fn as_rel_expr(expr: &Ranged<RawExpr>) -> Result<Ranged<RelExpr>, ExprToTypeError> {
    let pred_expr =
        match &expr.data {
            RawExpr::Decimal(num) | RawExpr::Hexdecimal(num) =>
                RelExpr::Integer(num.clone()),
            RawExpr::Name(n) if n == "_" =>
                RelExpr::Hole,
            RawExpr::Apply { head, body } if head.data == "sizeof" => {
                if body.get(0).map(|e| &e.data) == Some(&RawExpr::Name("_".into())) {
                    RelExpr::Sizeof
                }
                else {
                    return Err(ExprToTypeError::InvalidRelExpr(expr.range.unwrap()))
                }
            },
            RawExpr::Path(path) =>
                RelExpr::Ref(path.clone()),
            _ => return Err(ExprToTypeError::InvalidRelExpr(expr.range.unwrap())),
        };

    Ok(Ranged {
        data: pred_expr,
        range: expr.range
    })
}

fn as_rel(expr: &Ranged<RawExpr>) -> Result<Ranged<Rel>, ExprToTypeError> {
    match &expr.data {
        RawExpr::Apply { head, body } if is_rel_name(&head.data) => {
            let (left, right) = body.get(0..2)
                .map(|p| (&p[0], &p[1]))
                .ok_or(ExprToTypeError::InvalidRel(expr.range.unwrap()))?;
            if body.len() > 2 {
                return Err(ExprToTypeError::InvalidRel(expr.range.unwrap()))
            }

            let left = as_rel_expr(left)?;
            let right = as_rel_expr(right)?;

            let cons =
                match &*head.data {
                    "lt" => Rel::Lt,
                    "le" => Rel::Le,
                    "eq" => Rel::Eq,
                    "ge" => Rel::Ge,
                    "gt" => Rel::Gt,
                    _ => unreachable!()
                };

            let pred = Ranged {
                data: cons(left, right),
                range: expr.range,
            };

            Ok(pred)
        },
        _ => Err(ExprToTypeError::InvalidRel(expr.range.unwrap()))
    }
}

fn as_rel_list(expr: &Ranged<RawExpr>) -> Result<Vec<Ranged<Rel>>, ExprToTypeError> {
    match &expr.data {
        RawExpr::Apply { head, body } if head.data == "and" => {
            body.iter()
                .map(as_rel)
                .collect()
        },
        _ => as_rel(expr).map(|r| vec![r])
    }
}

fn as_size(expr: &Ranged<RawExpr>) -> Result<Ranged<Size>, ExprToTypeError> {
    let mut rels = vec![];
    match &expr.data {
        RawExpr::Name(n) if n == "_" => (),
        RawExpr::Decimal(num)
        | RawExpr::Hexdecimal(num) =>
            rels.push(
                Ranged {
                    data: Rel::Eq(
                        Ranged::new(RelExpr::Hole),
                        Ranged {
                            data: RelExpr::Integer(num.clone()),
                            range: expr.range,
                        }
                    ),
                    range: expr.range
                }
            ),
        RawExpr::Path(path) =>
            rels.push(Ranged {
                data: Rel::Eq(
                    Ranged::new(RelExpr::Hole),
                    Ranged {
                        data: RelExpr::Ref(path.clone()),
                        range: expr.range
                    }
                ),
                range: expr.range
            }),
        RawExpr::Apply { head, body } if head.data == "and" => {
            for e in body {
                let sz = as_size(e)?;
                rels.extend(sz.data.rels);
            }
        },
        RawExpr::Apply { head, .. } if is_rel_name(&head.data) => {
            let rel = as_rel(expr)?;
            rels.push(rel)
        },
        _ => return Err(ExprToTypeError::InvalidSize(expr.range.unwrap())),
    }

    Ok(Ranged {
        data: Size { rels },
        range: expr.range,
    })
}

fn as_simple_type(expr: &Ranged<RawExpr>) -> Option<Ranged<TypeExpr>> {
    use Endianness::*;

    let ty =
        match &expr.data {
            RawExpr::Name(n) if n == "_" =>
                TypeExpr::Top,
            RawExpr::Name(n) if n == "never" =>
                TypeExpr::Bottom,

            RawExpr::Name(n) if n == "bool" =>
                TypeExpr::Bool,
            RawExpr::Name(n) if n == "true" =>
                TypeExpr::Val(
                    Value::Boolean(true),
                    Box::new(TypeExpr::Bool)
                ),
            RawExpr::Name(n) if n == "false" =>
                TypeExpr::Val(
                    Value::Boolean(false),
                    Box::new(TypeExpr::Bool)
                ),

            RawExpr::Name(n) if n == "int" =>
                TypeExpr::Int,

            RawExpr::Name(n) if n == "u1" =>
                TypeExpr::U(1, Be),
            RawExpr::Name(n) if n == "u2" =>
                TypeExpr::U(2, Be),
            RawExpr::Name(n) if n == "u4" =>
                TypeExpr::U(4, Be),
            RawExpr::Name(n) if n == "u8" =>
                TypeExpr::U(8, Be),
            RawExpr::Name(n) if n == "u2le" =>
                TypeExpr::U(2, Le),
            RawExpr::Name(n) if n == "u4le" =>
                TypeExpr::U(4, Le),
            RawExpr::Name(n) if n == "u8le" =>
                TypeExpr::U(8, Le),

            RawExpr::Name(n) if n == "s1" =>
                TypeExpr::S(1, Be),
            RawExpr::Name(n) if n == "s2" =>
                TypeExpr::S(2, Be),
            RawExpr::Name(n) if n == "s4" =>
                TypeExpr::S(4, Be),
            RawExpr::Name(n) if n == "s8" =>
                TypeExpr::S(8, Be),
            RawExpr::Name(n) if n == "s2le" =>
                TypeExpr::S(2, Le),
            RawExpr::Name(n) if n == "s4le" =>
                TypeExpr::S(4, Le),
            RawExpr::Name(n) if n == "s8le" =>
                TypeExpr::S(8, Le),

            RawExpr::Decimal(s) | RawExpr::Hexdecimal(s) =>
                TypeExpr::Val(
                    Value::Integer(s.clone()),
                    Box::new(TypeExpr::Int)
                ),
            _ => return None,
        };

    Some(Ranged {
        data: ty,
        range: expr.range,
    })
}

#[derive(Clone, Copy, Debug, PartialEq,)]
pub enum ExprToTypeError {
    InvalidType(Location),
    InvalidArity {
        name: &'static str,
        expected: usize,
        actual: usize,
        location: Location,
    },
    InvalidSize(Location),
    InvalidRel(Location),
    InvalidRelExpr(Location),
    UnsupportedList(Location),
}

#[derive(Clone, Default)]
pub struct ExprToType {
    errors: Vec<ExprToTypeError>
}

impl ExprToType {
    pub fn new() -> Self {
        Default::default()
    }

    fn push_err<T>(&mut self, res: Result<T, ExprToTypeError>) -> Result<T, ()> {
        res.map_err(|e| self.errors.push(e))
    }

    pub fn into_types(
        mut self,
        table: PathTable<Ranged<RawExpr>>
    ) -> Result<PathTable<Ranged<TypeExpr>>, Vec<ExprToTypeError>> {
        let PathTable { rows, names, defs } = table;
        let rows = rows.into_iter()
            .filter_map(|TypeRow { path, ty }|
                self.as_type(&ty).map(|ty| TypeRow { path, ty })
            )
            .collect();

        if self.errors.is_empty() {
            Ok(PathTable { rows, names, defs })
        }
        else {
            Err(self.errors)
        }
    }

    fn as_type(&mut self, expr: &Ranged<RawExpr>) -> Option<Ranged<TypeExpr>> {
        if let Some(simple) = as_simple_type(expr) {
            return Some(simple)
        }

        let ty =
            match &expr.data {
                RawExpr::Apply { head, .. } if is_rel_name(&head.data) => {
                    let rels = self.push_err(as_rel_list(expr)).ok()?;

                    TypeExpr::Liq(Box::new(TypeExpr::Top), rels)
                },
                RawExpr::Apply { head, body } if &head.data == "and" => {
                    let types = body.iter()
                        .filter_map(|e| self.as_type(e))
                        .collect();

                    TypeExpr::And(types)
                }
                RawExpr::Apply { head, body } if &head.data == "or" => {
                    let types = body.iter()
                        .filter_map(|e| self.as_type(e))
                        .collect();

                    TypeExpr::Or(types)
                }
                RawExpr::Apply { head, body } if &head.data == "arr" => {
                    if body.len() != 2 {
                        self.errors.push(
                            ExprToTypeError::InvalidArity {
                                name: "arr",
                                expected: 2,
                                actual: body.len(),
                                location: expr.range.unwrap()
                            }
                        );
                        return None
                    }

                    let (ty, num) = (&body[0], &body[1]);
                    let ty = Box::new(self.as_type(ty)?);
                    let num = self.push_err(as_size(num)).ok()?;

                    TypeExpr::Arr(ty, num)
                },
                RawExpr::Apply { head, body } if &head.data == "str" => {
                    if body.len() != 1 {
                        self.errors.push(
                            ExprToTypeError::InvalidArity {
                                name: "str",
                                expected: 1,
                                actual: body.len(),
                                location: expr.range.unwrap()
                            }
                        );
                        return None
                    }

                    let num = self.push_err(as_size(&body[0])).ok()?;

                    TypeExpr::Str(num)
                },
                RawExpr::Apply { head, body } if &head.data == "bytes" => {
                    if body.len() != 1 {
                        self.errors.push(
                            ExprToTypeError::InvalidArity {
                                name: "bytes",
                                expected: 1,
                                actual: body.len(),
                                location: expr.range.unwrap()
                            }
                        );
                        return None
                    }

                    let num = self.push_err(as_size(&body[0])).ok()?;

                    TypeExpr::Arr(
                        Box::new(Ranged::new(TypeExpr::U(1, Endianness::Be))),
                        num
                    )
                },
                RawExpr::Apply { head, body } if &head.data == "size" => {
                    if body.len() != 1 {
                        self.errors.push(
                            ExprToTypeError::InvalidArity {
                                name: "size",
                                expected: 1,
                                actual: body.len(),
                                location: expr.range.unwrap()
                            }
                        );
                        return None
                    }

                    let size = self.push_err(as_size(&body[0])).ok()?;
                    let rel_size = Ranged {
                        data: RelExpr::Size(size.data),
                        range: size.range
                    };

                    TypeExpr::Liq(
                        Box::new(TypeExpr::Top),
                        vec![Ranged::new(
                            Rel::Eq(
                                Ranged::new(RelExpr::Sizeof),
                                rel_size
                            )
                        )]
                    )
                },
                RawExpr::Apply { head, body } if &head.data == ":prod" => {
                    let mut paths = vec![];
                    for e in body {
                        match &e.data {
                            RawExpr::Path(path) =>
                                paths.push(path.clone()),
                            _ => unreachable!(),
                        }
                    }

                    TypeExpr::Prod(paths)
                }
                RawExpr::Path(path) =>
                    TypeExpr::Ref(path.clone()),
                RawExpr::List(_) => {
                    self.errors.push(
                        ExprToTypeError::UnsupportedList(expr.range.unwrap())
                    );

                    return None
                },
                _ => {
                    self.errors.push(
                        ExprToTypeError::InvalidType(expr.range.unwrap())
                    );

                    return None
                }
            };

        Some(Ranged {
            data: ty,
            range: expr.range
        })
    }
}

pub fn as_makam_ty(ty: &TypeExpr) -> String {
    use TypeExpr::*;

    match ty {
        Top => "top".to_owned(),
        Bottom => "bottom".to_owned(),
        Bool => "boolean".to_owned(),
        Int => "integer".to_owned(),
        U(s, Endianness::Be) =>
            format!("(u {} big_endian)", s),
        U(s, Endianness::Le) =>
            format!("(u {} little_endian)", s),
        S(s, Endianness::Be) =>
            format!("(s {} big_endian)", s),
        S(s, Endianness::Le) =>
            format!("(s {} little_endian)", s),
        Arr(ty, num) => {
            let ty = as_makam_ty(&ty.data);
            let num = as_makam_size(&num.data);

            format!("(arr {} {})", ty, num)
        },
        Str(len) => {
            let len = as_makam_size(&len.data);

            format!("(str {})", len)
        },
        And(list) => {
            let list = list.iter()
                .map(|ty| as_makam_ty(&ty.data))
                .collect::<Vec<_>>()
                .join(", ");
            format!("(meet [{}])", list)
        },
        Or(list) => {
            let list = list.iter()
                .map(|ty| as_makam_ty(&ty.data))
                .collect::<Vec<_>>()
                .join(", ");
            format!("(join [{}])", list)
        },
        Ref(path) => {
            let path = as_makam_path(&path);

            format!("(ref [{}])", path)
        },
        Prod(paths) => {
            let paths = paths.iter()
                .map(|path| as_makam_path(&path))
                .collect::<Vec<_>>()
                .join(", ");

            format!("(prod [{}])", paths)
        },
        Liq(ty, rels) => {
            let ty = as_makam_ty(ty);
            let rels = rels.iter()
                .map(|p| as_makam_rel(&p.data))
                .collect::<Vec<_>>()
                .join(", ");

            format!("(liq {} [{}])", ty, rels)
        },
        Val(value, ty) => {
            let value = as_makam_value(value);
            let ty = as_makam_ty(ty);

            format!("(val {} {})", value, ty)
        },
    }
}

pub fn as_makam_path(path: &RzPath) -> String {
    let modules = path.modules.iter()
        .map(|m| format!("\"{}\"", m))
        .collect::<Vec<_>>()
        .join(", ");
    let data = path.data.iter()
        .map(|m| format!("\"{}\"", m))
        .collect::<Vec<_>>()
        .join(", ");

    format!("(path [{}] [{}])", modules, data)
}

pub fn as_makam_size(size: &Size) -> String {
    let rels = size.rels.iter()
        .map(|r| as_makam_rel(&r.data))
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{}]", rels)
}

pub fn as_makam_value(value: &Value) -> String {
    match value {
        Value::Boolean(b) =>
            format!("(value_bool {})", b),
        Value::Integer(i) =>
            format!("(value_int {})", i)
    }
}

pub fn as_makam_rel_expr(expr: &RelExpr) -> String {
    match expr {
        RelExpr::Hole => "rel_hole".to_owned(),
        RelExpr::Sizeof => "rel_sizeof".to_owned(),
        RelExpr::Integer(i) =>
            format!("(rel_int {})", i),
        RelExpr::Ref(path) =>
            format!("(rel_ref {})", as_makam_path(&path)),
        RelExpr::Size(sz) =>
            format!("(ref_size {})", as_makam_size(&sz)),
    }
}

pub fn as_makam_rel(rel: &Rel) -> String {
    match rel {
        Rel::Lt(left, right) => {
            let left = as_makam_rel_expr(&left.data);
            let right = as_makam_rel_expr(&right.data);

            format!("(rel_lt {} {})", left, right)
        }
        Rel::Le(left, right) => {
            let left = as_makam_rel_expr(&left.data);
            let right = as_makam_rel_expr(&right.data);

            format!("(rel_le {} {})", left, right)
        }
        Rel::Eq(left, right) => {
            let left = as_makam_rel_expr(&left.data);
            let right = as_makam_rel_expr(&right.data);

            format!("(rel_eq {} {})", left, right)
        }
        Rel::Ge(left, right) => {
            let left = as_makam_rel_expr(&left.data);
            let right = as_makam_rel_expr(&right.data);

            format!("(rel_ge {} {})", left, right)
        }
        Rel::Gt(left, right) => {
            let left = as_makam_rel_expr(&left.data);
            let right = as_makam_rel_expr(&right.data);

            format!("(rel_gt {} {})", left, right)
        }
    }
}

// pub enum SimpifyError {
//     InnerJoin(Location, Option<Location>),
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Simplifier {
// }

// impl Simplifier {
//     pub fn simpify(&mut self, ty: &mut Ranged<TypeExpr>) -> Result<(), ()> {
//         todo!()
//     }

//     fn meet(left: &Ranged<TypeExpr>, right: &Ranged<TypeExpr>) -> Result<Ranged<TypeExpr>, SimpifyError> {
//         use TypeExpr::*;

//         match (left, right) {
//             (
//                 Ranged { data: Or(_), range: join_range },
//                 Ranged { range, .. }
//             )
//             | (
//                 Ranged { range, .. },
//                 Ranged { data: Or(_), range: join_range }
//             ) =>
//                 return Err(SimpifyError::InnerJoin(join_range.unwrap(), *range)),
//             _ => ()
//         }

//         let ty =
//             match (&left.data, &right.data) {
//                 (Top, e) | (e, Top) =>
//                     e.clone(),
//                 (Bottom, _) | (_, Bottom) =>
//                     Bottom,

//                 (Bool, Bool) =>
//                     Bool,

//                 (U(s, e), U(s2, e2))
//                 if s == s2 && e == e2 =>
//                     U(*s, *e),
//                 (S(s, e), S(s2, e2))
//                 if s == s2 && e == e2 =>
//                     S(*s, *e),
//                 (Int, U(s, e)) | (U(s, e), Int) =>
//                     U(*s, *e),
//                 (Int, S(s, e)) | (S(s, e), Int) =>
//                     S(*s, *e),
//                 (Int, Int) =>
//                     Int,

//                 // (Arr(ty1, num1), Arr(ty2, num2)) => {
//                 //     let ty = Simplifier::meet(&ty1, &ty2)?;
//                 //     let num = Simplifier::meet(&num1, &num2)?;

//                 //     Arr(Box::new(ty), Box::new(num))
//                 // },
//                 // (Str(num1), Str(num2)) => {
//                 //     let num = Simplifier::meet(&num1, &num2)?;

//                 //     Str(Box::new(num))
//                 // },

//                 // (Liq(ty1, pred1), Liq(ty2, pred2)) => {
//                 //     todo!()
//                 // },
//                 // (Liq(ty1, pred), Val(v, ty2)) | (Val(v, ty2), Liq(ty1, pred)) => {
//                 //     todo!()
//                 // },
//                 // (Liq(ty1, pred), ty2) => {
//                 //     todo!()
//                 // },

//                 // (Val(v1, ty1), Val(v2, ty2)) => {
//                 //     todo!()
//                 // },
//                 // (Val(v, ty1), ty2) | (ty2, Val(v, ty1)) => {
//                 //     todo!()
//                 // },
//                 // (And(tys1), And(tys2)) => {
//                 //     todo!()
//                 // },
//                 // (And(tys), ty) | (ty, And(tys)) => {
//                 //     todo!()
//                 // },
//                 _ => Bottom,
//             };

//         Ok(Ranged {
//             data: ty,
//             range: None,
//         })
//     }
// }
