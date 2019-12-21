use super::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RzPath {
    modules: Vec<String>,
    data: Vec<String>,
}

impl RzPath {
    pub fn new() -> Self {
        RzPath {
            modules: vec![],
            data: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.modules.is_empty() && self.data.is_empty()
    }

    pub fn pop(&mut self) {
        if self.data.is_empty() {
            drop(self.modules.pop())
        }
        else {
            drop(self.data.pop())
        }
    }

    pub fn push_module(&mut self, module: String) {
        assert!(self.data.is_empty());

        self.modules.push(module);
    }

    pub fn push_data(&mut self, data: String) {
        self.data.push(data)
    }

    pub fn is_module_path(&self) -> bool {
        self.data.is_empty()
    }

    pub fn into_expr(self) -> Mexpr {
        let modules = self.modules.into_iter()
            .map(Mexpr::Name)
            .collect();
        let data = self.data.into_iter()
            .map(Mexpr::Name)
            .collect();

        Mexpr::Apply {
            name: ":ref".to_owned(),
            body: vec![
                Mexpr::List(modules),
                Mexpr::List(data),
            ],
        }
    }

    // Ad-hoc sorting to allow binary search
    fn cmp(&self, other: &RzPath) -> std::cmp::Ordering {
        use std::cmp::Ordering as O;

        fn choose(a: O, b: O) -> O {
            if a == O::Equal { b }
            else { a }
        }

        let cmp = self.modules.iter()
            .zip(&other.modules)
            .map(|(a, b)| a.cmp(b))
            .fold(O::Equal, choose);
        let cmp = choose(
            cmp,
            self.modules.len().cmp(&other.modules.len())
        );
        let cmp = choose(
            cmp,
            self.data.iter()
                .zip(&other.data)
                .map(|(a, b)| a.cmp(b))
                .fold(O::Equal, choose)
        );

        choose(
            cmp,
            self.data.len().cmp(&other.data.len())
        )
    }

    fn cmp_prefix(&self, prefix: &RzPath) -> std::cmp::Ordering {
        use std::cmp::Ordering as O;

        fn choose(a: O, b: O) -> O {
            if a == O::Equal { b }
            else { a }
        }

        let cmp = self.modules.iter()
            .zip(&prefix.modules)
            .map(|(a, b)| a.cmp(b))
            .fold(O::Equal, choose);

        match (self.modules.len().cmp(&prefix.modules.len()), self.data.len().cmp(&prefix.data.len())) {
            (O::Greater, _) => cmp,
            (O::Less, _) => O::Less,
            (O::Equal, O::Less) =>
                choose(cmp, O::Less),
            _ =>
                choose(
                    cmp,
                    self.data.iter()
                        .zip(&prefix.data)
                        .map(|(a, b)| a.cmp(b))
                        .fold(O::Equal, choose)
                ),
        }
    }
}

impl std::fmt::Display for RzPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.modules.join("."))?;
        write!(f, ".{}", self.data.join(" "))
    }
}

