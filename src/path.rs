use crate::expr::Location;
use crate::expr::RawModule;
use crate::expr::RawExpr;
use smol_str::SmolStr;

// use super::*;
use super::expr::Ranged;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum PathSegment {
    Pos(usize),
    Name(SmolStr),
}

impl std::fmt::Display for PathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathSegment::Pos(n) => write!(f, "{}", n),
            PathSegment::Name(name) => write!(f, "{}", name)
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct RzPath {
    pub modules: Vec<SmolStr>,
    pub data: Vec<PathSegment>,
}

impl RzPath {
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty() && self.data.is_empty()
    }

    pub fn is_module_path(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_data_path(&self) -> bool {
        !self.is_module_path()
    }

    pub fn pop(&mut self) {
        if self.data.is_empty() {
            drop(self.modules.pop())
        }
        else {
            drop(self.data.pop())
        }
    }

    pub fn prefix_cmp(&self, prefix: &RzPath) -> Ordering {
        let mod_cmp = zip_cmp(&self.modules, &prefix.modules);

        if self.is_module_path() && prefix.is_module_path() {
            mod_cmp
        }
        else {
            mod_cmp
                .then(self.modules.len().cmp(&prefix.modules.len()))
                .then(zip_cmp(&self.data, &prefix.data))
        }
    }

    pub fn clear(&mut self) {
        self.modules.clear();
        self.data.clear()
    }
}

// impl std::fmt::Display for RzPath {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(f, "{}", self.modules.join("."))?;
//         write!(f, ".{}", self.data.join(" "))
//     }
// }

/// Find the lexicographical order of two arrays
fn zip_cmp<T: Ord>(a: &[T], b: &[T]) -> Ordering {
    a.iter().zip(b)
        .map(|(a, b)| a.cmp(b))
        .fold(Ordering::Equal, Ordering::then)
}


