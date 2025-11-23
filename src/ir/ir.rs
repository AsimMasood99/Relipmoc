use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;

use crate::lexer::tokens::Token;
use crate::parser::enums::*;

// struct IR {
//     pub ir_string: String,
// }

// impl IR {
//     pub fn new() -> Self {
//         IR {
//             ir_string: String::new(),
//         }
//     }

//     fn variable_ir() {

//     }

//     fn function_ir() {

//     }

//     pub fn generate_ir(self, ast: &RootList) -> Result<String, ()> {

//         for root in ast {
//             match root {
//                 Root::Var(var_decl)  => {
//                     Self::variable_ir();
//                 }

//                 Root::Func(func_decl) => {
//                     Self::function_ir();
//                 }
//             }
//         }

//         Ok(self.ir_string)
//     }
// }

struct codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> codegen<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>, builder: Builder<'ctx>) -> Self {
        codegen {
            context,
            module,
            builder,
        }
    }

    fn tok_to_llvm_type(&self, type_tok: &Token) -> BasicTypeEnum {
        match type_tok {
            Token::T_INT => self.context.i64_type().into(),
            Token::T_FLOAT => self.context.f64_type().into(),
            Token::T_BOOL => self.context.bool_type().into(),
            _ => panic!("Unsupported type token"),
        }
    }

    fn gen_ir_variable(&self, var_decl: &VariableDeclaration) {}

    fn gen_ir_function(&self) {
        // Implement function IR generation logic here
    }

    pub fn generate_ir(&self, ast: &RootList) -> Result<String, ()> {
        for root in ast {
            match root {
                Root::Var(var_decl) => {
                    self.gen_ir_variable(var_decl);
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
