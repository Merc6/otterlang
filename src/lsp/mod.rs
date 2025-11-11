use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::runtime::symbol_registry::SymbolRegistry;
use crate::typecheck::{TypeChecker, TypeError};
use ast::nodes::{Program, Statement};
use common::Span;
use lexer::{tokenize, LexerError};
use parser::parse;
use utils::errors::{Diagnostic as OtterDiagnostic, DiagnosticSeverity as OtterDiagSeverity};

/// Symbol table mapping variable names to their definition locations
#[derive(Debug, Clone, Default)]
struct SymbolTable {
    /// Maps variable name to its definition span
    variables: HashMap<String, Span>,
    /// Maps function parameter names to their definition spans
    parameters: HashMap<String, Span>,
}

impl SymbolTable {
    fn new() -> Self {
        Self::default()
    }

    /// Find the definition span for a variable name
    fn find_definition(&self, name: &str) -> Option<&Span> {
        self.parameters.get(name).or_else(|| self.variables.get(name))
    }
}

#[derive(Default, Debug)]
struct DocumentStore {
    documents: HashMap<Url, String>,
    symbol_tables: HashMap<Url, SymbolTable>,
}

#[derive(Debug)]
pub struct Backend {
    client: Client,
    state: Arc<RwLock<DocumentStore>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            state: Arc::new(RwLock::new(DocumentStore::default())),
        }
    }

    async fn upsert_document(&self, uri: Url, text: String) {
        {
            let mut state = self.state.write().await;
            state.documents.insert(uri.clone(), text);
        }
        self.publish_diagnostics(uri).await;
    }

    async fn remove_document(&self, uri: &Url) {
        {
            let mut state = self.state.write().await;
            state.documents.remove(uri);
        }
        let _ = self
            .client
            .publish_diagnostics(uri.clone(), Vec::new(), None)
            .await;
    }

    async fn publish_diagnostics(&self, uri: Url) {
        let text = {
            let state = self.state.read().await;
            state.documents.get(&uri).cloned()
        };

        if let Some(text) = text {
            let (diagnostics, symbol_table) = compute_lsp_diagnostics_and_symbols(&text);
            
            // Store the symbol table
            {
                let mut state = self.state.write().await;
                state.symbol_tables.insert(uri.clone(), symbol_table);
            }
            
            let _ = self
                .client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        }
    }

    async fn document_text(&self, uri: &Url) -> Option<String> {
        let state = self.state.read().await;
        state.documents.get(uri).cloned()
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "otterlang-lsp initialized")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.upsert_document(params.text_document.uri, params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            self.upsert_document(params.text_document.uri, change.text)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.remove_document(&params.text_document.uri).await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let text = self.document_text(&uri).await.unwrap_or_default();

        let mut items = vec![
            CompletionItem::new_simple("print".into(), "fn print(message: string)".into()),
            CompletionItem::new_simple("len".into(), "fn len(value)".into()),
            CompletionItem::new_simple("await".into(), "await expression".into()),
        ];

        items.extend(
            collect_identifiers(&text)
                .into_iter()
                .map(|ident| CompletionItem::new_simple(ident, "identifier".into())),
        );

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(text) = self.document_text(&uri).await {
            if let Some(word) = word_at_position(&text, position) {
                let contents = HoverContents::Scalar(MarkedString::String(format!(
                    "symbol `{}` ({} chars)",
                    word,
                    word.len()
                )));
                return Ok(Some(Hover {
                    contents,
                    range: None,
                }));
            }
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let (text, symbol_table) = {
            let state = self.state.read().await;
            let text = state.documents.get(&uri).cloned();
            let symbol_table = state.symbol_tables.get(&uri).cloned();
            (text, symbol_table)
        };

        if let (Some(text), Some(symbol_table)) = (text, symbol_table) {
            if let Some(var_name) = word_at_position(&text, position) {
                if let Some(span) = symbol_table.find_definition(&var_name) {
                    let range = span_to_range(*span, &text);
                    return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                        uri: uri.clone(),
                        range,
                    })));
                }
            }
        }

        Ok(None)
    }
}

/// Run a standard I/O LSP server using the backend above.
pub async fn run_stdio_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}

/// Build a symbol table from a parsed program
fn build_symbol_table(program: &Program) -> SymbolTable {
    let mut table = SymbolTable::new();
    build_symbol_table_from_statements(&program.statements, &mut table);
    table
}

