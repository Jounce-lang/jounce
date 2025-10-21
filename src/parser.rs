use crate::ast::*;
use crate::errors::CompileError;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Precedence {
    Lowest,
    Equals,      // == !=
    LessGreater, // < > <= >=
    Sum,         // + -
    Product,     // * / %
}

lazy_static::lazy_static! {
    static ref PRECEDENCES: HashMap<TokenKind, Precedence> = {
        let mut m = HashMap::new();
        m.insert(TokenKind::Eq, Precedence::Equals);
        m.insert(TokenKind::NotEq, Precedence::Equals);
        m.insert(TokenKind::LAngle, Precedence::LessGreater);
        m.insert(TokenKind::RAngle, Precedence::LessGreater);
        m.insert(TokenKind::LtEq, Precedence::LessGreater);
        m.insert(TokenKind::GtEq, Precedence::LessGreater);
        m.insert(TokenKind::Plus, Precedence::Sum);
        m.insert(TokenKind::Minus, Precedence::Sum);
        m.insert(TokenKind::Star, Precedence::Product);
        m.insert(TokenKind::Slash, Precedence::Product);
        m.insert(TokenKind::Percent, Precedence::Product);
        m
    };
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current: Token,
    peek: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let current = lexer.next_token();
        let peek = lexer.next_token();
        Self { lexer, current, peek }
    }

    pub fn parse_program(&mut self) -> Result<Program, CompileError> {
        let mut statements = Vec::new();
        while self.current_token().kind != TokenKind::Eof {
            statements.push(self.parse_statement()?);
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, CompileError> {
        let stmt = match self.current_token().kind {
            TokenKind::Use => self.parse_use_statement().map(Statement::Use),
            TokenKind::Struct => self.parse_struct_definition().map(Statement::Struct),
            TokenKind::Enum => self.parse_enum_definition().map(Statement::Enum),
            TokenKind::Impl => self.parse_impl_block().map(Statement::ImplBlock),
            TokenKind::Trait => self.parse_trait_definition().map(Statement::Trait),
            TokenKind::Component => self.parse_component_definition().map(Statement::Component),
            TokenKind::At => {
                // Annotations can be on functions or components
                // Try parsing as function first, which handles @server/@client fn
                // Components are marked with "component" keyword, not @client
                self.parse_function_definition().map(Statement::Function)
            }
            TokenKind::Fn | TokenKind::Server | TokenKind::Client | TokenKind::Async => self.parse_function_definition().map(Statement::Function),
            TokenKind::Let => self.parse_let_statement().map(Statement::Let),
            TokenKind::Return => self.parse_return_statement().map(Statement::Return),
            TokenKind::If => self.parse_if_statement().map(Statement::If),
            TokenKind::While => self.parse_while_statement().map(Statement::While),
            TokenKind::For => {
                // Look ahead to distinguish between for-in and C-style for loop
                // for item in collection { } vs for (init; cond; update) { }
                if self.peek_token().kind == TokenKind::LParen {
                    self.parse_for_statement().map(Statement::For)
                } else {
                    self.parse_for_in_statement().map(Statement::ForIn)
                }
            },
            TokenKind::Identifier => {
                // Check if this is an assignment
                if self.peek_token().kind == TokenKind::Assign {
                    self.parse_assignment_statement().map(Statement::Assignment)
                } else {
                    self.parse_expression_statement().map(Statement::Expression)
                }
            }
            _ => self.parse_expression_statement().map(Statement::Expression),
        }?;
        self.consume_if_matches(&TokenKind::Semicolon);
        Ok(stmt)
    }

    fn parse_use_statement(&mut self) -> Result<UseStatement, CompileError> {
        self.expect_and_consume(&TokenKind::Use)?;
        let mut path = vec![self.parse_identifier()?];
        while self.consume_if_matches(&TokenKind::DoubleColon) {
            if self.current_token().kind == TokenKind::LBrace { break; }
            path.push(self.parse_identifier()?);
        }
        let mut imports = Vec::new();
        if self.consume_if_matches(&TokenKind::LBrace) {
            while self.current_token().kind != TokenKind::RBrace {
                imports.push(self.parse_identifier()?);
                if !self.consume_if_matches(&TokenKind::Comma) { break; }
            }
            self.expect_and_consume(&TokenKind::RBrace)?;
        }
        Ok(UseStatement { path, imports })
    }
    
    fn parse_type_params(&mut self) -> Result<Vec<Identifier>, CompileError> {
        if !self.consume_if_matches(&TokenKind::LAngle) {
            return Ok(Vec::new());
        }

        let mut type_params = Vec::new();
        while self.current_token().kind != TokenKind::RAngle {
            type_params.push(self.parse_identifier()?);
            if !self.consume_if_matches(&TokenKind::Comma) {
                break;
            }
        }
        self.expect_and_consume(&TokenKind::RAngle)?;
        Ok(type_params)
    }

    fn parse_struct_definition(&mut self) -> Result<StructDefinition, CompileError> {
        self.expect_and_consume(&TokenKind::Struct)?;
        let name = self.parse_identifier()?;
        let type_params = self.parse_type_params()?;
        self.expect_and_consume(&TokenKind::LBrace)?;
        let mut fields = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            let field_name = self.parse_identifier()?;
            self.expect_and_consume(&TokenKind::Colon)?;
            let field_type = self.parse_type_expression()?;
            fields.push((field_name, field_type));
            if !self.consume_if_matches(&TokenKind::Comma) { break; }
        }
        self.expect_and_consume(&TokenKind::RBrace)?;
        Ok(StructDefinition { name, lifetime_params: Vec::new(), type_params, fields, derives: Vec::new() })
    }

    fn parse_enum_definition(&mut self) -> Result<EnumDefinition, CompileError> {
        self.expect_and_consume(&TokenKind::Enum)?;
        let name = self.parse_identifier()?;
        let type_params = self.parse_type_params()?;
        self.expect_and_consume(&TokenKind::LBrace)?;

        let mut variants = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            let variant_name = self.parse_identifier()?;

            // Check if this variant has associated data
            let fields = if self.consume_if_matches(&TokenKind::LBrace) {
                // Struct-style variant: Name { field1: Type, field2: Type }
                let mut variant_fields = Vec::new();
                while self.current_token().kind != TokenKind::RBrace {
                    let field_name = self.parse_identifier()?;
                    self.expect_and_consume(&TokenKind::Colon)?;
                    let field_type = self.parse_type_expression()?;
                    variant_fields.push((field_name, field_type));
                    if !self.consume_if_matches(&TokenKind::Comma) { break; }
                }
                self.expect_and_consume(&TokenKind::RBrace)?;
                Some(variant_fields)
            } else if self.consume_if_matches(&TokenKind::LParen) {
                // Tuple-style variant: Name(Type1, Type2)
                let mut variant_fields = Vec::new();
                let mut index = 0;
                while self.current_token().kind != TokenKind::RParen {
                    let field_type = self.parse_type_expression()?;
                    // For tuple variants, use numeric field names
                    variant_fields.push((
                        Identifier { value: index.to_string() },
                        field_type,
                    ));
                    index += 1;
                    if !self.consume_if_matches(&TokenKind::Comma) { break; }
                }
                self.expect_and_consume(&TokenKind::RParen)?;
                Some(variant_fields)
            } else {
                // Simple variant with no data: Name
                None
            };

            variants.push(EnumVariant {
                name: variant_name,
                fields,
            });

            if !self.consume_if_matches(&TokenKind::Comma) { break; }
        }
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(EnumDefinition { name, lifetime_params: Vec::new(), type_params, variants, derives: Vec::new() })
    }

    fn parse_impl_block(&mut self) -> Result<ImplBlock, CompileError> {
        // impl<T> TypeName { methods... } or impl<T> Trait for TypeName { methods... }
        self.expect_and_consume(&TokenKind::Impl)?;

        // Parse optional type parameters
        let type_params = self.parse_type_params()?;

        let first_name = self.parse_identifier()?;

        // Skip optional type arguments on the first name (e.g., Box<T>)
        // We store type_params separately, so we can ignore these for now
        if self.current_token().kind == TokenKind::LAngle {
            self.parse_type_params()?; // Consume and ignore type arguments
        }

        // Check if this is "impl Trait for Type" or just "impl Type"
        let (trait_name, type_name) = if self.consume_if_matches(&TokenKind::For) {
            // This is a trait implementation
            let type_name = self.parse_identifier()?;

            // Skip optional type arguments on the type name
            if self.current_token().kind == TokenKind::LAngle {
                self.parse_type_params()?; // Consume and ignore type arguments
            }

            (Some(first_name), type_name)
        } else {
            // This is an inherent implementation
            (None, first_name)
        };

        self.expect_and_consume(&TokenKind::LBrace)?;

        let mut methods = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            // Parse method: fn method_name(...) -> ReturnType { body }
            self.expect_and_consume(&TokenKind::Fn)?;
            let method_name = self.parse_identifier()?;

            // Parse parameter list
            self.expect_and_consume(&TokenKind::LParen)?;
            let mut parameters = Vec::new();
            while self.current_token().kind != TokenKind::RParen {
                let param_name = self.parse_identifier()?;
                self.expect_and_consume(&TokenKind::Colon)?;
                let param_type = self.parse_type_expression()?;
                parameters.push(FunctionParameter {
                    name: param_name,
                    type_annotation: param_type,
                });
                if !self.consume_if_matches(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect_and_consume(&TokenKind::RParen)?;

            // Parse optional return type (-> Type)
            let return_type = if self.consume_if_matches(&TokenKind::Arrow) {
                Some(self.parse_type_expression()?)
            } else {
                None
            };

            // Parse method body (block statement)
            self.expect_and_consume(&TokenKind::LBrace)?;
            let mut statements = Vec::new();
            while self.current_token().kind != TokenKind::RBrace {
                statements.push(self.parse_statement()?);
            }
            self.expect_and_consume(&TokenKind::RBrace)?;

            methods.push(ImplMethod {
                name: method_name,
                parameters,
                return_type,
                body: BlockStatement { statements },
            });
        }

        self.expect_and_consume(&TokenKind::RBrace)?;
        Ok(ImplBlock { trait_name, lifetime_params: Vec::new(), type_params, type_name, methods })
    }

    fn parse_trait_definition(&mut self) -> Result<TraitDefinition, CompileError> {
        // trait TraitName<T> { method signatures... }
        self.expect_and_consume(&TokenKind::Trait)?;
        let name = self.parse_identifier()?;
        let type_params = self.parse_type_params()?;
        self.expect_and_consume(&TokenKind::LBrace)?;

        let mut methods = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            // Parse method signature: fn method_name(...) -> ReturnType;
            self.expect_and_consume(&TokenKind::Fn)?;
            let method_name = self.parse_identifier()?;

            // Parse parameter list
            self.expect_and_consume(&TokenKind::LParen)?;
            let mut parameters = Vec::new();
            while self.current_token().kind != TokenKind::RParen {
                let param_name = self.parse_identifier()?;
                self.expect_and_consume(&TokenKind::Colon)?;
                let param_type = self.parse_type_expression()?;
                parameters.push(FunctionParameter {
                    name: param_name,
                    type_annotation: param_type,
                });
                if !self.consume_if_matches(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect_and_consume(&TokenKind::RParen)?;

            // Parse optional return type (-> Type)
            let return_type = if self.consume_if_matches(&TokenKind::Arrow) {
                Some(self.parse_type_expression()?)
            } else {
                None
            };

            // Consume semicolon (trait methods are signatures only)
            self.consume_if_matches(&TokenKind::Semicolon);

            methods.push(TraitMethod {
                name: method_name,
                parameters,
                return_type,
            });
        }

        self.expect_and_consume(&TokenKind::RBrace)?;
        Ok(TraitDefinition { name, lifetime_params: Vec::new(), type_params, methods })
    }

    fn parse_component_definition(&mut self) -> Result<ComponentDefinition, CompileError> {
        // Check for optional @client annotation (components are client-only by default)
        let has_at = self.consume_if_matches(&TokenKind::At);
        let is_client = if has_at {
            self.consume_if_matches(&TokenKind::Client)
        } else {
            true  // Components are client-side by default
        };

        self.expect_and_consume(&TokenKind::Component)?;
        let name = self.parse_identifier()?;
        self.expect_and_consume(&TokenKind::LParen)?;
        let mut parameters = Vec::new();
        while self.current_token().kind != TokenKind::RParen {
            let param_name = self.parse_identifier()?;
            self.expect_and_consume(&TokenKind::Colon)?;
            let param_type = self.parse_type_expression()?;
            parameters.push(FunctionParameter {
                name: param_name,
                type_annotation: param_type,
            });
            if !self.consume_if_matches(&TokenKind::Comma) { break; }
        }
        self.expect_and_consume(&TokenKind::RParen)?;
        self.expect_and_consume(&TokenKind::LBrace)?;

        // Parse component body as a block of statements
        let mut statements = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            statements.push(self.parse_statement()?);
        }
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(ComponentDefinition {
            name,
            parameters,
            is_client,
            body: BlockStatement { statements },
        })
    }

    fn parse_function_definition(&mut self) -> Result<FunctionDefinition, CompileError> {
        // Check for optional @ symbol (for @server or @client)
        let has_at = self.consume_if_matches(&TokenKind::At);

        // Check for server/client annotations (with or without @)
        let is_server = if has_at {
            self.consume_if_matches(&TokenKind::Server)
        } else {
            self.consume_if_matches(&TokenKind::Server)
        };

        let is_client = if has_at && !is_server {
            self.consume_if_matches(&TokenKind::Client)
        } else if !has_at {
            self.consume_if_matches(&TokenKind::Client)
        } else {
            false
        };

        let is_async = self.consume_if_matches(&TokenKind::Async);

        // Expect fn keyword
        self.expect_and_consume(&TokenKind::Fn)?;

        // Parse function name
        let name = self.parse_identifier()?;

        // Parse optional type parameters
        let type_params = self.parse_type_params()?;

        // Parse parameter list
        self.expect_and_consume(&TokenKind::LParen)?;
        let mut parameters = Vec::new();
        while self.current_token().kind != TokenKind::RParen {
            let param_name = self.parse_identifier()?;
            self.expect_and_consume(&TokenKind::Colon)?;
            let param_type = self.parse_type_expression()?;
            parameters.push(FunctionParameter {
                name: param_name,
                type_annotation: param_type,
            });
            if !self.consume_if_matches(&TokenKind::Comma) { break; }
        }
        self.expect_and_consume(&TokenKind::RParen)?;

        // Parse optional return type (-> Type)
        let _return_type = if self.consume_if_matches(&TokenKind::Arrow) {
            Some(self.parse_type_expression()?)
        } else {
            None
        };

        // Parse function body (block statement)
        self.expect_and_consume(&TokenKind::LBrace)?;
        let mut statements = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            statements.push(self.parse_statement()?);
        }
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(FunctionDefinition {
            name,
            lifetime_params: Vec::new(),
            type_params,
            parameters,
            is_server,
            is_client,
            is_async,
            body: BlockStatement { statements },
        })
    }

    fn parse_type_expression(&mut self) -> Result<TypeExpression, CompileError> {
        // Check if this is a function type: fn(T1, T2) -> R
        if self.consume_if_matches(&TokenKind::Fn) {
            self.expect_and_consume(&TokenKind::LParen)?;
            let mut param_types = Vec::new();
            while self.current_token().kind != TokenKind::RParen {
                param_types.push(self.parse_type_expression()?);
                if !self.consume_if_matches(&TokenKind::Comma) { break; }
            }
            self.expect_and_consume(&TokenKind::RParen)?;
            self.expect_and_consume(&TokenKind::Arrow)?;
            let return_type = Box::new(self.parse_type_expression()?);
            return Ok(TypeExpression::Function(param_types, return_type));
        }

        // Check if this is a slice type [T]
        if self.consume_if_matches(&TokenKind::LBracket) {
            let inner_type = self.parse_type_expression()?;
            self.expect_and_consume(&TokenKind::RBracket)?;
            return Ok(TypeExpression::Slice(Box::new(inner_type)));
        }

        // Check if this is a reference type (&T or &mut T)
        if self.consume_if_matches(&TokenKind::Ampersand) {
            // Check for &mut T (mutable reference)
            if self.consume_if_matches(&TokenKind::Mut) {
                let inner_type = self.parse_type_expression()?;
                return Ok(TypeExpression::MutableReference(Box::new(inner_type)));
            }
            // Otherwise it's &T (immutable reference)
            let inner_type = self.parse_type_expression()?;
            return Ok(TypeExpression::Reference(Box::new(inner_type)));
        }

        let name = self.parse_identifier()?;
        if self.consume_if_matches(&TokenKind::LAngle) {
            let mut args = Vec::new();
            while self.current_token().kind != TokenKind::RAngle {
                args.push(self.parse_type_expression()?);
                if !self.consume_if_matches(&TokenKind::Comma) { break; }
            }
            self.expect_and_consume(&TokenKind::RAngle)?;
            Ok(TypeExpression::Generic(name, args))
        } else {
            Ok(TypeExpression::Named(name))
        }
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, CompileError> {
        self.expect_and_consume(&TokenKind::Let)?;
        let name = self.parse_identifier()?;

        // Parse optional type annotation: let x: Type = value
        let type_annotation = if self.consume_if_matches(&TokenKind::Colon) {
            Some(self.parse_type_expression()?)
        } else {
            None
        };

        self.expect_and_consume(&TokenKind::Assign)?;
        let value = self.parse_expression(Precedence::Lowest)?;
        Ok(LetStatement { name, type_annotation, value })
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, CompileError> {
        self.expect_and_consume(&TokenKind::Return)?;

        // Check if this is a bare return (followed by semicolon or closing brace)
        let value = if self.current_token().kind == TokenKind::Semicolon ||
                       self.current_token().kind == TokenKind::RBrace {
            // Bare return - return unit/void value represented as integer literal 0
            Expression::IntegerLiteral(0)
        } else {
            self.parse_expression(Precedence::Lowest)?
        };

        Ok(ReturnStatement { value })
    }

    fn parse_if_statement(&mut self) -> Result<IfStatement, CompileError> {
        self.expect_and_consume(&TokenKind::If)?;
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_and_consume(&TokenKind::LBrace)?;

        // Parse then branch
        let mut then_statements = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            then_statements.push(self.parse_statement()?);
        }
        self.expect_and_consume(&TokenKind::RBrace)?;
        let then_branch = BlockStatement { statements: then_statements };

        // Parse optional else branch
        let else_branch = if self.consume_if_matches(&TokenKind::Else) {
            if self.current_token().kind == TokenKind::If {
                // else if
                Some(Box::new(self.parse_if_statement().map(Statement::If)?))
            } else {
                // else block
                self.expect_and_consume(&TokenKind::LBrace)?;
                let mut else_statements = Vec::new();
                while self.current_token().kind != TokenKind::RBrace {
                    else_statements.push(self.parse_statement()?);
                }
                self.expect_and_consume(&TokenKind::RBrace)?;
                Some(Box::new(Statement::Expression(Expression::Identifier(
                    Identifier { value: "__else_block".to_string() }
                )))) // Placeholder, will need better handling
            }
        } else {
            None
        };

        Ok(IfStatement {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while_statement(&mut self) -> Result<WhileStatement, CompileError> {
        self.expect_and_consume(&TokenKind::While)?;
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_and_consume(&TokenKind::LBrace)?;

        // Parse loop body
        let mut body_statements = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            body_statements.push(self.parse_statement()?);
        }
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(WhileStatement {
            condition,
            body: BlockStatement { statements: body_statements },
        })
    }

    fn parse_for_statement(&mut self) -> Result<ForStatement, CompileError> {
        self.expect_and_consume(&TokenKind::For)?;
        self.expect_and_consume(&TokenKind::LParen)?;

        // Parse init (optional)
        let init = if self.current_token().kind == TokenKind::Semicolon {
            None
        } else {
            Some(Box::new(self.parse_statement()?))
        };

        // Consume semicolon after init (if there was one, or standalone)
        if !self.consume_if_matches(&TokenKind::Semicolon) {
            // parse_statement might have consumed it already
        }

        // Parse condition
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_and_consume(&TokenKind::Semicolon)?;

        // Parse update (optional)
        let update = if self.current_token().kind == TokenKind::RParen {
            None
        } else {
            Some(Box::new(self.parse_statement()?))
        };

        self.expect_and_consume(&TokenKind::RParen)?;
        self.expect_and_consume(&TokenKind::LBrace)?;

        // Parse loop body
        let mut body_statements = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            body_statements.push(self.parse_statement()?);
        }
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(ForStatement {
            init,
            condition,
            update,
            body: BlockStatement { statements: body_statements },
        })
    }

    fn parse_for_in_statement(&mut self) -> Result<ForInStatement, CompileError> {
        // Parse: for item in collection { body }
        self.expect_and_consume(&TokenKind::For)?;

        // Parse loop variable
        let variable = self.parse_identifier()?;

        // Expect 'in' keyword
        self.expect_and_consume(&TokenKind::In)?;

        // Parse iterator expression
        let iterator = self.parse_expression(Precedence::Lowest)?;

        // Parse loop body
        self.expect_and_consume(&TokenKind::LBrace)?;
        let mut body_statements = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            body_statements.push(self.parse_statement()?);
        }
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(ForInStatement {
            variable,
            iterator,
            body: BlockStatement { statements: body_statements },
        })
    }

    fn parse_assignment_statement(&mut self) -> Result<AssignmentStatement, CompileError> {
        let target = self.parse_identifier()?;
        self.expect_and_consume(&TokenKind::Assign)?;
        let value = self.parse_expression(Precedence::Lowest)?;
        Ok(AssignmentStatement { target, value })
    }

    fn parse_expression_statement(&mut self) -> Result<Expression, CompileError> {
        self.parse_expression(Precedence::Lowest)
    }
    
    #[allow(dead_code)] // For future macro support
    fn parse_macro_invocation(&mut self) -> Result<Statement, CompileError> {
        let macro_token = self.current_token().clone();
        self.next_token();
        let mut input_tokens = Vec::new();
        let mut brace_level = 0;
        let mut paren_level = 0;
        loop {
            let token = self.current_token().clone();
            input_tokens.push(token.clone());
            self.next_token();
            match token.kind {
                TokenKind::LParen => paren_level += 1,
                TokenKind::RParen => paren_level -= 1,
                TokenKind::LBrace => brace_level += 1,
                TokenKind::RBrace => brace_level -= 1,
                TokenKind::Eof => return Err(self.error("Unmatched braces in macro invocation")),
                _ => {}
            }
            if paren_level == 0 && brace_level == 0 { break; }
        }
        Ok(Statement::MacroInvocation(MacroInvocation {
            name: Identifier { value: macro_token.lexeme },
            input_tokens,
        }))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, CompileError> {
        let mut left_expr = self.parse_prefix()?;
        while self.current_token().kind != TokenKind::Semicolon && precedence < self.current_precedence() {
            left_expr = self.parse_infix(left_expr)?;
        }
        Ok(left_expr)
    }

    fn parse_prefix(&mut self) -> Result<Expression, CompileError> {
        let token = self.current_token().clone();
        match &token.kind {
            TokenKind::Identifier => {
                self.next_token();
                let ident = Identifier { value: token.lexeme };

                // Check if this is a struct literal (Identifier { field: value, ... })
                if self.current_token().kind == TokenKind::LBrace {
                    return self.parse_struct_literal(ident);
                }

                let mut expr = Expression::Identifier(ident);

                // Check for postfix operations (function call, field access, namespace resolution, array indexing, or try operator)
                loop {
                    match self.current_token().kind {
                        TokenKind::LParen => {
                            expr = self.parse_function_call(expr)?;
                        }
                        TokenKind::Dot => {
                            self.next_token(); // consume the dot
                            let field = self.parse_identifier()?;
                            expr = Expression::FieldAccess(FieldAccessExpression {
                                object: Box::new(expr),
                                field,
                            });
                        }
                        TokenKind::DoubleColon => {
                            // Handle namespace resolution: console::log()
                            self.next_token(); // consume the ::
                            let next_ident = self.parse_identifier()?;

                            // Build the full namespaced identifier
                            if let Expression::Identifier(base_ident) = expr {
                                let namespaced_name = Identifier {
                                    value: format!("{}::{}", base_ident.value, next_ident.value)
                                };
                                expr = Expression::Identifier(namespaced_name);
                            } else {
                                // If the left side is not an identifier, treat it as field access for now
                                expr = Expression::FieldAccess(FieldAccessExpression {
                                    object: Box::new(expr),
                                    field: next_ident,
                                });
                            }
                        }
                        TokenKind::LBracket => {
                            self.next_token(); // consume the [
                            let index = self.parse_expression(Precedence::Lowest)?;
                            self.expect_and_consume(&TokenKind::RBracket)?;
                            expr = Expression::IndexAccess(IndexExpression {
                                array: Box::new(expr),
                                index: Box::new(index),
                            });
                        }
                        TokenKind::Question => {
                            self.next_token(); // consume the ?
                            expr = Expression::TryOperator(TryOperatorExpression {
                                expression: Box::new(expr),
                            });
                        }
                        _ => break,
                    }
                }

                Ok(expr)
            },
            TokenKind::Integer(val) => { self.next_token(); Ok(Expression::IntegerLiteral(*val)) },
            TokenKind::Float(val) => { self.next_token(); Ok(Expression::FloatLiteral(val.clone())) },
            TokenKind::String(val) => { self.next_token(); Ok(Expression::StringLiteral(val.clone())) },
            TokenKind::Bool(val) => { self.next_token(); Ok(Expression::BoolLiteral(*val)) },
            TokenKind::Minus | TokenKind::Bang => {
                // Parse prefix expressions: -x or !x
                let operator = token.clone();
                self.next_token();
                let right = self.parse_expression(Precedence::Product)?; // High precedence for prefix ops
                Ok(Expression::Prefix(PrefixExpression {
                    operator,
                    right: Box::new(right),
                }))
            },
            TokenKind::Ampersand => {
                // Parse borrow expression: &x or &mut x
                self.next_token();
                // Check for &mut x (mutable borrow)
                if self.consume_if_matches(&TokenKind::Mut) {
                    let expression = self.parse_expression(Precedence::Product)?;
                    Ok(Expression::MutableBorrow(MutableBorrowExpression {
                        expression: Box::new(expression),
                    }))
                } else {
                    // Otherwise it's &x (immutable borrow)
                    let expression = self.parse_expression(Precedence::Product)?;
                    Ok(Expression::Borrow(BorrowExpression {
                        expression: Box::new(expression),
                    }))
                }
            },
            TokenKind::Star => {
                // Parse dereference expression: *x
                self.next_token();
                let expression = self.parse_expression(Precedence::Product)?; // High precedence for prefix ops
                Ok(Expression::Dereference(DereferenceExpression {
                    expression: Box::new(expression),
                }))
            },
            TokenKind::Await => {
                // Parse await expression: await future_expr
                self.next_token();
                let expression = self.parse_expression(Precedence::Product)?; // High precedence for prefix ops
                Ok(Expression::Await(AwaitExpression {
                    expression: Box::new(expression),
                }))
            },
            TokenKind::LParen => self.parse_lambda_or_grouped(),
            TokenKind::LAngle => self.parse_jsx_element(),
            TokenKind::Pipe => self.parse_lambda_with_pipes(),
            TokenKind::LBracket => self.parse_array_literal(),
            TokenKind::Match => self.parse_match_expression(),
            TokenKind::If => {
                // Parse if-expression: if cond { then_expr } else { else_expr }
                self.next_token(); // consume if
                let condition = Box::new(self.parse_expression(Precedence::Lowest)?);

                // Parse then block
                self.expect_and_consume(&TokenKind::LBrace)?;
                let mut then_statements = Vec::new();
                while self.current_token().kind != TokenKind::RBrace {
                    then_statements.push(self.parse_statement()?);
                }
                self.expect_and_consume(&TokenKind::RBrace)?;
                let then_expr = Box::new(Expression::Block(BlockStatement { statements: then_statements }));

                // Parse optional else block
                let else_expr = if self.consume_if_matches(&TokenKind::Else) {
                    if self.current_token().kind == TokenKind::If {
                        // else if - parse as nested if-expression
                        Some(Box::new(self.parse_prefix()?))
                    } else {
                        // else block
                        self.expect_and_consume(&TokenKind::LBrace)?;
                        let mut else_statements = Vec::new();
                        while self.current_token().kind != TokenKind::RBrace {
                            else_statements.push(self.parse_statement()?);
                        }
                        self.expect_and_consume(&TokenKind::RBrace)?;
                        Some(Box::new(Expression::Block(BlockStatement { statements: else_statements })))
                    }
                } else {
                    None
                };

                Ok(Expression::IfExpression(IfExpression {
                    condition,
                    then_expr,
                    else_expr,
                }))
            },
            TokenKind::LBrace => {
                // Parse block as expression: { statements... }
                self.next_token(); // consume {
                let mut statements = Vec::new();
                while self.current_token().kind != TokenKind::RBrace {
                    statements.push(self.parse_statement()?);
                }
                self.expect_and_consume(&TokenKind::RBrace)?;
                Ok(Expression::Block(BlockStatement { statements }))
            },
            _ => Err(self.error(&format!("No prefix parse function for {:?}", token.kind))),
        }
    }

    fn parse_function_call(&mut self, function: Expression) -> Result<Expression, CompileError> {
        self.expect_and_consume(&TokenKind::LParen)?;
        let mut arguments = Vec::new();
        while self.current_token().kind != TokenKind::RParen {
            arguments.push(self.parse_expression(Precedence::Lowest)?);
            if !self.consume_if_matches(&TokenKind::Comma) { break; }
        }
        self.expect_and_consume(&TokenKind::RParen)?;
        Ok(Expression::FunctionCall(FunctionCall {
            function: Box::new(function),
            arguments,
        }))
    }

    fn parse_lambda_or_grouped(&mut self) -> Result<Expression, CompileError> {
        self.expect_and_consume(&TokenKind::LParen)?;

        // Try to determine if this is a lambda, tuple, or grouped expression
        if self.current_token().kind == TokenKind::RParen {
            // Empty parameter list for lambda: () =>
            self.expect_and_consume(&TokenKind::RParen)?;
            if self.consume_if_matches(&TokenKind::FatArrow) {
                let body = self.parse_expression(Precedence::Lowest)?;
                return Ok(Expression::Lambda(LambdaExpression {
                    parameters: vec![],
                    body: Box::new(body),
                    captures: vec![],  // Will be analyzed later
                }));
            }
            // Just empty parens, not lambda - error
            return Err(self.error("Unexpected empty parentheses"));
        }

        // Parse first expression
        let first_expr = self.parse_expression(Precedence::Lowest)?;

        // Check if this is a tuple (has comma after first element)
        if self.consume_if_matches(&TokenKind::Comma) {
            // This is a tuple: (expr, ...)
            let mut elements = vec![first_expr];

            // Parse remaining elements
            while self.current_token().kind != TokenKind::RParen {
                elements.push(self.parse_expression(Precedence::Lowest)?);
                if !self.consume_if_matches(&TokenKind::Comma) {
                    break;
                }
            }

            self.expect_and_consume(&TokenKind::RParen)?;
            return Ok(Expression::TupleLiteral(TupleLiteral { elements }));
        }

        // No comma, so it's either grouped expression or lambda
        self.expect_and_consume(&TokenKind::RParen)?;

        // Check if this is actually a lambda with single param: (x) => body
        if self.consume_if_matches(&TokenKind::FatArrow) {
            if let Expression::Identifier(param) = first_expr {
                let body = self.parse_expression(Precedence::Lowest)?;
                return Ok(Expression::Lambda(LambdaExpression {
                    parameters: vec![param],
                    body: Box::new(body),
                    captures: vec![],  // Will be analyzed later
                }));
            }
        }

        // Just a grouped expression
        Ok(first_expr)
    }

    fn parse_lambda_with_pipes(&mut self) -> Result<Expression, CompileError> {
        self.expect_and_consume(&TokenKind::Pipe)?;
        let mut parameters = Vec::new();
        while self.current_token().kind != TokenKind::Pipe {
            parameters.push(self.parse_identifier()?);
            if !self.consume_if_matches(&TokenKind::Comma) { break; }
        }
        self.expect_and_consume(&TokenKind::Pipe)?;

        // Check for => or just use the next expression
        self.consume_if_matches(&TokenKind::FatArrow);

        let body = self.parse_expression(Precedence::Lowest)?;
        Ok(Expression::Lambda(LambdaExpression {
            parameters,
            body: Box::new(body),
            captures: vec![],  // Will be analyzed later
        }))
    }

    fn parse_array_literal(&mut self) -> Result<Expression, CompileError> {
        self.expect_and_consume(&TokenKind::LBracket)?;
        let mut elements = Vec::new();

        // Handle empty array []
        if self.current_token().kind == TokenKind::RBracket {
            self.expect_and_consume(&TokenKind::RBracket)?;
            return Ok(Expression::ArrayLiteral(ArrayLiteral { elements }));
        }

        // Parse comma-separated elements
        while self.current_token().kind != TokenKind::RBracket {
            elements.push(self.parse_expression(Precedence::Lowest)?);
            if !self.consume_if_matches(&TokenKind::Comma) {
                break;
            }
        }

        self.expect_and_consume(&TokenKind::RBracket)?;
        Ok(Expression::ArrayLiteral(ArrayLiteral { elements }))
    }

    fn parse_struct_literal(&mut self, name: Identifier) -> Result<Expression, CompileError> {
        self.expect_and_consume(&TokenKind::LBrace)?;
        let mut fields = Vec::new();

        // Handle empty struct literal {}
        if self.current_token().kind == TokenKind::RBrace {
            self.expect_and_consume(&TokenKind::RBrace)?;
            return Ok(Expression::StructLiteral(StructLiteral { name, fields }));
        }

        // Parse comma-separated field: value pairs
        while self.current_token().kind != TokenKind::RBrace {
            let field_name = self.parse_identifier()?;

            // Check for field shorthand: if followed by comma or }, use field_name as both key and value
            if self.current_token().kind == TokenKind::Comma || self.current_token().kind == TokenKind::RBrace {
                // Field shorthand: `username,` is equivalent to `username: username,`
                let field_value = Expression::Identifier(field_name.clone());
                fields.push((field_name, field_value));
            } else {
                // Regular field: value syntax
                self.expect_and_consume(&TokenKind::Colon)?;
                let field_value = self.parse_expression(Precedence::Lowest)?;
                fields.push((field_name, field_value));
            }

            if !self.consume_if_matches(&TokenKind::Comma) {
                break;
            }
        }

        self.expect_and_consume(&TokenKind::RBrace)?;
        Ok(Expression::StructLiteral(StructLiteral { name, fields }))
    }

    // Parse an expression without treating { as struct literal
    // Used for match scrutinee where { starts the match arms
    fn parse_simple_expression(&mut self) -> Result<Expression, CompileError> {
        let token = self.current_token().clone();
        match &token.kind {
            TokenKind::Identifier => {
                self.next_token();
                let ident = Identifier { value: token.lexeme };
                let mut expr = Expression::Identifier(ident);

                // Check for postfix operations but NOT struct literals
                loop {
                    match self.current_token().kind {
                        TokenKind::LParen => {
                            expr = self.parse_function_call(expr)?;
                        }
                        TokenKind::Dot => {
                            self.next_token();
                            let field = self.parse_identifier()?;
                            expr = Expression::FieldAccess(FieldAccessExpression {
                                object: Box::new(expr),
                                field,
                            });
                        }
                        TokenKind::DoubleColon => {
                            self.next_token();
                            let next_ident = self.parse_identifier()?;
                            if let Expression::Identifier(base_ident) = expr {
                                expr = Expression::Identifier(Identifier {
                                    value: format!("{}::{}", base_ident.value, next_ident.value)
                                });
                            }
                        }
                        TokenKind::LBracket => {
                            self.next_token();
                            let index = self.parse_expression(Precedence::Lowest)?;
                            self.expect_and_consume(&TokenKind::RBracket)?;
                            expr = Expression::IndexAccess(IndexExpression {
                                array: Box::new(expr),
                                index: Box::new(index),
                            });
                        }
                        _ => break,
                    }
                }
                Ok(expr)
            }
            TokenKind::Integer(val) => {
                self.next_token();
                Ok(Expression::IntegerLiteral(*val))
            }
            TokenKind::String(val) => {
                self.next_token();
                Ok(Expression::StringLiteral(val.clone()))
            }
            TokenKind::LParen => {
                self.next_token();
                let expr = self.parse_expression(Precedence::Lowest)?;
                self.expect_and_consume(&TokenKind::RParen)?;
                Ok(expr)
            }
            _ => self.parse_expression(Precedence::Lowest),
        }
    }

    fn parse_match_expression(&mut self) -> Result<Expression, CompileError> {
        self.expect_and_consume(&TokenKind::Match)?;

        // Parse the scrutinee (the value being matched)
        // We need to parse just the expression without allowing struct literals
        // because the { after the scrutinee starts the match arms, not a struct literal
        let scrutinee = Box::new(self.parse_simple_expression()?);

        self.expect_and_consume(&TokenKind::LBrace)?;

        // Parse match arms
        let mut arms = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            let pattern = self.parse_pattern()?;
            self.expect_and_consume(&TokenKind::FatArrow)?;

            // Parse body - can be a block { ... } or an expression
            let body = if self.current_token().kind == TokenKind::LBrace {
                // Parse block as expression
                self.expect_and_consume(&TokenKind::LBrace)?;
                let mut statements = Vec::new();
                while self.current_token().kind != TokenKind::RBrace {
                    statements.push(self.parse_statement()?);
                }
                self.expect_and_consume(&TokenKind::RBrace)?;
                Box::new(Expression::Block(BlockStatement { statements }))
            } else {
                Box::new(self.parse_expression(Precedence::Lowest)?)
            };

            arms.push(MatchArm { pattern, body });

            // Optionally consume comma between arms
            self.consume_if_matches(&TokenKind::Comma);
        }

        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(Expression::Match(MatchExpression { scrutinee, arms }))
    }

    fn parse_pattern(&mut self) -> Result<Pattern, CompileError> {
        let token = self.current_token().clone();

        match token.kind {
            // Wildcard pattern: _
            TokenKind::Identifier if token.lexeme == "_" => {
                self.next_token();
                Ok(Pattern::Wildcard)
            }
            // Identifier (variable binding or enum variant)
            TokenKind::Identifier => {
                let first_ident = self.parse_identifier()?;

                // Check for :: (enum variant like Result::Ok)
                if self.consume_if_matches(&TokenKind::DoubleColon) {
                    let variant_name = self.parse_identifier()?;

                    // Check for associated fields
                    let fields = if self.consume_if_matches(&TokenKind::LParen) {
                        let mut field_patterns = Vec::new();
                        while self.current_token().kind != TokenKind::RParen {
                            field_patterns.push(self.parse_pattern()?);
                            if !self.consume_if_matches(&TokenKind::Comma) { break; }
                        }
                        self.expect_and_consume(&TokenKind::RParen)?;
                        Some(field_patterns)
                    } else {
                        None
                    };

                    // Combine enum name and variant name
                    let full_name = Identifier {
                        value: format!("{}::{}", first_ident.value, variant_name.value)
                    };

                    Ok(Pattern::EnumVariant {
                        name: full_name,
                        fields,
                    })
                } else if self.current_token().kind == TokenKind::LParen {
                    // Enum variant without namespace (like Ok(...) or Err(...))
                    self.consume_if_matches(&TokenKind::LParen);
                    let mut field_patterns = Vec::new();
                    while self.current_token().kind != TokenKind::RParen {
                        field_patterns.push(self.parse_pattern()?);
                        if !self.consume_if_matches(&TokenKind::Comma) { break; }
                    }
                    self.expect_and_consume(&TokenKind::RParen)?;

                    Ok(Pattern::EnumVariant {
                        name: first_ident,
                        fields: Some(field_patterns),
                    })
                } else {
                    // Simple identifier binding
                    Ok(Pattern::Identifier(first_ident))
                }
            }
            // Literal patterns
            TokenKind::Integer(_) | TokenKind::Float(_) | TokenKind::String(_) |
            TokenKind::Bool(_) | TokenKind::True | TokenKind::False => {
                let literal_expr = self.parse_expression(Precedence::Lowest)?;
                Ok(Pattern::Literal(literal_expr))
            }
            _ => Err(self.error(&format!("Expected pattern, found {:?}", token.kind)))
        }
    }

    fn parse_infix(&mut self, left: Expression) -> Result<Expression, CompileError> {
        let operator = self.current_token().clone();
        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Ok(Expression::Infix(InfixExpression { left: Box::new(left), operator, right: Box::new(right) }))
    }

    fn parse_jsx_element(&mut self) -> Result<Expression, CompileError> {
        let opening_tag = self.parse_jsx_opening_tag()?;
        let children = if opening_tag.self_closing { vec![] } else { self.parse_jsx_children()? };
        let closing_tag = if opening_tag.self_closing { None } else { Some(self.parse_jsx_closing_tag()?) };
        if !opening_tag.self_closing && opening_tag.name.value != closing_tag.as_ref().unwrap().value {
            return Err(self.error("Mismatched closing tag"));
        }
        Ok(Expression::JsxElement(JsxElement { opening_tag, children, closing_tag }))
    }

    fn parse_jsx_opening_tag(&mut self) -> Result<JsxOpeningTag, CompileError> {
        self.expect_and_consume(&TokenKind::LAngle)?;
        let name = self.parse_identifier()?;
        let mut attributes = vec![];
        while self.current_token().kind != TokenKind::RAngle &&
              self.current_token().kind != TokenKind::Slash &&
              self.current_token().kind != TokenKind::JsxSelfClose {
            attributes.push(self.parse_jsx_attribute()?);
        }
        // Check for self-closing tag />
        let self_closing = if self.consume_if_matches(&TokenKind::JsxSelfClose) {
            // Self-closing tag doesn't enter JSX mode
            true
        } else if self.consume_if_matches(&TokenKind::Slash) {
            // Self-closing with separate / and >
            self.expect_and_consume(&TokenKind::RAngle)?;
            true
        } else {
            // Regular opening tag
            self.expect_and_consume(&TokenKind::RAngle)?;
            false
        };
        Ok(JsxOpeningTag { name, attributes, self_closing })
    }
    
    fn parse_jsx_attribute(&mut self) -> Result<JsxAttribute, CompileError> {
        let name = self.parse_identifier()?;
        self.expect_and_consume(&TokenKind::Assign)?;

        // Check if value is wrapped in curly braces for expression interpolation
        if self.consume_if_matches(&TokenKind::JsxOpenBrace) || self.consume_if_matches(&TokenKind::LBrace) {
            let value = self.parse_expression(Precedence::Lowest)?;
            if !self.consume_if_matches(&TokenKind::JsxCloseBrace) {
                self.expect_and_consume(&TokenKind::RBrace)?;
            }
            Ok(JsxAttribute { name, value })
        } else {
            // Parse just the prefix (string literal, identifier, etc.) without infix operators
            // This prevents treating > or / as operators
            let value = self.parse_prefix()?;
            Ok(JsxAttribute { name, value })
        }
    }

    fn parse_jsx_children(&mut self) -> Result<Vec<JsxChild>, CompileError> {
        let mut children = Vec::new();

        loop {
            // Check if we've reached the closing tag
            if self.current_token().kind == TokenKind::LAngle && self.peek_token().kind == TokenKind::Slash {
                break;
            }

            if self.current_token().kind == TokenKind::Eof {
                return Err(self.error("Unclosed JSX element"));
            }

            // Check for JSX text - any content that's not a tag or expression
            // This handles string literals and potentially other tokens as text
            match &self.current_token().kind {
                TokenKind::JsxText(text) => {
                    if !text.is_empty() {
                        children.push(JsxChild::Text(text.clone()));
                    }
                    self.next_token();
                    continue;
                }
                TokenKind::String(text) => {
                    // String literals in JSX are treated as text
                    children.push(JsxChild::Text(text.clone()));
                    self.next_token();
                    continue;
                }
                TokenKind::Identifier | TokenKind::Bang | TokenKind::Comma => {
                    // Various tokens in JSX children position are text content
                    // This handles words, punctuation, etc.
                    let text = self.current_token().lexeme.clone();
                    children.push(JsxChild::Text(text));
                    self.next_token();
                    continue;
                }
                _ => {}
            }

            // Check for {expr} interpolation with JSX-aware braces
            if self.consume_if_matches(&TokenKind::JsxOpenBrace) || self.consume_if_matches(&TokenKind::LBrace) {
                let expr = self.parse_expression(Precedence::Lowest)?;
                if !self.consume_if_matches(&TokenKind::JsxCloseBrace) {
                    self.expect_and_consume(&TokenKind::RBrace)?;
                }
                children.push(JsxChild::Expression(Box::new(expr)));
                continue;
            }

            // Check for nested JSX element (starts with <, but not </)
            if self.current_token().kind == TokenKind::LAngle {
                // Double-check it's not a closing tag
                if self.peek_token().kind == TokenKind::Slash {
                    break; // This is actually the closing tag, exit
                }
                // This is a nested element
                let child_expr = self.parse_jsx_element()?;
                if let Expression::JsxElement(el) = child_expr {
                    children.push(JsxChild::Element(Box::new(el)));
                }
                continue;
            }

            // If we get here and haven't matched anything, break
            break;
        }

        Ok(children)
    }

    fn parse_jsx_closing_tag(&mut self) -> Result<Identifier, CompileError> {
        self.expect_and_consume(&TokenKind::LAngle)?;
        self.expect_and_consume(&TokenKind::Slash)?;
        let name = self.parse_identifier()?;
        self.expect_and_consume(&TokenKind::RAngle)?;
        // Note: JSX mode is already exited in parse_jsx_children before we get here
        Ok(name)
    }

    fn parse_identifier(&mut self) -> Result<Identifier, CompileError> {
        let token = self.current_token();
        if let TokenKind::Identifier = &token.kind {
            let ident = Identifier { value: token.lexeme.clone() };
            self.next_token();
            Ok(ident)
        } else {
            Err(self.error(&format!("Expected Identifier, found {:?}", token.kind)))
        }
    }

    fn current_token(&self) -> &Token { &self.current }
    fn peek_token(&self) -> &Token { &self.peek }
    fn current_precedence(&self) -> Precedence { PRECEDENCES.get(&self.current_token().kind).cloned().unwrap_or(Precedence::Lowest) }
    fn next_token(&mut self) {
        self.current = self.peek.clone();
        self.peek = self.lexer.next_token();
    }

    // Re-fetch the peek token after changing lexer state (e.g., jsx_mode)
    #[allow(dead_code)] // For future JSX mode switching
    fn refresh_peek(&mut self) {
        self.peek = self.lexer.next_token();
    }

    fn expect_and_consume(&mut self, expected: &TokenKind) -> Result<(), CompileError> {
        if &self.current_token().kind == expected {
            self.next_token();
            Ok(())
        } else {
            Err(self.error(&format!("Expected {:?}, found {:?}", expected, self.current_token().kind)))
        }
    }

    fn consume_if_matches(&mut self, kind: &TokenKind) -> bool {
        if &self.current_token().kind == kind {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn error(&self, message: &str) -> CompileError {
        let t = self.current_token();
        CompileError::ParserError { message: message.to_string(), line: t.line, column: t.column }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_expr(source: &str) -> Result<Expression, CompileError> {
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(&mut lexer);
        parser.parse_expression(Precedence::Lowest)
    }

    #[test]
    fn test_jsx_empty_element() {
        let expr = parse_expr("<div></div>").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.opening_tag.name.value, "div");
                assert_eq!(jsx.children.len(), 0);
                assert_eq!(jsx.closing_tag.as_ref().unwrap().value, "div");
                assert!(!jsx.opening_tag.self_closing);
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_self_closing() {
        let expr = parse_expr("<img />").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.opening_tag.name.value, "img");
                assert_eq!(jsx.children.len(), 0);
                assert!(jsx.closing_tag.is_none());
                assert!(jsx.opening_tag.self_closing);
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_with_text() {
        let expr = parse_expr("<div>Hello World</div>").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                // Text may be split into multiple tokens
                assert!(jsx.children.len() >= 1);
                // Verify we have text children
                for child in &jsx.children {
                    match child {
                        JsxChild::Text(_) => {},
                        _ => panic!("Expected text child"),
                    }
                }
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_with_single_attribute() {
        let expr = parse_expr(r#"<div class="container"></div>"#).unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.opening_tag.attributes.len(), 1);
                let attr = &jsx.opening_tag.attributes[0];
                assert_eq!(attr.name.value, "class");
                match &attr.value {
                    Expression::StringLiteral(s) => assert_eq!(s, "container"),
                    _ => panic!("Expected string literal"),
                }
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_with_multiple_attributes() {
        let expr = parse_expr(r#"<div class="container" id="app"></div>"#).unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.opening_tag.attributes.len(), 2);
                assert_eq!(jsx.opening_tag.attributes[0].name.value, "class");
                assert_eq!(jsx.opening_tag.attributes[1].name.value, "id");
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_self_closing_with_attribute() {
        let expr = parse_expr(r#"<img src="photo.jpg" />"#).unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert!(jsx.opening_tag.self_closing);
                assert_eq!(jsx.opening_tag.attributes.len(), 1);
                assert_eq!(jsx.opening_tag.attributes[0].name.value, "src");
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_nested_element() {
        let expr = parse_expr("<div><span>Hello</span></div>").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.children.len(), 1);
                match &jsx.children[0] {
                    JsxChild::Element(child) => {
                        assert_eq!(child.opening_tag.name.value, "span");
                        assert_eq!(child.children.len(), 1);
                    }
                    _ => panic!("Expected element child"),
                }
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_with_expression() {
        let expr = parse_expr("<div>Hello {name}!</div>").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                // Should have at least text, expression, text
                assert!(jsx.children.len() >= 3);
                // Find the expression child
                let has_expr = jsx.children.iter().any(|child| {
                    matches!(child, JsxChild::Expression(e) if matches!(e.as_ref(), Expression::Identifier(id) if id.value == "name"))
                });
                assert!(has_expr, "Should contain expression with identifier 'name'");
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_multiple_children() {
        let expr = parse_expr("<div><span>A</span><span>B</span></div>").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.children.len(), 2);
                match (&jsx.children[0], &jsx.children[1]) {
                    (JsxChild::Element(a), JsxChild::Element(b)) => {
                        assert_eq!(a.opening_tag.name.value, "span");
                        assert_eq!(b.opening_tag.name.value, "span");
                    }
                    _ => panic!("Expected element children"),
                }
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_mismatched_tags() {
        let result = parse_expr("<div></span>");
        assert!(result.is_err());
        if let Err(CompileError::ParserError { message, .. }) = result {
            assert!(message.contains("Mismatched"));
        }
    }

    #[test]
    fn test_jsx_deeply_nested() {
        let expr = parse_expr("<div><section><article>Text</article></section></div>").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.opening_tag.name.value, "div");
                match &jsx.children[0] {
                    JsxChild::Element(section) => {
                        assert_eq!(section.opening_tag.name.value, "section");
                        match &section.children[0] {
                            JsxChild::Element(article) => {
                                assert_eq!(article.opening_tag.name.value, "article");
                            }
                            _ => panic!("Expected element"),
                        }
                    }
                    _ => panic!("Expected element"),
                }
            }
            _ => panic!("Expected JsxElement"),
        }
    }
}