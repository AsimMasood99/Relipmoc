use crate::parser::enums::{*};
use crate::lexer::tokens::Token;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SymbolType 
{
    Variable(String), //Store type, will be useful for type checking later
    Function 
    {
        return_type: String,
        params: Vec<String>, // parameter types
    },
}

#[derive(Debug, Clone)]
pub struct Symbol 
{
    pub name: String,
    pub symbol_type: SymbolType,
}

pub struct Scope 
{
    symbols: HashMap<String, Symbol>, //Map identifiers to its Symbol Information
}

impl Scope 
{
    fn new() -> Self 
    {
        Scope 
        {
            symbols: HashMap::new(),
        }
    }

    fn declare(&mut self, name: String, symbol: Symbol) -> Result<(), String> 
    {
        // Checking if is redeclaration
        if let Some(existing) = self.symbols.get(&name) 
        {
            // Provide specific error based on what's being redeclared
            match (&existing.symbol_type, &symbol.symbol_type) 
            {
                (SymbolType::Function { .. }, SymbolType::Function { .. }) => 
                {
                    return Err(format!("Function '{}' is already defined in this scope", name));
                }
                (SymbolType::Variable(_), SymbolType::Variable(_)) => 
                {
                    return Err(format!("Variable '{}' is already declared in this scope", name));
                }
                (SymbolType::Function { .. }, SymbolType::Variable(_)) => 
                {
                    return Err(format!("'{}' is already defined as a function, cannot redeclare as variable", name));
                }
                (SymbolType::Variable(_), SymbolType::Function { .. }) => 
                {
                    return Err(format!("'{}' is already declared as a variable, cannot redefine as function", name));
                }
            }
        }
        // is a new declaration so adding it to current scope
        self.symbols.insert(name, symbol);
        Ok(())
    }

    fn lookup(&self, name: &str) -> Option<&Symbol> 
    {
        self.symbols.get(name)
    }
}

pub struct ScopeAnalyzer 
{
    scopes: Vec<Scope>, //Is Spaghetti stack of scopes
    errors: Vec<String>,
}

impl ScopeAnalyzer 
{
    pub fn new() -> Self 
    {
        ScopeAnalyzer 
        {
            scopes: vec![Scope::new()], // 0th index is Global Scope
            errors: Vec::new(),
        }
    }

    // Entering a new scope
    fn enter_scope(&mut self) 
    {
        self.scopes.push(Scope::new());
    }

    // Exiting the current scope
    fn exit_scope(&mut self) 
    {
        self.scopes.pop();
    }

    // Helper to declare function in current scope which is the innermost scope
    fn declare_symbol(&mut self, name: String, symbol: Symbol) 
    {
        if let Some(current_scope) = self.scopes.last_mut() 
        {
            if let Err(e) = current_scope.declare(name, symbol) 
            {
                self.errors.push(e);
            }
        }
    }

    // Lookup helper function to iteratively search for symbol from current to all the way to outer global scope
    fn lookup_symbol(&self, name: &str) -> Option<&Symbol> 
    {
        for scope in self.scopes.iter().rev() 
        {
            if let Some(symbol) = scope.lookup(name) 
            {
                return Some(symbol);
            }
        }
        None
    }

    // Helper function to convert Token to string type
    fn token_to_string(&self, token: &Token) -> String 
    {
        match token 
        {
            Token::T_INT => "int".to_string(),
            Token::T_FLOAT => "float".to_string(),
            Token::T_BOOL => "bool".to_string(),
            Token::T_STRING => "string".to_string(),
            Token::T_VOID => "void".to_string(),
            _ => format!("{:?}", token),
        }
    }

// AST parsing functions

    // Main function starting from root list 
    pub fn analyze(&mut self, root_list: &RootList) 
    {
        for root in root_list 
        {
            self.analyze_root(root);
        }
    }

    fn analyze_root(&mut self, root: &Root) 
    {
        match root 
        {
            Root::Var(var_decl) => 
            {
                // Checking initializing expression
                self.analyze_expression(&var_decl.expression);

                // Add variable to scope , will handle redeclaration automatically
                let type_str = self.token_to_string(&var_decl.type_token);
                self.declare_symbol(
                    var_decl.identifier.clone(),
                    Symbol 
                    {
                        name: var_decl.identifier.clone(),
                        symbol_type: SymbolType::Variable(type_str),
                    },
                );
            }
            Root::Func(func) => 
            {
                self.analyze_function(func);
            }
        }
    }

    fn analyze_function(&mut self, func: &FunctionStatement) 
    {
        // Declare the function in the current (global) scope, nested functions not supported by syntax
        let return_type = self.token_to_string(&func.return_type);

        let mut param_types: Vec<String> = Vec::new();
        for param in &func.parameters 
        {
            let type_str = self.token_to_string(&param.param_type);
            param_types.push(type_str);
        }

        //Adding declaration of function to current scope
        self.declare_symbol(
            func.identifier.clone(),
            Symbol 
            {
                name: func.identifier.clone(),
                symbol_type: SymbolType::Function 
                {
                    return_type,
                    params: param_types,
                },
            },
        );

        // Add function scope and its params
        self.enter_scope();

        // Declaring parameters in function scope
        for param in &func.parameters 
        {
            let param_type = self.token_to_string(&param.param_type);
            self.declare_symbol(
                param.identifier.clone(),
                Symbol 
                {
                    name: param.identifier.clone(),
                    symbol_type: SymbolType::Variable(param_type),
                },
            );
        }

        // Recursively analyze function body
        self.analyze_block(&func.block);

        // Pop function scope
        self.exit_scope();
    }