/// Recursively extract variable definitions from statements
fn build_symbol_table_from_statements(statements: &[Statement], table: &mut SymbolTable) {
    for stmt in statements {
        match stmt {
            Statement::Let { name, span, .. } => {
                if let Some(span) = span {
                    table.variables.insert(name.clone(), *span);
                }
            }
            Statement::Function(func) => {
                for param in &func.params {
                    if let Some(span) = param.span {
                        table.parameters.insert(param.name.clone(), span);
                    }
                }
                build_symbol_table_from_statements(&func.body.statements, table);
            }
            Statement::If {
                then_block,
                elif_blocks,
                else_block,
                ..
            } => {
                build_symbol_table_from_statements(&then_block.statements, table);
                for (_, block) in elif_blocks {
                    build_symbol_table_from_statements(&block.statements, table);
                }
                if let Some(block) = else_block {
                    build_symbol_table_from_statements(&block.statements, table);
                }
            }
            Statement::For { var, var_span, body, .. } => {
                if let Some(span) = var_span {
                    table.variables.insert(var.clone(), *span);
                }
                build_symbol_table_from_statements(&body.statements, table);
            }
            Statement::While { body, .. } => {
                build_symbol_table_from_statements(&body.statements, table);
            }
            Statement::Try {
                body,
                handlers,
                else_block,
                finally_block,
                ..
            } => {
                build_symbol_table_from_statements(&body.statements, table);
                for handler in handlers {
                    build_symbol_table_from_statements(&handler.body.statements, table);
                }
                if let Some(block) = else_block {
                    build_symbol_table_from_statements(&block.statements, table);
                }
                if let Some(block) = finally_block {
                    build_symbol_table_from_statements(&block.statements, table);
                }
            }
            Statement::Block(block) => {
                build_symbol_table_from_statements(&block.statements, table);
            }
            _ => {}
        }
    }
}

/// Compute diagnostics and build symbol table from source text
fn compute_lsp_diagnostics_and_symbols(text: &str) -> (Vec<Diagnostic>, SymbolTable) {
    let source_id = "lsp";
    match tokenize(text) {
        Ok(tokens) => match parse(&tokens) {
            Ok(program) => {
                // Build symbol table from the parsed program
                let symbol_table = build_symbol_table(&program);
                
                let diagnostics = {
                    let mut checker = TypeChecker::new().with_registry(SymbolRegistry::global());
                    if checker.check_program(&program).is_err() {
                        checker.errors().iter().map(type_error_to_lsp).collect()
                    } else {
                        Vec::new()
                    }
                };
                
                (diagnostics, symbol_table)
            }
            Err(errors) => {
                let diagnostics = errors
                    .into_iter()
                    .map(|err| otter_diag_to_lsp(&err.to_diagnostic(source_id), text))
                    .collect();
                (diagnostics, SymbolTable::new())
            }
        },
        Err(errors) => {
            let diagnostics = errors
                .into_iter()
                .map(|err| otter_diag_to_lsp(&lexer_error_to_diag(source_id, &err), text))
                .collect();
            (diagnostics, SymbolTable::new())
        }
    }
}

fn word_at_position(text: &str, position: Position) -> Option<String> {
    let line = text.lines().nth(position.line as usize)?;
    let chars: Vec<char> = line.chars().collect();
    let mut idx = position.character as isize;
    if idx as usize >= chars.len() {
        idx = chars.len() as isize - 1;
    }
    while idx >= 0 && !chars[idx as usize].is_alphanumeric() && chars[idx as usize] != '_' {
        idx -= 1;
    }
    if idx < 0 {
        return None;
    }
    let start = {
        let mut s = idx as usize;
        while s > 0 && (chars[s - 1].is_alphanumeric() || chars[s - 1] == '_') {
            s -= 1;
        }
        s
    };
    let mut end = idx as usize;
    while end + 1 < chars.len() && (chars[end + 1].is_alphanumeric() || chars[end + 1] == '_') {
        end += 1;
    }
    Some(chars[start..=end].iter().collect())
}

fn collect_identifiers(text: &str) -> Vec<String> {
    let mut set = BTreeSet::new();
    for token in text.split(|c: char| !(c.is_alphanumeric() || c == '_')) {
        if token.len() > 1 && token.chars().next().map_or(false, |c| c.is_alphabetic()) {
            set.insert(token.to_string());
        }
    }
    set.into_iter().collect()
}

fn lexer_error_to_diag(source: &str, err: &LexerError) -> OtterDiagnostic {
    err.to_diagnostic(source)
}