fn is_reserved_name(name: &str) -> bool {
    match name {
        "u1" | "u2" | "u4" | "u8"
        | "_" =>
            true,
        _ => false
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PathTable {
    pub rows: Vec<(RzPath, Mexpr)>,
    pub names: Vec<(RzPath, RzPath)>,
    pub defs: Vec<RzPath>,
}

impl PathTable {
    fn new() -> Self {
        PathTable {
            rows: vec![],
            names: vec![],
            defs: vec![],
        }
    }

    fn add_def(&mut self, def: RzPath) {
        self.defs.push(def)
    }

    fn insert(&mut self, path: RzPath, ty: Mexpr) {
        self.rows.push((path, ty));
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExprConverter {
    current: RzPath,
    current_name: RzPath,
    table: PathTable,
}

impl ExprConverter {
    pub fn new() -> ExprConverter {
        ExprConverter {
            current: RzPath::new(),
            current_name: RzPath::new(),
            table: PathTable::new(),
        }
    }

    fn push_ty(&mut self, name: String, number: Option<usize>) {
        self.current_name.push_data(name.clone());

        if let Some(n) = number {
            self.current.push_data(n.to_string());
        }
        else {
            self.current.push_data(name)
        }

        self.table.names.push((self.current.clone(), self.current_name.clone()))
    }

    fn enter_data(&mut self, data: &Mexpr, is_in_list: bool) {
        let path = self.current.clone();

        self.table.insert(path, data.clone());
        if is_in_list {
            self.table.names.push((self.current.clone(), self.current_name.clone()))
        }
    }

    fn enter_ty(&mut self, name: String, body: &Mexpr, number: Option<usize>) {
        self.push_ty(name, number);

        match body {
            Mexpr::List(body) =>
                self.enter_list(body),
            e =>
                self.enter_data(e, false),
        }

        self.current.pop();
        self.current_name.pop();
    }

    fn enter_list(&mut self, body: &[Mexpr]) {
        for (i, e) in body.iter().enumerate() {
            let num =
                if !self.current.is_module_path() { Some(i) }
                else { None };

            match e {
                Mexpr::Apply { name, body } if name == "ty" || name == "def" => {
                    let (head, tail) = get_name_value(body).unwrap();

                    if name == "def" {
                        let mut path = self.current.clone();
                        path.push_data(head.clone());
                        self.table.add_def(path);
                    }

                    self.enter_ty(head.clone(), tail, num);
                },
                Mexpr::Apply { name, body } if name == ":module" => {
                    let (head, tail) = get_name_value(body).unwrap();
                    let tail =
                        match tail {
                            Mexpr::List(body) => body,
                            _ => panic!()
                        };

                    self.current.push_module(head.clone());
                    self.current_name.push_module(head.clone());

                    self.enter_list(tail);

                    self.current.pop();
                    self.current_name.pop();
                },
                Mexpr::Apply { name, .. } if name == "ann" => (),
                Mexpr::List(body) =>
                    self.enter_list(body),
                e => {
                    self.current.push_data(num.unwrap().to_string());
                    self.current_name.push_data(num.unwrap().to_string());
                    self.enter_data(e, true);
                    self.current.pop();
                    self.current_name.pop();
                },
            }
        }
    }

    pub fn convert(mut self, expr: &Mexpr) -> PathTable {
        match expr {
            Mexpr::List(body) =>
                self.enter_list(body),
            _ => panic!(),
        }

        self.table.rows.sort_by(|a, b| a.0.cmp(&b.0));
        self.table.names.sort_by(|a, b| a.0.cmp(&b.0));
        self.table
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NameResolver {
    pub not_found: Vec<String>,
}

impl NameResolver {
    pub fn new() -> Self {
        NameResolver {
            not_found: vec![]
        }
    }

    fn find_path<'a>(&self, base: &RzPath, prefix: &[&str], name_map: &'a [(RzPath, RzPath)]) -> Option<&'a RzPath> {
        let mut path = base.clone();
        let is_module = base.is_module_path();
        let mut index = 0;

        for i in 0..prefix.len() {
            if is_module {
                path.push_module(prefix[i].to_owned())
            }
            else {
                path.push_data(prefix[i].to_owned())
            }

            match name_map.binary_search_by(|(_, n)| n.cmp_prefix(&path)) {
                Ok(i) =>
                    index = i,
                _ if is_module => {
                    path.pop();
                    prefix[i..].iter()
                        .for_each(|&p| path.push_data(p.to_owned()));

                    return name_map.binary_search_by(|(_, n)| n.cmp(&path))
                        .map(|i| &name_map[i].0)
                        .ok()
                },
                _ =>
                    return None,
            }
        }

        Some(&name_map[index].0)
    }

    fn resolve_in_expr(&mut self, path: &RzPath, value: &mut Mexpr, name_map: &[(RzPath, RzPath)]) {
        match value {
            Mexpr::Apply { body, .. } | Mexpr::List(body) => {
                body.iter_mut()
                    .for_each(|e| self.resolve_in_expr(path, e, name_map))
            }
            Mexpr::Name(name) if !is_reserved_name(name) => {
                let local: Vec<_> = name.split('.').collect();
                let mut path = path.clone();
                path.pop();

                loop {
                    if let Some(path) = self.find_path(&path, local.as_ref(), name_map) {
                        let path = path.clone().into_expr();
                        *value = path;
                        return
                    };

                    if path.is_empty() { break }
                    path.pop()
                }

                self.not_found.push(name.clone())
            }
            _ => (),
        }
    }

    pub fn resolve_names(&mut self, table: &mut PathTable) {
        let mut name_map = table.names.clone();
        name_map.sort_by(|a, b| a.1.cmp(&b.1));

        for i in 0..table.rows.len() {
            let path_ix = table.names
                .binary_search_by(|n| n.0.cmp(&table.rows[i].0))
                .unwrap();
            let path = &table.names[path_ix].1;
            let mut value = table.rows[i].1.clone();

            self.resolve_in_expr(path, &mut value, &name_map);
            table.rows[i].1 = value
        }
    }
}
