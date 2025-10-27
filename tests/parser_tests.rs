use otterlang::ast::nodes::{Expr, Literal, Statement};
use otterlang::lexer::tokenize;
use otterlang::parser::parse;

#[test]
fn parse_print_function() {
    let source = "fn main:\n    print(\"Hello\")\n";
    let tokens = tokenize(source).expect("tokenization should succeed");
    let program = parse(&tokens).expect("parsing should succeed");

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Function(function) => {
            assert_eq!(function.name, "main");
            assert_eq!(function.body.statements.len(), 1);
            match &function.body.statements[0] {
                Statement::Expr(expr) => match expr {
                    Expr::Call { func, args } => {
                        match &**func {
                            Expr::Identifier(name) => assert_eq!(name, "print"),
                            other => panic!("expected print identifier, got {:?}", other),
                        }
                        assert_eq!(args.len(), 1);
                        match &args[0] {
                            Expr::Literal(Literal::String(value)) => assert_eq!(value, "Hello"),
                            other => panic!("expected string literal, got {:?}", other),
                        }
                    }
                    other => panic!("expected function call, got {:?}", other),
                },
                stmt => panic!("expected expression statement, got {:?}", stmt),
            }
        }
        stmt => panic!("expected function statement, got {:?}", stmt),
    }
}

#[test]
fn parse_function_call_expression() {
    let source = "fn main:\n    x = add(2, 3)\n";
    let tokens = tokenize(source).expect("tokenization should succeed");
    let program = parse(&tokens).expect("parsing should succeed");

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Function(function) => {
            assert_eq!(function.name, "main");
            assert_eq!(function.body.statements.len(), 1);
            match &function.body.statements[0] {
                Statement::Assignment { name, expr } => {
                    assert_eq!(name, "x");
                    match expr {
                        Expr::Call { func, args } => {
                            match &**func {
                                Expr::Identifier(name) => assert_eq!(name, "add"),
                                other => panic!("expected identifier func, got {:?}", other),
                            }
                            assert_eq!(args.len(), 2);
                        }
                        other => panic!("expected call expression, got {:?}", other),
                    }
                }
                other => panic!("expected assignment statement, got {:?}", other),
            }
        }
        other => panic!("expected function statement, got {:?}", other),
    }
}