fn type_error_to_lsp(err: &TypeError) -> Diagnostic {
    let mut message = err.message.clone();
    if let Some(hint) = &err.hint {
        message.push_str(&format!("\nHint: {}", hint));
    }
    if let Some(help) = &err.help {
        message.push_str(&format!("\nHelp: {}", help));
    }

    Diagnostic {
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 0,
            },
        },
        severity: Some(DiagnosticSeverity::ERROR),
        code: None,
        code_description: None,
        source: Some("otterlang".into()),
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

fn otter_diag_to_lsp(diag: &OtterDiagnostic, text: &str) -> Diagnostic {
    let range = span_to_range(diag.span(), text);
    let mut message = diag.message().to_string();
    if let Some(suggestion) = diag.suggestion() {
        message.push_str(&format!("\nSuggestion: {}", suggestion));
    }
    if let Some(help) = diag.help() {
        message.push_str(&format!("\nHelp: {}", help));
    }

    Diagnostic {
        range,
        severity: Some(match diag.severity() {
            OtterDiagSeverity::Error => DiagnosticSeverity::ERROR,
            OtterDiagSeverity::Warning => DiagnosticSeverity::WARNING,
            OtterDiagSeverity::Info => DiagnosticSeverity::INFORMATION,
            OtterDiagSeverity::Hint => DiagnosticSeverity::HINT,
        }),
        code: None,
        code_description: None,
        source: Some("otterlang".into()),
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

fn span_to_range(span: Span, text: &str) -> Range {
    Range {
        start: offset_to_position(text, span.start()),
        end: offset_to_position(text, span.end()),
    }
}

fn offset_to_position(text: &str, offset: usize) -> Position {
    let mut counted = 0usize;
    let mut line = 0u32;
    let mut character = 0u32;
    for ch in text.chars() {
        if counted >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            character = 0;
        } else {
            character += 1;
        }
        counted += ch.len_utf8();
    }
    Position { line, character }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_symbol_table() {
        let test_code = r#"
let x = 10
let y = 20

def add(a, b):
    let result = a + b
    return result

let sum = add(x, y)

for i in [1, 2, 3]:
    let doubled = i * 2
    print(doubled)
"#;

        match tokenize(test_code) {
            Ok(tokens) => match parse(&tokens) {
                Ok(program) => {
                    let symbol_table = build_symbol_table(&program);
                    
                    assert!(symbol_table.variables.contains_key("x"), "Variable 'x' should be in symbol table");
                    assert!(symbol_table.variables.contains_key("y"), "Variable 'y' should be in symbol table");
                    assert!(symbol_table.variables.contains_key("result"), "Variable 'result' should be in symbol table");
                    assert!(symbol_table.variables.contains_key("sum"), "Variable 'sum' should be in symbol table");
                    assert!(symbol_table.variables.contains_key("doubled"), "Variable 'doubled' should be in symbol table");
                    assert!(symbol_table.parameters.contains_key("a"), "Parameter 'a' should be in symbol table");
                    assert!(symbol_table.parameters.contains_key("b"), "Parameter 'b' should be in symbol table");
                    assert!(symbol_table.variables.contains_key("i"), "Loop variable 'i' should be in symbol table");
                    
                    println!("✓ All symbol table tests passed!");
                    println!("  Variables: {:?}", symbol_table.variables.keys().collect::<Vec<_>>());
                    println!("  Parameters: {:?}", symbol_table.parameters.keys().collect::<Vec<_>>());
                }
                Err(errors) => {
                    panic!("Parsing failed: {:?}", errors);
                }
            },
            Err(errors) => {
                panic!("Tokenization failed: {:?}", errors);
            }
        }
    }

    #[test]
    fn test_find_definition() {
        let test_code = "let x = 10\nlet y = x + 5\n";
        
        match tokenize(test_code) {
            Ok(tokens) => match parse(&tokens) {
                Ok(program) => {
                    let symbol_table = build_symbol_table(&program);
                    
                    let x_span = symbol_table.find_definition("x");
                    assert!(x_span.is_some(), "Should find definition for 'x'");
                    
                    let y_span = symbol_table.find_definition("y");
                    assert!(y_span.is_some(), "Should find definition for 'y'");
                    
                    let z_span = symbol_table.find_definition("z");
                    assert!(z_span.is_none(), "Should not find definition for 'z'");
                }
                Err(errors) => {
                    panic!("Parsing failed: {:?}", errors);
                }
            },
            Err(errors) => {
                panic!("Tokenization failed: {:?}", errors);
            }
        }
    }
}
