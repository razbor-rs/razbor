use std::path::Path;

use super::*;

fn expand_import(expr: Mexpr, current_path: &Path) -> Result<Vec<Mexpr>, Error> {
    let (_, body) = destruct_apply(expr).unwrap();
    let mut acc = vec![];

    for e in body {
        let name = destruct_name(e).ok_or(UnknownError)?;
        let path = current_path.join(format!("{}.mexpr", name));
        let include = parse_file(&path)?;
        let expr = Mexpr::Apply {
            name: ":module".to_owned(),
            body: vec![
                Mexpr::Name(name),
                include_imports(include, current_path)?
            ]
        };

        acc.push(expr)
    }

    Ok(acc)
}

pub fn include_imports(expr: Mexpr, current_path: &Path) -> Result<Mexpr, Error> {
    match expr {
        Mexpr::Apply { name, body } => {
            let mut new = vec![];
            for e in body.into_iter() {
                if is_m_name(&e, "import") {
                    new.extend(expand_import(e, current_path)?)
                }
                else {
                    new.push(e)
                }
            }

            Ok(Mexpr::Apply { name, body: new })
        },
        Mexpr::List(body) => {
            let mut new = vec![];
            for e in body.into_iter() {
                if is_m_name(&e, "import") {
                    new.extend(expand_import(e, current_path)?)
                }
                else {
                    new.push(e)
                }
            }

            Ok(Mexpr::List(new))
        },
        e => Ok(e)
    }
}
