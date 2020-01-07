use annotate_snippets::display_list::DisplayList;
use annotate_snippets::formatter::DisplayListFormatter;

use razbor::report::ToSnippet;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let loader = razbor::expr::ExprLoader::new();
    let (file_table, tox) = loader.load("../tox/tox-ksy/razbor/tox.mexpr");
    let mods = tox.unwrap();

    let converter = razbor::path::ExprConverter::new();
    let mut table = converter.convert(&mods).unwrap();

    let namer = razbor::path::NameResolver::new();
    let errors = namer.resolve(&mut table).unwrap_err();
    
    let mut sourcer = razbor::report::Sourcer::default();
    for err in &errors {
        let id = err.file_id();
        let path = &*file_table.files()[id];
        sourcer.load_file(id, path);
    }

    for err in &errors {
        let snippet = err.to_snippet(&sourcer);
        let dl = DisplayList::from(snippet);
        let dlf = DisplayListFormatter::new(true, false);
        println!("{}", dlf.format(&dl));
    }

    Ok(())
}
