use crate::lexer::tokens::Token;
use crate::parser::enums::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Void,
    Unknown, // For error recovery
}

impl Type {
    fn from_string(s: &str) -> Self {
        match s {
            "int" => Type::Int,
            "float" => Type::Float,
            "bool" => Type::Bool,
            "string" => Type::String,
            "void" => Type::Void,
            _ => Type::Unknown,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::Bool => "bool".to_string(),
            Type::String => "string".to_string(),
            Type::Void => "void".to_string(),
            Type::Unknown => "unknown".to_string(),
        }
    }

    // Check if two types are compatible for operations
    fn is_compatible(&self, other: &Type) -> bool {
        self == other
            || matches!(
                (self, other),
                (Type::Int, Type::Float) | (Type::Float, Type::Int)
            )
    }

    // Get the result type of a binary operation
    fn result_type(&self, other: &Type) -> Type {
        match (self, other) {
            (Type::Float, Type::Int) | (Type::Int, Type::Float) | (Type::Float, Type::Float) => {
                Type::Float
            }
            (Type::Int, Type::Int) => Type::Int,
            (Type::String, Type::String) => Type::String,
            (Type::Bool, Type::Bool) => Type::Bool,
            _ => Type::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SymbolType {
    Variable(String), //Store type, will be useful for type checking later
    Function {
        return_type: String,
        params: Vec<String>, // parameter types
    },
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
}

pub struct Scope {
    symbols: HashMap<String, Symbol>, //Map identifiers to its Symbol Information
}

impl Scope {
    fn new() -> Self {
        Scope {
            symbols: HashMap::new(),
        }
    }

    fn declare(&mut self, name: String, symbol: Symbol) -> Result<(), String> {
        // Checking if is redeclaration
        if let Some(existing) = self.symbols.get(&name) {
            // Provide specific error based on what's being redeclared
            match (&existing.symbol_type, &symbol.symbol_type) {
                (SymbolType::Function { .. }, SymbolType::Function { .. }) => {
                    return Err(format!(
                        "Function '{}' is already defined in this scope",
                        name
                    ));
                }
                (SymbolType::Variable(_), SymbolType::Variable(_)) => {
                    return Err(format!(
                        "Variable '{}' is already declared in this scope",
                        name
                    ));
                }
                (SymbolType::Function { .. }, SymbolType::Variable(_)) => {
                    return Err(format!(
                        "'{}' is already defined as a function, cannot redeclare as variable",
                        name
                    ));
                }
                (SymbolType::Variable(_), SymbolType::Function { .. }) => {
                    return Err(format!(
                        "'{}' is already declared as a variable, cannot redefine as function",
                        name
                    ));
                }
            }
        }
        // is a new declaration so adding it to current scope
        self.symbols.insert(name, symbol);
        Ok(())
    }

    fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

pub struct ScopeAnalyzer {
    scopes: Vec<Scope>, //Is Spaghetti stack of scopes
    errors: Vec<String>,
    current_function_return_type: Option<Type>, // Track current function's return type
    loop_depth: usize,                          // Track if we're inside a loop
}

impl ScopeAnalyzer {
    pub fn new() -> Self {
        ScopeAnalyzer {
            scopes: vec![Scope::new()], // 0th index is Global Scope
            errors: Vec::new(),
            current_function_return_type: None,
            loop_depth: 0,
        }
    }

    // Entering a new scope
    fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    // Exiting the current scope
    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    // Helper to declare function in current scope which is the innermost scope
    fn declare_symbol(&mut self, name: String, symbol: Symbol) {
        if let Some(current_scope) = self.scopes.last_mut() {
            if let Err(e) = current_scope.declare(name, symbol) {
                self.errors.push(e);
            }
        }
    }

    // Lookup helper function to iteratively search for symbol from current to all the way to outer global scope
    fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.lookup(name) {
                return Some(symbol);
            }
        }
        None
    }

    // Helper function to convert Token to string type
    fn token_to_string(&self, token: &Token) -> String {
        match token {
            Token::T_INT => "int".to_string(),
            Token::T_FLOAT => "float".to_string(),
            Token::T_BOOL => "bool".to_string(),
            Token::T_STRING => "string".to_string(),
            Token::T_VOID => "void".to_string(),
            _ => format!("{:?}", token),
        }
    }

    // Helper function to convert Token to Type enum
    fn token_to_type(&self, token: &Token) -> Type {
        match token {
            Token::T_INT => Type::Int,
            Token::T_FLOAT => Type::Float,
            Token::T_BOOL => Type::Bool,
            Token::T_STRING => Type::String,
            Token::T_VOID => Type::Void,
            _ => Type::Unknown,
        }
    }

    // Type inference for expressions
    fn infer_expression_type(&mut self, expr: &Expression) -> Type {
        match expr {
            Expression::Literal(constant) => match constant {
                Constants::Int(_) => Type::Int,
                Constants::Float(_) => Type::Float,
                Constants::Str(_) => Type::String,
                Constants::Bool(_) => Type::Bool,
            },
            Expression::Identifier(name) => {
                if let Some(symbol) = self.lookup_symbol(name) {
                    match &symbol.symbol_type {
                        SymbolType::Variable(type_str) => Type::from_string(type_str),
                        SymbolType::Function { .. } => {
                            // Already reported as error in analyze_expression
                            Type::Unknown
                        }
                    }
                } else {
                    // Already reported as error in analyze_expression
                    Type::Unknown
                }
            }
            Expression::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let left_type = self.infer_expression_type(left);
                let right_type = self.infer_expression_type(right);

                match operator {
                    // Arithmetic operators: +, -, *, /, ^
                    Token::T_PLUS_OPR
                    | Token::T_MINUS_OPR
                    | Token::T_MULTIPLY_OPR
                    | Token::T_DIVIDE_OPR
                    | Token::T_EXPONENT_OPR => {
                        if left_type.is_compatible(&right_type) {
                            left_type.result_type(&right_type)
                        } else {
                            self.errors.push(format!(
                                "Type mismatch in arithmetic operation: cannot apply operator to '{}' and '{}'",
                                left_type.to_string(), right_type.to_string()
                            ));
                            Type::Unknown
                        }
                    }
                    // Comparison operators: ==, !=, <, >, <=, >=
                    Token::T_EQUALS_OPR
                    | Token::T_NOT_EQUALS_OPR
                    | Token::T_LESS_THAN_OPR
                    | Token::T_GREATER_THAN_OPR
                    | Token::T_LESS_THAN_EQUAL_TO_OPR
                    | Token::T_GREATER_THAN_EQUAL_TO_OPR => {
                        if left_type.is_compatible(&right_type) {
                            Type::Bool
                        } else {
                            self.errors.push(format!(
                                "Type mismatch in comparison: cannot compare '{}' and '{}'",
                                left_type.to_string(),
                                right_type.to_string()
                            ));
                            Type::Bool // Return bool for error recovery
                        }
                    }
                    // Logical operators: &&, ||
                    Token::T_AND_OPR | Token::T_OR_OPR => {
                        if left_type != Type::Bool {
                            self.errors.push(format!(
                                "Logical operator requires boolean operands, got '{}' on left side",
                                left_type.to_string()
                            ));
                        }
                        if right_type != Type::Bool {
                            self.errors.push(format!(
                                "Logical operator requires boolean operands, got '{}' on right side",
                                right_type.to_string()
                            ));
                        }
                        Type::Bool
                    }
                    // Bitwise operators: &, |, <<, >>
                    Token::T_LEFT_SHIFT_OPR | Token::T_RIGHT_SHIFT_OPR => {
                        if left_type != Type::Int {
                            self.errors.push(format!(
                                "Bitwise shift operator requires integer operands, got '{}'",
                                left_type.to_string()
                            ));
                        }
                        if right_type != Type::Int {
                            self.errors.push(format!(
                                "Bitwise shift operator requires integer operands, got '{}'",
                                right_type.to_string()
                            ));
                        }
                        Type::Int
                    }
                    _ => Type::Unknown,
                }
            }
            Expression::UnaryOperation {
                operator,
                expression,
            } => {
                let expr_type = self.infer_expression_type(expression);
                match operator {
                    Token::T_MINUS_OPR => {
                        if expr_type == Type::Int || expr_type == Type::Float {
                            expr_type
                        } else {
                            self.errors.push(format!(
                                "Unary minus requires numeric type, got '{}'",
                                expr_type.to_string()
                            ));
                            Type::Unknown
                        }
                    }
                    Token::T_NOT => {
                        if expr_type != Type::Bool {
                            self.errors.push(format!(
                                "Logical NOT requires boolean type, got '{}'",
                                expr_type.to_string()
                            ));
                        }
                        Type::Bool
                    }
                    _ => Type::Unknown,
                }
            }
            Expression::Assignment { left, right } => {
                let left_type = self.infer_expression_type(left);
                let right_type = self.infer_expression_type(right);

                if !left_type.is_compatible(&right_type) {
                    self.errors.push(format!(
                        "Type mismatch in assignment: cannot assign '{}' to '{}'",
                        right_type.to_string(),
                        left_type.to_string()
                    ));
                }
                left_type
            }
            Expression::FunctionCall(func_call) => {
                if let Some(symbol) = self.lookup_symbol(&func_call.identifier) {
                    match &symbol.symbol_type {
                        SymbolType::Function { return_type, .. } => Type::from_string(return_type),
                        _ => Type::Unknown,
                    }
                } else {
                    Type::Unknown
                }
            }
        }
    }

    // AST parsing functions

    // Main function starting from root list
    pub fn analyze(&mut self, root_list: &RootList) {
        for root in root_list {
            self.analyze_root(root);
        }
    }

    fn analyze_root(&mut self, root: &Root) {
        match root {
            Root::Var(var_decl) => {
                // Checking initializing expression
                self.analyze_expression(&var_decl.expression);

                // Type checking: check if expression type matches variable type
                let declared_type = self.token_to_type(&var_decl.type_token);
                let expr_type = self.infer_expression_type(&var_decl.expression);

                if !declared_type.is_compatible(&expr_type) {
                    self.errors.push(format!(
                        "Type mismatch in variable declaration '{}': expected '{}', got '{}'",
                        var_decl.identifier,
                        declared_type.to_string(),
                        expr_type.to_string()
                    ));
                }

                // Add variable to scope , will handle redeclaration automatically
                let type_str = self.token_to_string(&var_decl.type_token);
                self.declare_symbol(
                    var_decl.identifier.clone(),
                    Symbol {
                        name: var_decl.identifier.clone(),
                        symbol_type: SymbolType::Variable(type_str),
                    },
                );
            }
            Root::Func(func) => {
                self.analyze_function(func);
            }
        }
    }

    fn analyze_function(&mut self, func: &FunctionStatement) {
        // Declare the function in the current (global) scope, nested functions not supported by syntax
        let return_type = self.token_to_string(&func.return_type);

        let mut param_types: Vec<String> = Vec::new();
        for param in &func.parameters {
            let type_str = self.token_to_string(&param.param_type);
            param_types.push(type_str);
        }

        //Adding declaration of function to current scope
        self.declare_symbol(
            func.identifier.clone(),
            Symbol {
                name: func.identifier.clone(),
                symbol_type: SymbolType::Function {
                    return_type: return_type.clone(),
                    params: param_types,
                },
            },
        );

        // Set current function return type for return statement checking
        let func_return_type = self.token_to_type(&func.return_type);
        self.current_function_return_type = Some(func_return_type.clone());

        // Add function scope and its params
        self.enter_scope();

        // Declaring parameters in function scope
        for param in &func.parameters {
            let param_type = self.token_to_string(&param.param_type);
            self.declare_symbol(
                param.identifier.clone(),
                Symbol {
                    name: param.identifier.clone(),
                    symbol_type: SymbolType::Variable(param_type),
                },
            );
        }

        // Recursively analyze function body and check for return statements
        let has_return = self.block_has_return(&func.block);

        // Check if non-void function has a return statement
        if func_return_type != Type::Void && !has_return {
            self.errors.push(format!(
                "Function '{}' with return type '{}' must have a return statement",
                func.identifier,
                func_return_type.to_string()
            ));
        }

        // Pop function scope
        self.exit_scope();

        // Clear current function return type
        self.current_function_return_type = None;
    }

    fn analyze_block(&mut self, block: &Block) {
        for statement in &block.statements {
            self.analyze_statement(statement);
        }
    }

    // Helper to check if a block contains a return statement
    fn block_has_return(&mut self, block: &Block) -> bool {
        self.analyze_block(block);
        block
            .statements
            .iter()
            .any(|stmt| self.statement_has_return(stmt))
    }

    // Helper to check if a statement contains a return
    fn statement_has_return(&self, statement: &Statement) -> bool {
        match statement {
            Statement::Return(_) => true,
            Statement::If(if_stmt) => {
                // An if statement guarantees a return only if all branches return
                let if_returns = if_stmt
                    .block
                    .statements
                    .iter()
                    .any(|s| self.statement_has_return(s));

                let all_elif_return = if_stmt.elif_blocks.iter().all(|elif| {
                    elif.block
                        .statements
                        .iter()
                        .any(|s| self.statement_has_return(s))
                });

                let else_returns = if let Some(else_block) = &if_stmt.else_block {
                    else_block
                        .statements
                        .iter()
                        .any(|s| self.statement_has_return(s))
                } else {
                    false
                };

                // Only if we have if, all elifs, and else all returning
                if_returns && all_elif_return && else_returns && if_stmt.else_block.is_some()
            }
            _ => false, // while and for loops do not guarantee return (never run), so error on them
        }
    }

    fn analyze_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VarDecl(var_decl) => {
                self.analyze_expression(&var_decl.expression);

                // Type checking: check if expression type matches variable type
                let declared_type = self.token_to_type(&var_decl.type_token);
                let expr_type = self.infer_expression_type(&var_decl.expression);

                if !declared_type.is_compatible(&expr_type) {
                    self.errors.push(format!(
                        "Type mismatch in variable declaration '{}': expected '{}', got '{}'",
                        var_decl.identifier,
                        declared_type.to_string(),
                        expr_type.to_string()
                    ));
                }

                let type_str = self.token_to_string(&var_decl.type_token);
                self.declare_symbol(
                    var_decl.identifier.clone(),
                    Symbol {
                        name: var_decl.identifier.clone(),
                        symbol_type: SymbolType::Variable(type_str),
                    },
                );
            }
            Statement::Expr(expr) => {
                self.analyze_expression(expr);
            }
            Statement::Return(expr) => {
                self.analyze_expression(expr);

                // Type check return statement
                let return_type = self.infer_expression_type(expr);
                if let Some(expected_type) = &self.current_function_return_type {
                    if !expected_type.is_compatible(&return_type) {
                        self.errors.push(format!(
                            "Type mismatch in return statement: expected '{}', got '{}'",
                            expected_type.to_string(),
                            return_type.to_string()
                        ));
                    }
                }
            }
            Statement::Break => {
                if self.loop_depth == 0 {
                    self.errors
                        .push("'break' statement must be inside a loop".to_string());
                }
            }
            Statement::Continue => {
                if self.loop_depth == 0 {
                    self.errors
                        .push("'continue' statement must be inside a loop".to_string());
                }
            }
            Statement::If(if_stmt) => {
                self.analyze_if_statement(if_stmt);
            }
            Statement::While(while_stmt) => {
                self.analyze_while_statement(while_stmt);
            }
            Statement::For(for_stmt) => {
                self.analyze_for_statement(for_stmt);
            }
        }
    }

    fn analyze_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal(_) => {
                // No need to check scope for literals
                //TODO: Type checking to be added here later
            }
            Expression::Identifier(name) => {
                if let Some(symbol) = self.lookup_symbol(name) {
                    // Checking if is identifier and not a function
                    if matches!(symbol.symbol_type, SymbolType::Function { .. }) {
                        self.errors
                            .push(format!("'{}' is a function, not a variable", name));
                    }
                } else {
                    self.errors
                        .push(format!("Variable '{}' is not defined", name));
                }
            }
            Expression::BinaryOperation { left, right, .. } => {
                self.analyze_expression(left);
                self.analyze_expression(right);
            }
            Expression::UnaryOperation { expression, .. } => {
                self.analyze_expression(expression);
            }
            Expression::Assignment { left, right } => {
                self.analyze_expression(left);
                self.analyze_expression(right);
            }
            Expression::FunctionCall(func_call) => {
                self.analyze_function_call(func_call);
            }
        }
    }

    fn analyze_function_call(&mut self, func_call: &FunctionCallStatement) {
        // Check declaration and get function signature
        let function_info = if let Some(symbol) = self.lookup_symbol(&func_call.identifier) {
            // Check symbol type
            match &symbol.symbol_type {
                SymbolType::Variable(_) => {
                    self.errors.push(format!(
                        "'{}' is a variable, not a function",
                        func_call.identifier
                    ));
                    None
                }
                SymbolType::Function { params, .. } => Some(params.clone()),
            }
        } else {
            self.errors.push(format!(
                "Function '{}' is not defined",
                func_call.identifier
            ));
            None
        };

        // Checking arguments
        for arg in &func_call.args {
            self.analyze_expression(arg);
        }

        // Type check function arguments if we have function info
        if let Some(params) = function_info {
            if func_call.args.len() != params.len() {
                self.errors.push(format!(
                    "Function '{}' expects {} argument(s), got {}",
                    func_call.identifier,
                    params.len(),
                    func_call.args.len()
                ));
            } else {
                // Check each argument type
                for (i, (arg, expected_type_str)) in
                    func_call.args.iter().zip(params.iter()).enumerate()
                {
                    let arg_type = self.infer_expression_type(arg);
                    let expected_type = Type::from_string(expected_type_str);

                    if !expected_type.is_compatible(&arg_type) {
                        self.errors.push(format!(
                            "Type mismatch in argument {} of function '{}': expected '{}', got '{}'",
                            i + 1,
                            func_call.identifier,
                            expected_type.to_string(),
                            arg_type.to_string()
                        ));
                    }
                }
            }
        }
    }

    fn analyze_if_statement(&mut self, if_stmt: &IfStatement) {
        // Condition expression check if (....)
        self.analyze_expression(&if_stmt.condition);

        // Type check: condition must be boolean
        let condition_type = self.infer_expression_type(&if_stmt.condition);
        if condition_type != Type::Bool && condition_type != Type::Unknown {
            self.errors.push(format!(
                "If statement condition must be boolean, got '{}'",
                condition_type.to_string()
            ));
        }

        //checking if block {}
        self.enter_scope();
        self.analyze_block(&if_stmt.block);
        self.exit_scope();

        // elifBlock, can be none or many
        for elif in &if_stmt.elif_blocks {
            self.analyze_expression(&elif.condition);

            // Type check: elif condition must be boolean
            let elif_condition_type = self.infer_expression_type(&elif.condition);
            if elif_condition_type != Type::Bool && elif_condition_type != Type::Unknown {
                self.errors.push(format!(
                    "Elif statement condition must be boolean, got '{}'",
                    elif_condition_type.to_string()
                ));
            }

            self.enter_scope();
            self.analyze_block(&elif.block);
            self.exit_scope();
        }

        // else block
        if let Some(else_block) = &if_stmt.else_block {
            self.enter_scope();
            self.analyze_block(else_block);
            self.exit_scope();
        }
    }

    fn analyze_while_statement(&mut self, while_stmt: &WhileStatement) {
        self.analyze_expression(&while_stmt.condition);

        // Type check: while condition must be boolean
        let condition_type = self.infer_expression_type(&while_stmt.condition);
        if condition_type != Type::Bool && condition_type != Type::Unknown {
            self.errors.push(format!(
                "While statement condition must be boolean, got '{}'",
                condition_type.to_string()
            ));
        }

        self.enter_scope();
        self.loop_depth += 1;
        self.analyze_block(&while_stmt.block);
        self.loop_depth -= 1;
        self.exit_scope();
    }

    fn analyze_for_statement(&mut self, for_stmt: &ForStatement) {
        // makes its own scope variables in parenthesis
        self.enter_scope();

        // checking initialization if exists, for (int i= 0)
        if let Some(init_var) = &for_stmt.init_var {
            self.analyze_expression(&init_var.expression);

            // Type check initialization
            let declared_type = self.token_to_type(&init_var.type_token);
            let expr_type = self.infer_expression_type(&init_var.expression);

            if !declared_type.is_compatible(&expr_type) {
                self.errors.push(format!(
                    "Type mismatch in for loop initialization '{}': expected '{}', got '{}'",
                    init_var.identifier,
                    declared_type.to_string(),
                    expr_type.to_string()
                ));
            }

            let type_str = self.token_to_string(&init_var.type_token);
            self.declare_symbol(
                init_var.identifier.clone(),
                Symbol {
                    name: init_var.identifier.clone(),
                    symbol_type: SymbolType::Variable(type_str),
                },
            );
        }

        // Checking condition if exists
        if let Some(condition) = &for_stmt.condition {
            self.analyze_expression(condition);

            // Type check: for condition must be boolean
            let condition_type = self.infer_expression_type(condition);
            if condition_type != Type::Bool && condition_type != Type::Unknown {
                self.errors.push(format!(
                    "For loop condition must be boolean, got '{}'",
                    condition_type.to_string()
                ));
            }
        }

        // Checking update if exists
        if let Some(update) = &for_stmt.update {
            self.analyze_expression(update);
        }

        // Analyze block
        self.loop_depth += 1;
        self.analyze_block(&for_stmt.block);
        self.loop_depth -= 1;

        self.exit_scope();
    }

    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

// Main
pub fn semantic_analysis(ast: RootList) -> Result<(), Vec<String>> {
    let mut analyzer = ScopeAnalyzer::new();
    analyzer.analyze(&ast);

    if analyzer.is_valid() {
        println!("Scope and type analysis passed!");
        Ok(())
    } else {
        println!(
            "Scope and type analysis failed with {} error(s):",
            analyzer.get_errors().len()
        );
        for (i, error) in analyzer.get_errors().iter().enumerate() {
            println!("  {}. {}", i + 1, error);
        }
        Err(analyzer.errors)
    }
}
