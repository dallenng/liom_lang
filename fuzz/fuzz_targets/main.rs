#![no_main]

use libfuzzer_sys::fuzz_target;
use liom_syntax::ast::AstNode;

fuzz_target!(|data: &str| {
    run(data);
});

fn run(data: &str) {
    let parse = liom_syntax::parse(data);
    let syntax = parse.syntax();
    let _validation_errors = liom_syntax::validation::validate(&syntax);
    let root = liom_syntax::ast::Root::cast(syntax).unwrap();
    let mut database = liom_hir::database::Database::default();
    let _stmts = database.lower(&root).collect::<Vec<_>>();
}
