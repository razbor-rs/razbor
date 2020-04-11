use razbor::expr::ExprLoader;

const SPEC_DIR: &'static str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/spec");

#[test] #[should_panic] fn cyclic_import() {
    let loader = ExprLoader::new();
    let path = format!("{}/cyclic_import/a.mexpr", SPEC_DIR);
    
    let (_, modules) = loader.load(&path);
    modules.unwrap();
}
