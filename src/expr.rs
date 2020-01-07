use smol_str::SmolStr;

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub from: usize,
    pub to: usize,
}

impl Span {
    pub fn start(self) -> Self {
        Span {
            from: self.from,
            to: self.from,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub file_id: usize,
    pub span: Span,
}

impl Location {
    pub fn start(self) -> Self {
        Location {
            span: self.span.start(),
            ..self
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ranged<T> {
    pub data: T,
    pub range: Option<Location>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RawExpr {
    Apply {
        head: Ranged<SmolStr>,
        body: Vec<Ranged<RawExpr>>,
    },
    List(Vec<Ranged<RawExpr>>),
    Name(SmolStr),
    Decimal(SmolStr),
    Hexdecimal(SmolStr),
    String(SmolStr),

    Ty { name: Ranged<SmolStr>, data: Box<Ranged<RawExpr>> },
    Def { name: Ranged<SmolStr>, data: Box<Ranged<RawExpr>> },
    Path(crate::path::RzPath),
}

type RawExprList = Ranged<Vec<Ranged<RawExpr>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct RawModule {
    pub file_id: usize,
    pub path: Vec<SmolStr>,
    pub data: Ranged<Vec<Ranged<RawExpr>>>,
}

#[derive(Debug)]
struct LoadError {
    errors: Vec<LoadErrorKind>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileTable {
    table: Vec<PathBuf>,
}

impl FileTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, file: PathBuf) -> usize {
        assert!(file.is_file());

        if let Some(i) = self.table.iter().position(|f| f == &file) {
            i
        }
        else {
            self.table.push(file);
            self.table.len() - 1
        }
    }

    pub fn position(&self, path: &Path) -> Option<usize> {
        self.table.iter().position(|p| p == path)
    }

    pub fn files(&self) -> &[PathBuf] {
        &self.table
    }
}

#[derive(Debug)]
pub enum LoadErrorKind {
    Io(Option<Location>, std::io::Error),
    Pest(pest::error::Error<crate::Rule>, usize),
    NotFound(Location),
    Cycle(Vec<Location>),
    InvalidName(Location),
    InvalidTy(Location),
    ImportInside(Location),
}

fn pair_location(pair: &crate::Pair<crate::Rule>, file_id: usize) -> Location {
    Location {
        file_id,
        span: Span {
            from: pair.as_span().start(),
            to: pair.as_span().end(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct QueuedModule {
    name: (SmolStr, Option<Location>),
    file: usize,
    imports: Vec<(SmolStr, Span)>,
    content: Ranged<Vec<Ranged<RawExpr>>>,
}

fn get_module_path(
    base: &Path,
    import: &str,
    parrent: Option<&str>
) -> Option<PathBuf> {
    let path = base.join(format!("{}.mexpr", import));

    if path.is_file() { return Some(path) }

    let alt_path = parrent.map(|m| base.join(format!("{}/{}.mexpr", m, import)));
    let alt_is_file = alt_path.as_ref()
        .map(|p| p.is_file())
        .unwrap_or(false);

    if alt_is_file { alt_path }
    else { None }
}

#[derive(Debug, Default)]
pub struct ExprLoader {
    queue: Vec<QueuedModule>,
    complete: Vec<RawModule>,
    errors: Vec<LoadErrorKind>,
    files: FileTable,
}

impl ExprLoader {
    pub fn new() -> Self {
        Self::default()
    }

    fn push_err<T, E>(&mut self, res: Result<T, E>) -> Result<T, ()>
    where E: Into<LoadErrorKind>
    {
        res.map_err(|e| self.errors.push(e.into()))
    }

    pub fn load<P: AsRef<Path>>(mut self, path: P)
        -> (FileTable, Result<Vec<RawModule>, Vec<LoadErrorKind>>)
    {
        use crate::{MexprParser as M, Rule as R};
        use pest::Parser;

        if let Err(e) = self.load_root(path.as_ref()) {
            return (self.files, Err(vec![e]))
        };

        while let Some(module) = self.queue.last_mut() {
            let imp = module.imports.pop();

            if let Some(imp) = imp {
                let loc = Location {
                    span: imp.1,
                    file_id: module.file,
                };
                let name = module.name.0.clone();

                let source = self.load_import((imp.0.clone(), loc), &name);
                let source = self.push_err(source);
                if source.is_err() { continue }
                let (file_id, source) = source.unwrap();

                let pair = M::parse(R::mexpr, &source)
                    .map(|mut p| p.next().unwrap())
                    .map_err(|e| LoadErrorKind::Pest(e, file_id));
                let pair = self.push_err(pair);
                if pair.is_err() { continue }

                let (expr, imps) = self.parse_root(pair.unwrap(), file_id);

                self.queue.push(
                    QueuedModule {
                        name: (imp.0, Some(loc)),
                        file: file_id,
                        imports: imps,
                        content: expr,
                    }
                );
            }
            else {
                self.complete_module()
            }
        }

        let res =
            if self.errors.is_empty() { Ok(self.complete) }
            else { Err(self.errors) };

        (self.files, res)
    }

    fn load_root(&mut self, path: &Path) -> Result<(), LoadErrorKind> {
        use crate::{MexprParser as M, Rule as R};
        use pest::Parser;

        let source = std::fs::read_to_string(path)
            .map_err(|e| LoadErrorKind::Io(None, e))?;

        let file_id = self.files.append(path.to_owned());

        let pair = M::parse(R::mexpr, &source)
            .map(|mut p| p.next().unwrap())
            .map_err(|e| LoadErrorKind::Pest(e, file_id))?;
        let (expr, imps) = self.parse_root(pair, file_id);

        let name = path.file_stem().unwrap().to_string_lossy();
        let name = SmolStr::new(name.as_ref());

        self.queue.push(
            QueuedModule {
                name: (name, None),
                file: file_id,
                imports: imps,
                content: expr,
            }
        );

        Ok(())
    }

    fn complete_module(&mut self) {
        let path = self.queue.iter()
            .map(|m| m.name.0.clone())
            .collect();
        let module = self.queue.pop().unwrap();
        let raw = RawModule {
            file_id: module.file,
            path,
            data: module.content
        };

        self.complete.push(raw)
    }

    fn load_import(
        &mut self,
        import: (SmolStr, Location),
        parrent: &str
    ) -> Result<(usize, String), LoadErrorKind> {
        let base = &self.files.table[import.1.file_id];
        let path = get_module_path(base.parent().unwrap(), &import.0, Some(parrent))
            .ok_or(LoadErrorKind::NotFound(import.1))?;

        let file_id = self.files.append(path.to_owned());
        let pos = self.queue.iter().position(|m| m.file == file_id);

        if let Some(pos) = pos {
            let locs = self.queue[pos..].iter()
                .filter_map(|m| m.name.1)
                .chain(Some(import.1))
                .collect();

            return Err(LoadErrorKind::Cycle(locs))
        }

        std::fs::read_to_string(path)
            .map(|s| (file_id, s))
            .map_err(|e| LoadErrorKind::Io(Some(import.1), e))
    }

    fn parse_root(&mut self, pair: crate::Pair<crate::Rule>, file_id: usize)
        -> (RawExprList, Vec<(SmolStr, Span)>)
    {
        use crate::Rule as R;

        assert_eq!(pair.as_rule(), R::mexpr);

        let loc = pair_location(&pair, file_id);

        let iter = pair.into_inner().filter(|r| r.as_rule() != R::EOI);
        let mut content = vec![];
        let mut imports = vec![];

        for p in iter {
            let imp = self.destruct_import(p.clone(), file_id);
            if imp.is_err() { continue }

            if let Some(imp) = imp.unwrap() {
                imports.extend(imp)
            }
            else {
                let expr = self.parse_pair(p, file_id);
                if expr.is_err() { continue }

                content.push(expr.unwrap());
            }
        }

        let ranged = Ranged {
            data: content,
            range: Some(loc)
        };

        (ranged, imports)
    }

    fn parse_pair(&mut self, pair: crate::Pair<crate::Rule>, file_id: usize)
        -> Result<Ranged<RawExpr>, LoadErrorKind>
    {
        use crate::Rule as R;

        let loc = pair_location(&pair, file_id);
        match pair.as_rule() {
            R::m => {
                let mut inner = pair.into_inner();
                let head = inner.next().unwrap();
                let head_str: SmolStr = head.as_str().into();
                match &*head_str {
                    "import" =>
                        Err(LoadErrorKind::ImportInside(loc)),
                    "ty" | "def" => {
                        let name = inner.next().ok_or(LoadErrorKind::InvalidTy(loc))?;
                        if name.as_rule() != R::name {
                            let loc = pair_location(&name, file_id);
                            return Err(LoadErrorKind::InvalidName(loc));
                        }

                        let data = inner.next().ok_or(LoadErrorKind::InvalidTy(loc))?;
                        let data = self.parse_pair(data, file_id)?;

                        if inner.next().is_some() {
                            return Err(LoadErrorKind::InvalidTy(loc))
                        }

                        let name = Ranged {
                            data: name.as_str().into(),
                            range: Some(pair_location(&name, file_id))
                        };
                        let expr =
                            if &head_str == "ty" {
                                RawExpr::Ty {
                                    name,
                                    data: Box::new(data)
                                }
                            }
                            else {
                                RawExpr::Def {
                                    name,
                                    data: Box::new(data)
                                }
                            };

                        Ok(Ranged {
                            data: expr,
                            range: Some(loc)
                        })
                    },
                    _ => {
                        let body: Result<Vec<_>, _> = inner
                            .map(|p| self.parse_pair(p, file_id)).collect();

                        let expr = RawExpr::Apply {
                            head: Ranged {
                                data: head_str,
                                range: Some(pair_location(&head, file_id))
                            },
                            body: body?
                        };

                        Ok(Ranged {
                            data: expr,
                            range: Some(loc)
                        })
                    }
                }
            },
            R::list => {
                let expr = RawExpr::List(
                    pair.into_inner()
                        .map(|p| self.parse_pair(p, file_id))
                        .collect::<Result<_, _>>()?
                );

                Ok(Ranged {
                    data: expr,
                    range: Some(loc)
                })
            },
            R::name | R::decimal | R::hexdecimal | R::string => {
                let data = pair.as_str().into();
                let expr = match pair.as_rule() {
                    R::name => RawExpr::Name(data),
                    R::decimal => RawExpr::Decimal(data),
                    R::hexdecimal => RawExpr::Hexdecimal(data),
                    R::string => RawExpr::String(data),
                    _ => unreachable!()
                };

                Ok(Ranged {
                    data: expr,
                    range: Some(loc)
                })
            },
            _ => unreachable!()
        }
    }

    fn destruct_import(&mut self, pair: crate::Pair<crate::Rule>, file_id: usize)
        -> Result<Option<Vec<(SmolStr, Span)>>, ()>
    {
        if pair.as_rule() != crate::Rule::m { return Ok(None) }

        let mut inner = pair.into_inner();
        let name = inner.next().map(|p| p.as_str()).unwrap();

        if name != "import" { return Ok(None) }

        let mut mods = vec![];
        for p in inner {
            let span = Span {
                from: p.as_span().start(),
                to: p.as_span().end(),
            };
            if p.as_rule() != crate::Rule::name {
                let loc = pair_location(&p, file_id);

                self.push_err(Err(LoadErrorKind::InvalidName(loc)))?
            }

            mods.push(
                (SmolStr::new(p.as_str()), span)
            )
        }

        Ok(Some(mods))
    }
}