    fn analyze_block(&mut self, block: &Block) 
    {
        for statement in &block.statements
        {
            self.analyze_statement(statement);
        }
    }

    fn analyze_statement(&mut self, statement: &Statement) 
    {
        match statement 
        {
            Statement::VarDecl(var_decl) => 
            {
                self.analyze_expression(&var_decl.expression);
                
                let type_str = self.token_to_string(&var_decl.type_token);
                self.declare_symbol(
                    var_decl.identifier.clone(),
                    Symbol 
                    {
                        name: var_decl.identifier.clone(),
                        symbol_type: SymbolType::Variable(type_str),
                    },
                );
            }
            Statement::Expr(expr) => 
            {
                self.analyze_expression(expr);
            }
            Statement::Return(expr) => 
            {
                self.analyze_expression(expr);
            }
            Statement::If(if_stmt) => 
            {
                self.analyze_if_statement(if_stmt);
            }
            Statement::While(while_stmt) => 
            {
                self.analyze_while_statement(while_stmt);
            }
            Statement::For(for_stmt) => 
            {
                self.analyze_for_statement(for_stmt);
            }
        }
    }

    fn analyze_expression(&mut self, expr: &Expression) 
    {
        match expr {
            Expression::Literal(_) => 
            {
                // No need to check scope for literals
                //TODO: Type checking to be added here later
            }
            Expression::Identifier(name) => 
            {
                if let Some(symbol) = self.lookup_symbol(name) 
                {
                    // Checking if is identifier and not a function
                    if matches!(symbol.symbol_type, SymbolType::Function { .. }) 
                    {
                        self.errors.push(format!("'{}' is a function, not a variable", name));
                    }
                } 
                else 
                {
                    self.errors.push(format!("Variable '{}' is not defined", name));
                }
            }
            Expression::BinaryOperation 
            { 
                left, right, .. 
            } => 
            {
                self.analyze_expression(left);
                self.analyze_expression(right);
            }
            Expression::UnaryOperation 
            { 
                expression, .. 
            } => 
            {
                self.analyze_expression(expression);
            }
            Expression::Assignment 
            { 
                left, right 
            } => 
            {
                self.analyze_expression(left);
                self.analyze_expression(right);
            }
            Expression::FunctionCall(func_call) 
            => 
            {
                self.analyze_function_call(func_call);
            }
        }
    }

    fn analyze_function_call(&mut self, func_call: &FunctionCallStatement) 
    {
        
        // Check declaration
        if let Some(symbol) = self.lookup_symbol(&func_call.identifier) 
        {
            // Check symbol type
            if matches!(symbol.symbol_type, SymbolType::Variable(_)) 
            {
                self.errors.push(format!("'{}' is a variable, not a function", func_call.identifier));
            }
        } 
        else 
        {
            self.errors.push(format!("Function '{}' is not defined", func_call.identifier));
        }

        // Checking arguments
        for arg in &func_call.args 
        {
            self.analyze_expression(arg);
        }
    }

    fn analyze_if_statement(&mut self, if_stmt: &IfStatement) 
    {
        // Condition expression check if (....)
        self.analyze_expression(&if_stmt.condition);

        //checking if block {}
        self.enter_scope();
        self.analyze_block(&if_stmt.block);
        self.exit_scope();

        // elifBlock, can be none or many
        for elif in &if_stmt.elif_blocks 
        {
            self.analyze_expression(&elif.condition);
            self.enter_scope();
            self.analyze_block(&elif.block);
            self.exit_scope();
        }

        // else block
        if let Some(else_block) = &if_stmt.else_block 
        {
            self.enter_scope();
            self.analyze_block(else_block);
            self.exit_scope();
        }
    }

    fn analyze_while_statement(&mut self, while_stmt: &WhileStatement) 
    {
        self.analyze_expression(&while_stmt.condition);

        self.enter_scope();
        self.analyze_block(&while_stmt.block);
        self.exit_scope();
    }

    fn analyze_for_statement(&mut self, for_stmt: &ForStatement) 
    {
        // makes its own scope variables in parenthesis
        self.enter_scope();

        // checking initialization if exists, for (int i= 0)
        if let Some(init_var) = &for_stmt.init_var 
        {
            self.analyze_expression(&init_var.expression);
            let type_str = self.token_to_string(&init_var.type_token);
            self.declare_symbol(
                init_var.identifier.clone(),
                Symbol 
                {
                    name: init_var.identifier.clone(),
                    symbol_type: SymbolType::Variable(type_str),
                },
            );
        }

        // Checking condition if exists
        if let Some(condition) = &for_stmt.condition 
        {
            self.analyze_expression(condition);
        }

        // Checking update if exists
        if let Some(update) = &for_stmt.update 
        {
            self.analyze_expression(update);
        }

        // Analyze block
        self.analyze_block(&for_stmt.block);

        self.exit_scope();
    }

    pub fn get_errors(&self) -> &[String] 
    {
        &self.errors
    }

    pub fn is_valid(&self) -> bool 
    {
        self.errors.is_empty()
    }
}

// Main
pub fn scope_checking(ast: RootList) -> Result<(), Vec<String>> 
{
    let mut analyzer = ScopeAnalyzer::new();
    analyzer.analyze(&ast);
    
    if analyzer.is_valid() 
    {
        println!("Scope analysis passed!");
        Ok(())
    } 
    else 
    {
        println!("Scope analysis failed with {} error(s):", analyzer.get_errors().len());
        for (i, error) in analyzer.get_errors().iter().enumerate() 
        {
            println!("  {}. {}", i + 1, error);
        }
        Err(analyzer.errors)
    }
}