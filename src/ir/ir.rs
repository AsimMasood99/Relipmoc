use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{
    BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue, PointerValue,
};
use inkwell::{AddressSpace, FloatPredicate, IntPredicate};

use std::collections::HashMap;

use crate::lexer::tokens::Token;
use crate::parser::enums::*;

struct codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
    current_func: Option<FunctionValue<'ctx>>,
    loop_stack: Vec<(
        inkwell::basic_block::BasicBlock<'ctx>,
        inkwell::basic_block::BasicBlock<'ctx>,
    )>,
}

impl<'ctx> codegen<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>, builder: Builder<'ctx>) -> Self {
        codegen {
            context,
            module,
            builder,
            variables: HashMap::new(),
            current_func: None,
            loop_stack: Vec::new(),
        }
    }
    fn tok_to_llvm_type(&self, type_tok: &Token) -> BasicTypeEnum<'ctx> {
        match type_tok {
            Token::T_INT => self.context.i64_type().into(),
            Token::T_FLOAT => self.context.f64_type().into(),
            Token::T_BOOL => self.context.bool_type().into(),
            Token::T_STRING => self
                .context
                .ptr_type(AddressSpace::default())
                .as_basic_type_enum(),
            _ => panic!("Unsupported type token"),
        }
    }

    fn global_var_ir(&mut self, var_decl: &VariableDeclaration) -> Result<(), ()> {
        let var_type = self.tok_to_llvm_type(&var_decl.type_token);
        let global_context = self.module.add_global(var_type, None, &var_decl.identifier);

        match &var_decl.expression {
            Expression::Literal(Constants::Int(value)) => {
                let const_value = self.context.i64_type().const_int(*value as u64, false);
                global_context.set_initializer(&const_value);
            }
            Expression::Literal(Constants::Float(value)) => {
                let const_value = self.context.f64_type().const_float(*value);
                global_context.set_initializer(&const_value);
            }
            Expression::Literal(Constants::Bool(value)) => {
                let const_value = self
                    .context
                    .bool_type()
                    .const_int(if *value { 1 } else { 0 }, false);
                global_context.set_initializer(&const_value);
            }
            Expression::Literal(Constants::Str(value)) => {
                // build_global_string_ptr returns Result<GlobalValue, BuilderError>, unwrap or handle the error
                let const_value = self
                    .builder
                    .build_global_string_ptr(value, "str")
                    .expect("Failed to build global string");
                global_context.set_initializer(&const_value.as_pointer_value());
            }
            _ => {
                panic!("Unsupported expression in global variable initializer");
            }
        }

        self.variables.insert(
            var_decl.identifier.clone(),
            global_context.as_pointer_value(),
        );
        Ok(())
    }

    fn gen_ir_function(&self) {
        // Implement function IR generation logic here
    }

    pub fn generate_ir(&mut self, ast: &RootList) -> Result<String, ()> {
        for root in ast {
            match root {
                Root::Var(var_decl) => {
                    self.global_var_ir(var_decl);
                }

                Root::Func(func_decl) => {
                    self.gen_ir_function();
                }
            }
        }

        Ok(String::new()); // Placeholder
    }
}

pub fn ir(ast: &RootList) -> Result<String, ()> {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    // let ir_generator = IR::new();
    // let ir_code = ir_generator.generate_ir(ast)?;
    // Ok(ir_code)
}
