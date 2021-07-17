#![feature(stdio_locked)]

use std::io::{self, BufRead, Write};

use liom_syntax::ast::AstNode;

fn main() -> io::Result<()> {
    let mut stdin = io::stdin_locked();
    let mut stdout = io::stdout_locked();

    let mut input = String::new();

    loop {
        write!(stdout, "-> ")?;
        stdout.flush()?;

        if stdin.read_line(&mut input)? == 0 {
            break;
        }

        let parse = liom_syntax::parse(&input);
        writeln!(stdout, "{:#?}", parse)?;

        let syntax = parse.syntax();

        for error in liom_syntax::validation::validate(&syntax) {
            writeln!(stdout, "{}", error)?;
        }

        let root = liom_syntax::ast::Root::cast(syntax).unwrap();

        for stmt in root.stmts() {
            match stmt {
                liom_syntax::ast::Stmt::VariableDef(def) => {
                    writeln!(stdout, "var {:?} = {:?}", def.name(), def.value())?;
                }
                liom_syntax::ast::Stmt::Expr(expr) => writeln!(stdout, "expr {:?}", expr)?,
            }
        }

        for stmt in liom_hir::database::Database::default().lower(&root) {
            match stmt {
                liom_hir::Stmt::VariableDef { name, value } => {
                    writeln!(stdout, "var {} = {:?}", name, value)?;
                }
                liom_hir::Stmt::Expr(expr) => writeln!(stdout, "expr {:?}", expr)?,
            }
        }

        input.clear();
    }

    Ok(())
}
