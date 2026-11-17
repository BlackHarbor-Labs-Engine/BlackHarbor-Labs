use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicBlock, BasicBlockBuilder, BasicBlockType, IntegerType, FloatType, PointerType};
use inkwell::types::Type;
use inkwell::values::{Value, Constant};
use inkwell::backend::Target;
use inkwell::types::IntegerType;
use crate::ast::{Expr, Literal, Operator, VariableRef, FunctionCall, Stmt};

/// The IRGenerator traverses the AST and emits corresponding LLVM IR.
pub struct IRGenerator {
    context: Context,
    module: Module,
    current_block: BasicBlock,
}

impl IRGenerator {
    /// Creates a new generator initialized with a fresh LLVM context and module.
    pub fn new(context: Context, module: Module) -> Self {
        let builder = BasicBlockBuilder::new(&context, &module, "entry_block");
        IRGenerator {
            context,
            module,
            current_block: builder.build(module.as_ref()),
        }
    }

    /// Generates the IR for the entire expression tree starting from the given root.
    pub fn generate_ir(&mut self, expr: &Expr) -> Value {
        self.gen_expr(expr)
    }

    /// Dispatches to the correct generator based on the expression type.
    fn gen_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(lit) => self.gen_literal(lit),
            Expr::Variable(var) => self.gen_variable(var),
            Expr::Binary(bin) => self.gen_binary(bin),
            Expr::FunctionCall(call) => self.gen_function_call(call),
            Expr::If(if_expr) => self.gen_if(if_expr),
            Expr::While(while_expr) => self.gen_while(while_expr),
            Expr::Block(block) => self.gen_block(block),
            _ => panic!("Unsupported expression type in IRGenerator: {:?}", expr),
        }
    }

    /// Generates IR for a literal value.
    fn gen_literal(&self, lit: &Literal) -> Value {
        match lit {
            Literal::Int(i) => Constant::new_int(*i, &self.context),
            Literal::Float(f) => Constant::new_float(*f, &self.context),
            Literal::String(s) => {
                let string_type = self.context.string_type(s.len() as u32);
                let string_val = string_type.get_string(s);
                string_val.as_basic_value().into()
            }
            _ => panic!("Unsupported literal type in IRGenerator: {:?}", lit),
        }
    }

    /// Generates IR for a variable reference.
    fn gen_variable(&self, var: &VariableRef) -> Value {
        // Placeholder: In a real implementation, this would load the value from a symbol table.
        // Here, we return a constant placeholder to satisfy the type checker.
        println!("Warning: Generating placeholder for variable: {}", var.0.name);
        Constant::new_int(0, &self.context)
    }

    /// Generates IR for a binary operation.
    fn gen_binary(&mut self, bin: &crate::ast::Binary) -> Value {
        let lhs = self.gen_expr(&bin.left);
        let rhs = self.gen_expr(&bin.right);
        let op = &bin.op;

        match op {
            Operator::Add => lhs.add(rhs, &self.context),
            Operator::Sub => lhs.sub(rhs, &self.context),
            Operator::Mul => lhs.mul(rhs, &self.context),
            Operator::Div => lhs.div(rhs, &self.context),
            Operator::Eq => lhs.icmp(IntPredicate::EQ, rhs, &self.context),
            Operator::Ne => lhs.icmp(IntPredicate::NE, rhs, &self.context),
            Operator::Lt => lhs.icmp(IntPredicate::ULT, rhs, &self.context),
            Operator::Le => lhs.icmp(IntPredicate::ULE, rhs, &self.context),
            Operator::Gt => lhs.icmp(IntPredicate::UGT, rhs, &self.context),
            Operator::Ge => lhs.icmp(IntPredicate::UGE, rhs, &self.context),
            Operator::And => {
                let lhs_val = self.gen_expr(&bin.left);
                let rhs_val = self.gen_expr(&bin.right);
                lhs_val.and(rhs_val, &self.context)
            }
            Operator::Or => {
                let lhs_val = self.gen_expr(&bin.left);
                let rhs_val = self.gen_expr(&bin.right);
                lhs_val.or(rhs_val, &self.context)
            }
            Operator::Not => {
                let val = self.gen_expr(&bin.left);
                val.not(&self.context)
            }
            Operator::Assign => {
                let val = self.gen_expr(&bin.right);
                self.gen_assign(&bin.left, val)
            }
        }
    }

    /// Generates IR for a function call.
    fn gen_function_call(&mut self, call: &FunctionCall) -> Value {
        unimplemented!()
    }

    /// Generates IR for an if statement.
    fn gen_if(&mut self, if_expr: &crate::ast::If) -> Value {
        unimplemented!()
    }

    /// Generates IR for a while loop.
    fn gen_while(&mut self, while_expr: &crate::ast::While) -> Value {
        unimplemented!()
    }

    /// Generates IR for a block of statements.
    fn gen_block(&mut self, block: &Vec<crate::ast::Stmt>) -> Value {
        unimplemented!()
    }

    /// Generates IR for an assignment.
    fn gen_assign(&mut self, var: &VariableRef, value: Value) -> Value {
        unimplemented!()
    }
}