fn is_reserved_name(name: &str) -> bool {
    let names = [
        "_", "never",
        "bool", "int",
        "u1", "u2", "u4", "u8",
        "s1", "s2", "s4", "s8",
    ];

    names.contains(&name)
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeRow<T> {
    pub path: RzPath,
    pub ty: T
}

#[derive(Debug, Clone, PartialEq, )]
pub struct NameRow {
    pub path: RzPath,
    pub name: Ranged<RzPath>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathTable<T> {
    pub rows: Vec<TypeRow<T>>,
    pub names: Vec<NameRow>,
    pub defs: Vec<RzPath>,
}

impl<T> Default for PathTable<T> {
    fn default() -> Self {
        PathTable {
            rows: vec![],
            names: vec![],
            defs: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprConvertError {
    UnboundValue(Location),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExprConverter {
    current_path: RzPath,
    current_name: RzPath,
    table: PathTable<Ranged<RawExpr>>,
    errors: Vec<ExprConvertError>,
}

impl ExprConverter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn convert(mut self, mods: &[RawModule])
        -> Result<PathTable<Ranged<RawExpr>>, Vec<ExprConvertError>>
    {
        for m in mods {
            self.visit_module(m)
        }

        if self.errors.is_empty() {
            self.table.rows.sort_by(|a, b| a.path.cmp(&b.path));
            self.table.names.sort_by(|a, b| a.path.cmp(&b.path));

            Ok(self.table)
        }
        else {
            Err(self.errors)
        }
    }

    fn visit_module(&mut self, module: &RawModule) {
        self.current_path.clear();
        self.current_path.modules.extend(module.path.iter().cloned());
        self.current_name.clear();
        self.current_name.modules.extend(module.path.iter().cloned());

        for expr in &module.data.data {
            match &expr.data {
                RawExpr::Def { name, data } =>
                    self.visit_def(name, &data),
                RawExpr::Ty{ name, data } => {
                    self.visit_ty(name, &data, None)
                },
                RawExpr::Apply { head, .. }
                if head.data == "ann" || head.data == "meta" =>
                    continue,
                _ => {
                    let err = ExprConvertError::UnboundValue(expr.range.unwrap());
                    self.errors.push(err)
                }
            }
        }
    }

    fn visit_def(&mut self, name: &Ranged<SmolStr>, body: &Ranged<RawExpr>) {
        let mut path = self.current_path.clone();
        path.data.push(PathSegment::Name(name.data.clone()));
        self.table.defs.push(path);

        self.visit_ty(name, body, None)
    }

    fn visit_ty(
        &mut self,
        name: &Ranged<SmolStr>,
        body: &Ranged<RawExpr>,
        number: Option<usize>
    ) {
        self.push_ty(name, number);

        match &body.data {
            RawExpr::List(data) => self.visit_list(&data),
            RawExpr::Ty { .. } | RawExpr::Def { .. } => todo!(),
            _ => self.visit_data(body, None)
        }

        self.pop()
    }

    fn push_ty(&mut self, name: &Ranged<SmolStr>, number: Option<usize>) {
        if let Some(num) = number {
            self.current_path.data.push(PathSegment::Pos(num));
        }
        else {
            self.current_path.data.push(PathSegment::Name(name.data.clone()))
        }

        self.current_name.data.push(PathSegment::Name(name.data.clone()));

        let name_row = NameRow {
            path: self.current_path.clone(),
            name: Ranged {
                data: self.current_name.clone(),
                range: name.range,
            }
        };
        self.table.names.push(name_row);
    }

    fn push_data(&mut self, number: usize) {
        self.current_path.data.push(PathSegment::Pos(number));
        self.current_name.data.push(PathSegment::Pos(number))
    }

    fn pop(&mut self) {
        drop(self.current_path.data.pop());
        drop(self.current_name.data.pop())
    }

    fn push_prod(&mut self, number: usize) {
    }

    fn visit_list(&mut self, list: &[Ranged<RawExpr>]) {
        let mut num = 0;
        for e in list {
            match &e.data {
                RawExpr::Def { name, data } => {
                    self.visit_def(name, &data);
                    continue;
                },
                RawExpr::Ty { name, data } =>
                    self.visit_ty(name, &data, Some(num)),
                RawExpr::Apply { head, .. } if head.data == "ann" =>
                    continue,
                _ =>
                    self.visit_data(&e, Some(num))
            }

            num += 1
        }

        self.push_prod(num)
    }

    fn visit_data(&mut self, data: &Ranged<RawExpr>, num: Option<usize>) {
        if let Some(num) = num {
            self.push_data(num);

            let ty_row = TypeRow {
                path: self.current_path.clone(),
                ty: data.clone()
            };
            let name_row = NameRow {
                path: self.current_path.clone(),
                name: Ranged {
                    data: self.current_name.clone(),
                    range: data.range.map(Location::start)
                },
            };
            self.table.rows.push(ty_row);
            self.table.names.push(name_row);

            self.pop()
        }
        else {
            let ty_row = TypeRow {
                path: self.current_path.clone(),
                ty: data.clone()
            };
            self.table.rows.push(ty_row);
        }
    }
}

fn parse_name(name: &str) -> Option<Vec<&str>> {
    let vec: Vec<_> = name.split('.').collect();
    if !vec.contains(&"") {
        Some(vec)
    }
    else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, )]
pub enum NameResolveError {
    NotFound(Location),
    InvalidName(Location),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NameResolver {
    errors: Vec<NameResolveError>
}

impl NameResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve(
        mut self,
        table: &mut PathTable<Ranged<RawExpr>>
    ) -> Result<(), Vec<NameResolveError>> {
        let mut name_map = table.names.clone();
        name_map.sort_by(|a, b| a.name.data.cmp(&b.name.data));

        for i in 0..table.rows.len() {
            let path_ix = table.names
                .binary_search_by(|n| n.path.cmp(&table.rows[i].path))
                .unwrap();
            let path = &table.names[path_ix].name;
            let ty = &mut table.rows[i].ty;

            self.resolve_in_expr(path, ty, &name_map);
        }

        if self.errors.is_empty() {
            Ok(())
        }
        else {
            Err(self.errors)
        }
    }

    fn resolve_in_expr(
        &mut self,
        path: &Ranged<RzPath>,
        ty: &mut Ranged<RawExpr>,
        name_map: &[NameRow]
    ) {
        match &mut ty.data {
            RawExpr::Apply { body, ..}
            | RawExpr::List(body) =>
                body.iter_mut()
                    .for_each(|e| self.resolve_in_expr(path, e, name_map)),
            RawExpr::Name(name) if !is_reserved_name(name) => {
                if let Some(suffix) = parse_name(name) {
                    if let Some(path) = self.resolve_name(path, &suffix, name_map) {
                        ty.data = RawExpr::Path(path)
                    }
                    else {
                        let err = NameResolveError::NotFound(ty.range.unwrap());
                        self.errors.push(err)
                    }
                }
                else {
                    let err = NameResolveError::InvalidName(ty.range.unwrap());
                    self.errors.push(err);
                }
            },
            _ => ()
        }
    }

    fn resolve_name(
        &self,
        path: &Ranged<RzPath>,
        suffix: &[&str],
        name_map: &[NameRow]
    ) -> Option<RzPath> {
        let mut prefix = path.data.clone();
        prefix.data.pop();

        loop {
            if let Some(path) = self.find_path(&prefix, &suffix, name_map) {
                return Some(path.clone())
            }
            if prefix.is_empty() { break }
            prefix.pop()
        }

        None
    }

    fn find_path<'a>(
        &self,
        prefix: &RzPath,
        suffix: &[&str],
        name_map: &'a [NameRow]
    ) -> Option<&'a RzPath> {
        let mut path = prefix.clone();

        let mut index = 0;
        for &s in suffix.iter() {
            if path.is_data_path() { break }
            path.modules.push(s.into());

            let ix = name_map.binary_search_by(|n| n.name.data.prefix_cmp(&path));
            if ix.is_err() {
                path.pop();
                break
            }

            index += 1;
        }

        suffix[index..].iter()
            .for_each(|&p| path.data.push(PathSegment::Name(p.into())));

        name_map.binary_search_by(|n| n.name.data.cmp(&path))
            .map(|i| &name_map[i].name.data)
            .ok()
    }
}


#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    fn rzpath() -> impl Strategy<Value=RzPath> {
        use proptest::collection::vec;
        let s1 = proptest::string::string_regex("[a-zA-Z0-9_.]{1, 16}")
            .unwrap()
            .prop_map(SmolStr::from);
        let s2 = proptest::string::string_regex("[a-zA-Z0-9_.]{1, 16}")
            .unwrap()
            .prop_map(|s| PathSegment::Name(s.into()));

        (vec(s1, 0..32), vec(s2, 0..32))
            .prop_map(|(modules, data)| RzPath { modules, data })
    }

    proptest! {
        #[test]
        fn cmp_prefix_is_transitive(a in rzpath(), b in rzpath(), c in rzpath()) {
            if a.prefix_cmp(&b) == b.prefix_cmp(&c) {
                assert_eq!(a.prefix_cmp(&c), a.prefix_cmp(&b))
            }
        }

        #[test]
        fn cmp_prefix_is_reflexive(p in rzpath()) {
            assert_eq!(p.prefix_cmp(&p), std::cmp::Ordering::Equal);
        }
    }
}
