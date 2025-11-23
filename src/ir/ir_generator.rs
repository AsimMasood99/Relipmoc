use crate::lexer::tokens::Token;
use crate::parser::enums::{
    Block, Constants, Expression, ForStatement, FunctionStatement, IfStatement, Root, RootList,
    Statement, VariableDeclaration, WhileStatement,
};

pub struct IrGenerator {
    temp_counter: usize,
    label_counter: usize,
    code: Vec<String>,
    loop_stack: Vec<(String, String)>, // (continue_label, break_label)
}

impl IrGenerator {
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
            label_counter: 0,
            code: Vec::new(),
            loop_stack: Vec::new(),
        }
    }

    fn new_temp(&mut self) -> String {
        let temp = format!("t{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }

    fn new_label(&mut self) -> String {
        let label = format!("L{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    fn emit(&mut self, instr: String) {
        self.code.push(instr);
    }

    pub fn generate_ir(&mut self, ast: &RootList) -> Result<String, ()> {
        for root in ast {
            match root {
                Root::Func(func) => self.gen_func(func),
                Root::Var(var) => {
                    self.gen_var_decl(var);
                }
            }
        }
        Ok(self.code.join("\n"))
    }

    fn gen_func(&mut self, func: &FunctionStatement) {
        self.emit(format!("{}:", func.identifier));
        self.emit("BeginFunc".to_string());

        // Parameters
        for param in &func.parameters {
            self.emit(format!("PopParam {}", param.identifier));
        }

        self.gen_block(&func.block);
        self.emit("EndFunc".to_string());
    }

    fn gen_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.gen_stmt(stmt);
        }
    }

    fn gen_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VarDecl(var) => self.gen_var_decl(var),
            Statement::Expr(expr) => {
                self.gen_expr(expr);
            }
            Statement::Return(expr) => {
                let val = self.gen_expr(expr);
                self.emit(format!("Return {}", val));
            }
            Statement::Break => {
                if let Some((_, end_label)) = self.loop_stack.last() {
                    self.emit(format!("Goto {}", end_label));
                }
            }
            Statement::Continue => {
                if let Some((continue_label, _)) = self.loop_stack.last() {
                    self.emit(format!("Goto {}", continue_label));
                }
            }
            Statement::If(if_stmt) => self.gen_if(if_stmt),
            Statement::While(while_stmt) => self.gen_while(while_stmt),
            Statement::For(for_stmt) => self.gen_for(for_stmt),
        }
    }

    fn gen_var_decl(&mut self, var: &VariableDeclaration) {
        let val = self.gen_expr(&var.expression);
        self.emit(format!("{} = {}", var.identifier, val));
    }

    fn gen_expr(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Literal(c) => match c {
                Constants::Int(i) => i.to_string(),
                Constants::Float(f) => f.to_string(),
                Constants::Str(s) => format!("\"{}\"", s),
                Constants::Bool(b) => b.to_string(),
            },
            Expression::Identifier(id) => id.clone(),
            Expression::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let l = self.gen_expr(left);
                let r = self.gen_expr(right);
                let temp = self.new_temp();
                let op_str = self.token_to_op(operator);
                self.emit(format!("{} = {} {} {}", temp, l, op_str, r));
                temp
            }
            Expression::UnaryOperation {
                operator,
                expression,
            } => {
                let e = self.gen_expr(expression);
                let temp = self.new_temp();
                let op_str = self.token_to_op(operator);
                self.emit(format!("{} = {} {}", temp, op_str, e));
                temp
            }
            Expression::Assignment { left, right } => {
                let r = self.gen_expr(right);
                match &**left {
                    Expression::Identifier(id) => {
                        self.emit(format!("{} = {}", id, r));
                        id.clone()
                    }
                    _ => panic!("L value must be an identifier"),
                }
            }
            Expression::FunctionCall(call) => {
                for arg in &call.args {
                    let a = self.gen_expr(arg);
                    self.emit(format!("Param {}", a));
                }
                let temp = self.new_temp();
                self.emit(format!(
                    "{} = Call {}, {}",
                    temp,
                    call.identifier,
                    call.args.len()
                ));
                temp
            }
        }
    }

    fn gen_if(&mut self, if_stmt: &IfStatement) {
        let end_label = self.new_label();

        // Main If
        let next_label = self.new_label();
        let cond = self.gen_expr(&if_stmt.condition);
        self.emit(format!("IfZ {} Goto {}", cond, next_label));
        self.gen_block(&if_stmt.block);
        self.emit(format!("Goto {}", end_label));
        self.emit(format!("{}:", next_label));

        // Elifs
        for elif in &if_stmt.elif_blocks {
            let next_elif_label = self.new_label();
            let cond = self.gen_expr(&elif.condition);
            self.emit(format!("IfZ {} Goto {}", cond, next_elif_label));
            self.gen_block(&elif.block);
            self.emit(format!("Goto {}", end_label));
            self.emit(format!("{}:", next_elif_label));
        }

        // Else
        if let Some(else_block) = &if_stmt.else_block {
            self.gen_block(else_block);
        }

        self.emit(format!("{}:", end_label));
    }

    fn gen_while(&mut self, while_stmt: &WhileStatement) {
        let start_label = self.new_label();
        let end_label = self.new_label();

        self.loop_stack
            .push((start_label.clone(), end_label.clone()));

        self.emit(format!("{}:", start_label));
        let cond = self.gen_expr(&while_stmt.condition);
        self.emit(format!("IfZ {} Goto {}", cond, end_label));

        self.gen_block(&while_stmt.block);
        self.emit(format!("Goto {}", start_label));

        self.emit(format!("{}:", end_label));

        self.loop_stack.pop();
    }

    fn gen_for(&mut self, for_stmt: &ForStatement) {
        let start_label = self.new_label();
        let continue_label = self.new_label();
        let end_label = self.new_label();

        self.loop_stack
            .push((continue_label.clone(), end_label.clone()));

        if let Some(init) = &for_stmt.init_var {
            self.gen_var_decl(init);
        }

        self.emit(format!("{}:", start_label));

        if let Some(cond_expr) = &for_stmt.condition {
            let cond = self.gen_expr(cond_expr);
            self.emit(format!("IfZ {} Goto {}", cond, end_label));
        }

        self.gen_block(&for_stmt.block);

        self.emit(format!("{}:", continue_label));
        if let Some(update) = &for_stmt.update {
            self.gen_expr(update);
        }
        self.emit(format!("Goto {}", start_label));

        self.emit(format!("{}:", end_label));

        self.loop_stack.pop();
    }

    fn token_to_op(&self, token: &Token) -> String {
        match token {
            Token::T_PLUS_OPR => "+".to_string(),
            Token::T_MINUS_OPR => "-".to_string(),
            Token::T_MULTIPLY_OPR => "*".to_string(),
            Token::T_DIVIDE_OPR => "/".to_string(),
            Token::T_EXPONENT_OPR => "^".to_string(),
            Token::T_EQUALS_OPR => "==".to_string(),
            Token::T_NOT_EQUALS_OPR => "!=".to_string(),
            Token::T_LESS_THAN_OPR => "<".to_string(),
            Token::T_GREATER_THAN_OPR => ">".to_string(),
            Token::T_LESS_THAN_EQUAL_TO_OPR => "<=".to_string(),
            Token::T_GREATER_THAN_EQUAL_TO_OPR => ">=".to_string(),
            Token::T_AND_OPR => "&&".to_string(),
            Token::T_OR_OPR => "||".to_string(),
            Token::T_RIGHT_SHIFT_OPR => ">>".to_string(),
            Token::T_LEFT_SHIFT_OPR => "<<".to_string(),
            Token::T_ASSIGNMENT_OPR => "=".to_string(),
            Token::T_NOT => "!".to_string(),
            _ => format!("{:?}", token),
        }
    }
}

pub fn ir_generator(ast: &RootList) -> Result<String, ()> {
    let mut ir_gen = IrGenerator::new();
    ir_gen.generate_ir(ast)
}
