// Reactive Analyzer - Detects signal/computed reads in expressions
//
// This module walks the AST to identify expressions that access reactive values
// (signals and computeds) via `.value` property access. These expressions need
// to be wrapped in effects for automatic DOM updates.
//
// Example:
//   todos.value.length  → Reactive (reads todos.value)
//   count.value * 2     → Reactive (reads count.value)
//   "Hello"             → Not reactive (static string)
//   foo()               → Might be reactive (depends on function body)

use crate::ast::{Expression, JsxChild, JsxElement, JsxAttribute, ObjectProperty, TemplatePart};

/// Analyzes expressions to detect reactive value access
pub struct ReactiveAnalyzer;

impl ReactiveAnalyzer {
    /// Check if an expression contains any reactive value reads (`.value` access)
    ///
    /// Returns true if the expression accesses `.value` on any identifier,
    /// indicating it's reading a signal or computed value.
    pub fn is_reactive(expr: &Expression) -> bool {
        match expr {
            // Direct .value access: signal.value, count.value, etc.
            Expression::FieldAccess(field) => {
                // Check if this is a `.value` access
                if field.field.value == "value" {
                    return true;
                }
                // Recursively check the object (e.g., foo.bar.value)
                Self::is_reactive(&field.object)
            }

            // Optional chaining .value access: signal?.value
            Expression::OptionalChaining(opt) => {
                // Check if this is a `.value` access
                if opt.field.value == "value" {
                    return true;
                }
                // Recursively check the object
                Self::is_reactive(&opt.object)
            }

            // Binary operations: a.value + b.value
            Expression::Infix(infix) => {
                Self::is_reactive(&infix.left) || Self::is_reactive(&infix.right)
            }

            // Assignment expressions: x.value = 5 or obj.value = newValue
            Expression::Assignment(assignment) => {
                Self::is_reactive(&assignment.target) || Self::is_reactive(&assignment.value)
            }

            // Unary operations: !flag.value
            Expression::Prefix(prefix) => {
                Self::is_reactive(&prefix.right)
            }

            // Postfix operations: count.value++
            Expression::Postfix(postfix) => {
                Self::is_reactive(&postfix.left)
            }

            // Function calls: might contain reactive args OR be called on reactive values
            // Examples:
            // - bmi.value.toFixed(1) -> function is FieldAccess(.value).toFixed, which is reactive
            // - items.value.map(x => ...) -> function is FieldAccess(.value).map, which is reactive
            Expression::FunctionCall(call) => {
                // Check if the function itself is reactive (method called on .value)
                Self::is_reactive(&call.function) ||
                // Also check if any arguments are reactive
                call.arguments.iter().any(|arg| Self::is_reactive(arg))
            }

            // Array literals: [a.value, b.value]
            Expression::ArrayLiteral(array) => {
                array.elements.iter().any(|elem| Self::is_reactive(elem))
            }

            // Object literals: { count: count.value, ...spread }
            Expression::ObjectLiteral(obj) => {
                obj.properties.iter().any(|prop| {
                    match prop {
                        ObjectProperty::Field(_, value) => Self::is_reactive(value),
                        ObjectProperty::Spread(expr) => Self::is_reactive(expr),
                    }
                })
            }

            // Ternary: cond.value ? a : b
            Expression::Ternary(ternary) => {
                Self::is_reactive(&ternary.condition) ||
                Self::is_reactive(&ternary.true_expr) ||
                Self::is_reactive(&ternary.false_expr)
            }

            // Index access: arr[index.value]
            Expression::IndexAccess(index) => {
                Self::is_reactive(&index.array) || Self::is_reactive(&index.index)
            }

            // Lambda functions: might capture reactive values
            // NOTE: We don't check lambda bodies here because they're
            // already in a closure scope. We only care about immediate reactive access.
            Expression::Lambda(_) => false,

            // Await: await promise.value
            Expression::Await(await_expr) => {
                Self::is_reactive(&await_expr.expression)
            }

            // Match expressions: match signal.value { ... }
            Expression::Match(match_expr) => {
                Self::is_reactive(&match_expr.scrutinee)
            }

            // If expressions: if cond.value { ... }
            Expression::IfExpression(if_expr) => {
                Self::is_reactive(&if_expr.condition)
            }

            // Template literals: check if any expressions are reactive
            Expression::TemplateLiteral(template) => {
                template.parts.iter().any(|part| {
                    match part {
                        TemplatePart::String(_) => false,
                        TemplatePart::Expression(expr) => Self::is_reactive(expr),
                    }
                })
            }

            // Literals and identifiers without .value are not reactive
            Expression::Identifier(_) |
            Expression::IntegerLiteral(_) |
            Expression::FloatLiteral(_) |
            Expression::StringLiteral(_) |
            Expression::CharLiteral(_) |
            Expression::BoolLiteral(_) |
            Expression::UnitLiteral |
            Expression::Spread(_) => false,

            // Composite literals - check recursively
            Expression::TupleLiteral(tuple) => {
                tuple.elements.iter().any(|elem| Self::is_reactive(elem))
            }
            Expression::StructLiteral(struct_lit) => {
                struct_lit.fields.iter().any(|(_, value)| Self::is_reactive(value))
            }
            Expression::ArrayRepeat(repeat) => {
                Self::is_reactive(&repeat.value) || Self::is_reactive(&repeat.count)
            }

            // References and dereferencing - check inner expression
            Expression::Borrow(borrow) => {
                Self::is_reactive(&borrow.expression)
            }
            Expression::MutableBorrow(borrow) => {
                Self::is_reactive(&borrow.expression)
            }
            Expression::Dereference(deref) => {
                Self::is_reactive(&deref.expression)
            }

            // Range expressions
            Expression::Range(range) => {
                range.start.as_ref().map_or(false, |e| Self::is_reactive(e)) ||
                range.end.as_ref().map_or(false, |e| Self::is_reactive(e))
            }

            // Try operator
            Expression::TryOperator(try_op) => {
                Self::is_reactive(&try_op.expression)
            }

            // Type cast
            Expression::TypeCast(cast) => {
                Self::is_reactive(&cast.expression)
            }

            // Block expressions - not reactive (handled separately)
            Expression::Block(_) => false,

            // Macro calls - not reactive (static)
            Expression::MacroCall(_) => false,

            // CSS macro - not reactive (static)
            Expression::CssMacro(_) => false,

            // Reactivity primitives - these CREATE reactive values, but aren't themselves reactive
            Expression::Signal(_) |
            Expression::Computed(_) |
            Expression::Effect(_) |
            Expression::Batch(_) => false,

            // JSX elements - need to check children recursively
            Expression::JsxElement(jsx) => {
                Self::is_jsx_reactive(jsx)
            }

            // Script blocks - treat as potentially reactive
            Expression::ScriptBlock(_) => false,
        }
    }

