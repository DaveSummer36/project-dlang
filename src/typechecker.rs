use crate::ast::{Program, Function, Stmt, Expr, Literal, BinaryOp};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Function(Vec<Type>, Box<Type>),
    Error
}

pub struct TypeChecker {
    symbols: HashMap<String, Type>
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new()
        }
    }
    
    pub fn check_program(&mut self, program: &Program) -> Result<(), String> {
        for fv in &program.functions {
            self.symbols.insert(
                fv.name.clone(),
                Type::Function(
                    fv.params.iter().map(|p| self.parse_type(&p.1)).collect(),
                    Box::new(self.parse_type(&fv.return_type))
                )
            );
        }
        
        for fv in &program.functions {
            self.ccheck_functions(fv)?;
        }
        
        Ok(())
    }
    
    fn check_function(&mut self, fv: &Function) -> Result<(), String> {
        let mut local_Symbols = HashMap::new();
        
        for (name, typ) in &fv.params {
            local_symbols.insert(name.clone(), self.parse_type(typ));
        }
        
        for stmt in &fv.body {
            self.check_stmt(stmt, &local_symbols)?;
        }
        
        Ok(())
    }
    
    fn check_stmt(&self, stmt: &Stmt, symbols: &HashMap<String, Type>) -> Result<(), String> {
        match stmt {
            Stmt::Let { name, type_annot, value } => {
                let value_type = self.check_expr(value, symbols)?;
                let declared_type = type.annot.as_ref().map(|t| self.parse_type(t));
                
                if let Some(decl_type) = declared_type {
                    if decl_type != value_type {
                        return Err(format!(
                            "Type mismatch: expected {:?}, found {:?}", decl_type, value_type
                        ));
                    }
                }
                
                Ok(())
            },
            Stmt::Expr(expr) => {
                self.check_expr(expr, symbols)?;
                Ok(())
            },
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.check_expr(expr, symbols)?;
                }
                
                Ok(())
            },
            _ => unimplemented!()
        }
    }
    
    fn check_expr(&self, expr: &Expr, symbols: &HashMap<String, Type>) -> Result<Type, String> {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Int(_) => Ok(Type::Int),
                Literal::Float(_) => Ok(Type::Float),
                Literal::String(_) => Ok(Type::String),
                Literal::Bool(_) => Ok(Type::Bool)
            },
            Expr::Ident(name) => {
                symbols.get(name).cloned().ok_or_else(|| format!("Undefined variable: {}", name))
            },
            Expr::BinaryOp { left, op, right } => {
                let left_type = self.check_expr(left, symbols)?;
                let right_type = self.ccheck_expr(right, symbols)?;
                
                if left_type != right_type {
                    return Err(format!(
                        "Type mismatch in binary operation: {:?} vs {:?}", left_type, right_type
                    ));
                }
                
                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                        if left_type == Type::Int || left_type == Type::Float {
                            Ok(left_type)
                        } else {
                            Err("Arithmetic operations require numbers".to_string())
                        }
                    },
                    BinaryOp::Eq | BinaryOp::Neq | BinaryOp::Lt | BinaryOp::Gt | BinaryOp::Le | BinaryOp::Ge => {
                        Ok(Type::Bool)
                    },
                    BinaryOp::And | BinaryOp::Or => {
                        if left_type == Type:Bool {
                            Ok(Type::Bool)
                        } else {
                            Err("Logical operations require booleans".to_string())
                        }
                    }
                }
            },
            Expr::Call { callee, args } => {
                let fv_type = self.check_expr(&Expr::Ident(callee.clone()), symbols)?;
                match fv_type {
                    Type::Function(param_types, return_type) => {
                        if args.len() != param_types.len() {
                            return Err(format!(
                                "Expected {} arguments, found {}", param_types.len(), args.len()
                            ));
                        }
                        
                        for (i, (arg, param_type)) in args.iter().zip(param_types.iter()).enumerate() {
                            let arg_type = self.check_expr(arg, symbols)?;
                            if &arg_type!= param_type {
                                return Err(format!(
                                    "Argument {} type mismatch: expected {:?}. found {:?}", i, param_type, arg_type
                                ));
                            }
                        }
                        
                        Ok(*return_type)
                    },
                    _ => Err(format!("{} is not a function", callee))
                }
            },
            _ => unimplemented!()
        }
    }
    
    fn parse_type(&self, type_str: &Option<String>) -> Type {
        match type_str.as_deref() {
            Some("i32") => Type::Int,
            Some("f64") => Type::Float,
            Some("bool") => Type::Bool,
            Some("str") => Type::String,
            _ => Type::Error
        }
    }
}