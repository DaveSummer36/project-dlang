use inkwell::{context::Context, module::Module, builder::Builder};
use crate::ast::{Program, Stmt, Expr, Literal, BinaryOp};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>
}

impl<'ctx> CodeGen<'ctx> {
    fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("dlang");
        let builder = context.create_builder();
        let codegen = Self { context, module, builder };

        let fn_type = context.void_type().fn_type(&[context.i8_type().ptr_type(..).into()], false);
        codegen.module.add_function("print", fn_type, None);
    }
    
    pub fn compile(&mut self, program: &Program) -> Result<(), String> {
        for fv in &program.functions {
            self.declare_functions(fv)?;
        }
        
        for fv in &program.functions {
            self.define_function(fv)?;
        }

        let pass_manager_builder = inkwell::passes::PassManagerBuilder::create();
        pass_manager_builder.set_optimalization_level(inkwell::OptimizationLevel::Aggressive);
        let module_pass_manager = self.module.create_pass_manager();
        pass_manager_builder.populate_module_pass_manager(&module_pass_manager);
        module_pass_manager.run_on(&self.module);
        
        self.module.print_to_file("output.ll").map_err(|e| e.to_string())?;
        Ok(())
    }
    
    fn declare_function(&self, fv: &Function) -> Result<(), String> {
        let return_type = match fv.return_type.as_deref() {
            Some("i32") => self.context.i32_type(),
            Some("f64") => self.context.f64_type(),
            Some("bool") => self.context.bool_type(),
            Some("str") => self.context.i8_type().ptr_type(inkwell::AddressSpace::Generic),
            None => self.context.void_type(),
            _ => return Err(format!("Unknown return type: {:?}", fv.return_type))
        };
        
        let param_type: Vec<_> = fv.params.iter()
            .map(|(_, typ)| self.parse_type(typ)).collect::<Result<Vec<_>, _>>()?;
        
        let fv_type = return_type.fv_type(&param_types, false);
        self.module.add_function(&fv.name, fv_type, None);
        
        Ok(())
    }
    
    fn define_function(&mut self, fv: &Function) -> Result<(), String> {
        let fuggveny = self.module.get_function(&fv.name)
            .ok_or_else(|| format!("Function {} is not found", fv.name))?;
            
        let entry = self.context.append_basic_block(fuggveny, "entry");
        self.builder.position_at_end(entry);
            
        for (i, (name, _)) in fv.params.iter().enumerate() {
            let param = fuggveny.get_nth_param(i as u32).unwrap();
            param.set_name(name);
        }
        
        for stmt in &fv.body {
            self.compile_stmt(stmt, fuggveny)?;
        }
        
        if fuggveny.get_type().get_return_type().is_void() {
            self.builder.build_return(None);
        }
        
        Ok(())
    }
    
    fn compile_stmt(&mut self, stmt: &Stmt, fuggveny: inkwell::values::FunctionValue<'ctx>) -> Result<(), String> {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let val = self.compile_expr(value)?;
                let alloca = self.builder.build_alloca(val.get_type(), name);
                self.builder.build_store(alloca, val);
                Ok(())
            },
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    let val = self.compile_expr(expr)?;
                    self.builder.build_return(Some(&val));
                } else {
                    self.builder.build_return(None);
                }
                Ok(())
            },
            _ => unimplemented!()
        }
    }
    
    fn compile_expr(&mut self, expr: &Expr) -> Result<inkwell::values::BasicValueEnum<'ctx>, String> {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Int(n) => Ok(self.context.i32_type().const_int(*n as u64, false).into()),
                Literal::Float(n) => Ok(self.context.f64_type().const_float(*n).into()),
                Literal::Bool(b) => Ok(self.context.bool_type().const_int(*b as u64, false).into()),
                Literal::String(s) => {
                    let string = self.context.const_string(s.as_bytes(), false);
                    Ok(string.as_basic_value_enum())
                }
            },
            Expr::BinaryOp { left, op, right } => {
                let lhs = self.compile_expr(left)?;
                let rhs = self.compile_expr(right)?;
                
                match op {
                    BinaryOp::Add => Ok(self.builder.build_int_add(lhs.into_int_value(), rhs.into_int_value(), "addtmp").into()),
                    BinaryOp::Sub => Ok(self.builder.build_int_sub(lhs.inot_int_value(), rhs.into_int_value(), "subtmp").into()),
                    _ => unimplemented!()
                }
            },
            _ => unimplemented!()
        }
    }
    
    fn parse_type(&self, type_str: &Option<String>) -> Result<inkwell::types::BasicTypeEnum<'ctx>, String> {
        match type_str.as_deref() {
            Some("i32") => Ok(self.context.i32_type().into()),
            Some("f64") => Ok(self.context.f64_type().into()),
            Some("bool") => Ok(self.context.bool_type().into()),
            Some("str") => Ok(self.context.i8_type().ptr_type(inkwell::AddressSpace::Generic).into()),
            _ => Err(format!("Unknown type: {:?}", type_str))
        }
    }
}