    /// Check if a JSX element contains any reactive expressions
    pub fn is_jsx_reactive(jsx: &JsxElement) -> bool {
        // Check attributes
        let attrs_reactive = jsx.opening_tag.attributes.iter()
            .any(|attr| Self::is_reactive(&attr.value));

        // Check children
        let children_reactive = jsx.children.iter()
            .any(|child| Self::is_jsx_child_reactive(child));

        attrs_reactive || children_reactive
    }

    /// Check if a JSX child is reactive
    pub fn is_jsx_child_reactive(child: &JsxChild) -> bool {
        match child {
            JsxChild::Element(elem) => Self::is_jsx_reactive(elem),
            JsxChild::Expression(expr) => Self::is_reactive(expr),
            JsxChild::Text(_) => false,
        }
    }

    /// Check if a JSX attribute is reactive
    pub fn is_jsx_attribute_reactive(attr: &JsxAttribute) -> bool {
        Self::is_reactive(&attr.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_field_access_value_is_reactive() {
        // count.value → reactive
        let expr = Expression::FieldAccess(FieldAccessExpression {
            object: Box::new(Expression::Identifier(Identifier {
                value: "count".to_string(),
            })),
            field: Identifier {
                value: "value".to_string(),
            },
        });
        assert!(ReactiveAnalyzer::is_reactive(&expr));
    }

    #[test]
    fn test_field_access_non_value_is_not_reactive() {
        // user.name → not reactive (not .value)
        let expr = Expression::FieldAccess(FieldAccessExpression {
            object: Box::new(Expression::Identifier(Identifier {
                value: "user".to_string(),
            })),
            field: Identifier {
                value: "name".to_string(),
            },
        });
        assert!(!ReactiveAnalyzer::is_reactive(&expr));
    }

    #[test]
    fn test_identifier_is_not_reactive() {
        // count → not reactive (need .value)
        let expr = Expression::Identifier(Identifier {
            value: "count".to_string(),
        });
        assert!(!ReactiveAnalyzer::is_reactive(&expr));
    }

    #[test]
    fn test_string_literal_is_not_reactive() {
        // "hello" → not reactive
        let expr = Expression::StringLiteral("hello".to_string());
        assert!(!ReactiveAnalyzer::is_reactive(&expr));
    }

    #[test]
    fn test_binary_op_with_reactive_left() {
        // count.value + 5 → reactive
        let expr = Expression::Infix(InfixExpression {
            left: Box::new(Expression::FieldAccess(FieldAccessExpression {
                object: Box::new(Expression::Identifier(Identifier {
                    value: "count".to_string(),
                })),
                field: Identifier {
                    value: "value".to_string(),
                },
            })),
            operator: Token::new(TokenKind::Plus, "+".to_string(), 1, 1),
            right: Box::new(Expression::IntegerLiteral(5)),
        });
        assert!(ReactiveAnalyzer::is_reactive(&expr));
    }

    #[test]
    fn test_binary_op_with_reactive_right() {
        // 5 + count.value → reactive
        let expr = Expression::Infix(InfixExpression {
            left: Box::new(Expression::IntegerLiteral(5)),
            operator: Token::new(TokenKind::Plus, "+".to_string(), 1, 1),
            right: Box::new(Expression::FieldAccess(FieldAccessExpression {
                object: Box::new(Expression::Identifier(Identifier {
                    value: "count".to_string(),
                })),
                field: Identifier {
                    value: "value".to_string(),
                },
            })),
        });
        assert!(ReactiveAnalyzer::is_reactive(&expr));
    }

    #[test]
    fn test_binary_op_without_reactive() {
        // 5 + 10 → not reactive
        let expr = Expression::Infix(InfixExpression {
            left: Box::new(Expression::IntegerLiteral(5)),
            operator: Token::new(TokenKind::Plus, "+".to_string(), 1, 1),
            right: Box::new(Expression::IntegerLiteral(10)),
        });
        assert!(!ReactiveAnalyzer::is_reactive(&expr));
    }
}
