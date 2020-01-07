use razbor::expr::ExprLoader;
use razbor::path::{ExprConverter, NameResolver};

const SPEC_DIR: &'static str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/spec");

#[test] fn common_import() {
    let loader = ExprLoader::new();
    let path = format!("{}/common_import/root.mexpr", SPEC_DIR);

    let (_, modules) = loader.load(&path);
    let modules = modules.unwrap();

    for file_id in 0..4 {
        assert!(modules.iter().any(|m| m.file_id == file_id))
    }
}

#[test] fn subdir_import() {
    let loader = ExprLoader::new();
    let path = format!("{}/subdir_import/root.mexpr", SPEC_DIR);

    let (_, modules) = loader.load(&path);
    modules.unwrap();
}

#[test] fn resolve_inside() {
    let loader = ExprLoader::new();
    let path = format!("{}/resolve_inside/root.mexpr", SPEC_DIR);

    let (_, modules) = loader.load(&path);
    let modules = modules.unwrap();

    let converter = ExprConverter::new();
    let mut table = converter.convert(&modules).unwrap();

    let resolver = NameResolver::new();
    resolver.resolve(&mut table).unwrap();
}

#[test] fn resolve_in_modules() {
    let loader = ExprLoader::new();
    let path = format!("{}/resolve_in_modules/root.mexpr", SPEC_DIR);

    let (_, modules) = loader.load(&path);
    let modules = modules.unwrap();

    let converter = ExprConverter::new();
    let mut table = converter.convert(&modules).unwrap();

    let resolver = NameResolver::new();
    resolver.resolve(&mut table).unwrap();
}
