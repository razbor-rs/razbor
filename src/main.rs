use razbor::{
    parse_file,
    import::include_imports,
    path::ExprConverter,
    path::NameResolver,
};

use std::path::Path;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let tox = parse_file("../tox/tox-ksy/razbor/tox.mexpr")?;

    let imports_included = include_imports(tox, Path::new("../tox/tox-ksy/razbor"))?;

    let mut table = ExprConverter::new().convert(&imports_included);

    let mut resolver = NameResolver::new();
    resolver.resolve_names(&mut table);

    for row in &table.rows {
        println!("{} -> {}", row.0, row.1);
    }

    println!();

    for name in resolver.not_found {
        println!("NOT FOUND: {}", name);
    }

    Ok(())
}
