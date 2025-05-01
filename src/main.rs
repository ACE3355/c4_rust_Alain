

/// Enum representing the virtual machine's instruction set for executing compiled code.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    IMM(i64),
    PSH,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    JMP(usize),
    BZ(usize),
    BNZ(usize),
    JSR(usize),
    ENT(usize),
    ADJ(usize),
    LEV,
    LEA(usize),
    LI,
    LC,
    SI,
    SC,
    EXIT,
    PRTF,
    MALC,
    FREE,
    MSET,
    MCMP,
    OPEN,
    READ,
    CLOS,
    EQ,
    LT,
    GT,
}

// Stack-based virtual machine
pub struct VM {
    pub stack: Vec<i64>,
    pub pc: usize,
    pub bp: usize,
    pub program: Vec<Instruction>,
    pub running: bool,
}


/// A stack-based virtual machine for executing C4-style bytecode instructions.
impl VM {
    pub fn new(program: Vec<Instruction>) -> Self {
        VM {
            stack: Vec::new(),
            pc: 0,
            bp: 0,
            program,
            running: true,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            if self.pc >= self.program.len() {
                panic!("Program counter out of bounds");
            }

            match self.program[self.pc] {
                Instruction::IMM(val) => {
                    self.stack.push(val);
                }
                Instruction::PSH => {
                    if let Some(&top) = self.stack.last() {
                        self.stack.push(top);
                    } else {
                        panic!("PSH failed: stack is empty");
                    }
                }
                Instruction::ADD => {
                    let b = self.stack.pop().expect("ADD: missing operand B");
                    let a = self.stack.pop().expect("ADD: missing operand A");
                    self.stack.push(a + b);
                }
                Instruction::SUB => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                Instruction::MUL => {
                    let b = self.stack.pop().expect("MUL: missing operand B");
                    let a = self.stack.pop().expect("MUL: missing operand A");
                    self.stack.push(a * b);
                }
                Instruction::DIV => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                }
                Instruction::MOD => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a % b);
                }
                Instruction::JMP(target) => {
                    self.pc = target;
                    continue;
                }
                Instruction::BZ(target) => {
                    let cond = self.stack.pop().unwrap();
                    if cond == 0 {
                        self.pc = target;
                        continue;
                    }
                }
                Instruction::BNZ(target) => {
                    let cond = self.stack.pop().unwrap();
                    if cond != 0 {
                        self.pc = target;
                        continue;
                    }
                }
                Instruction::JSR(target) => {
                    self.stack.push((self.pc + 1) as i64);
                    self.pc = target;
                    continue;
                }
                Instruction::ENT(size) => {
                    self.stack.push(self.bp as i64);
                    self.bp = self.stack.len();
                    self.stack.resize(self.stack.len() + size, 0);
                }
                Instruction::ADJ(n) => {
                    for _ in 0..n {
                        self.stack.pop();
                    }
                }
                Instruction::LEV => {
                    let old_bp = self.stack[self.bp - 1];
                    self.stack.truncate(self.bp - 1);
                    self.bp = old_bp as usize;
                    self.pc = self.stack.pop().unwrap() as usize;
                    continue;
                }
                Instruction::LEA(offset) => {
                    let addr = self.bp + offset;
                    self.stack.push(addr as i64);
                }
                Instruction::LI => {
                    let addr = self.stack.pop().unwrap() as usize;
                    let val = self.stack[addr];
                    self.stack.push(val);
                }
                Instruction::LC => {
                    let addr = self.stack.pop().unwrap() as usize;
                    let val = self.stack[addr] & 0xFF;
                    self.stack.push(val);
                }
                Instruction::SI => {
                    let val = self.stack.pop().unwrap();
                    let addr = self.stack.pop().unwrap() as usize;
                    self.stack[addr] = val;
                }
                Instruction::SC => {
                    let val = self.stack.pop().unwrap() & 0xFF;
                    let addr = self.stack.pop().unwrap() as usize;
                    self.stack[addr] = val;
                }
                Instruction::EXIT => {
                    if let Some(&result) = self.stack.last() {
                        println!("output {}", result);
                    } else {
                        println!("Program got exit without a return value");
                    }
                    self.running = false;
                }
                Instruction::PRTF => {
                    let _arg_count = self.stack.pop().unwrap();
                    let _fmt_addr = self.stack.pop().unwrap();
                    println!("[PRTF] Simulated printf");
                    self.stack.push(0);
                }
                Instruction::MALC => {
                    self.stack.push(0x1000);
                }
                Instruction::FREE => {
                    let _ = self.stack.pop();
                }
                Instruction::MSET => {
                    let _ = self.stack.pop();
                    let _ = self.stack.pop();
                    let _ = self.stack.pop();
                }
                Instruction::MCMP => {
                    let _ = self.stack.pop();
                    let _ = self.stack.pop();
                    let _ = self.stack.pop();
                    self.stack.push(0);
                }
                Instruction::OPEN => {
                    let _ = self.stack.pop();
                    let _ = self.stack.pop();
                    self.stack.push(3);
                }
                Instruction::READ => {
                    let _ = self.stack.pop();
                    let _ = self.stack.pop();
                    let _ = self.stack.pop();
                    self.stack.push(10);
                }
                Instruction::CLOS => {
                    let _ = self.stack.pop();
                    self.stack.push(0);
                }
                Instruction::EQ => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a == b) as i64);
                }
                Instruction::LT => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a < b) as i64);
                }
                Instruction::GT => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a > b) as i64);
                }
            }
            self.pc += 1;
        }
    }
}

// Generate VM instructions from parsed AST
pub fn get_vm(ast: &ASTNode) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut symbol_table = HashMap::new();
    let mut next_offset = 0;
    let mut patches: Vec<(usize, String)> = Vec::new();

    instructions.push(Instruction::ENT(0));

    get_vm_inner(ast, &mut instructions, &mut symbol_table, &mut next_offset, &mut patches);

    let mut function_addresses = HashMap::new();
    if let ASTNode::Sequence(stmts) = ast {
        for (i, stmt) in stmts.iter().enumerate() {
            if let ASTNode::FunctionDef { name, .. } = stmt {
                function_addresses.insert(name.clone(), i);
            }
        }
    }

    for (index, func_name) in patches {
        if let Some(&addr) = function_addresses.get(&func_name) {
            instructions[index] = Instruction::JSR(addr);
        } else {
            panic!("Unresolved function call: {}", func_name);
        }
    }

    instructions[0] = Instruction::ENT(next_offset);

    instructions
}

// Recursively generates instructions from the AST
fn get_vm_inner(
    ast: &ASTNode,
    instructions: &mut Vec<Instruction>,
    symbol_table: &mut HashMap<String, usize>,
    next_offset: &mut usize,
    patches: &mut Vec<(usize, String)>,
) {
    match ast {
         // Generate return value and exit the program
        // Handles: return <expr>;
        ASTNode::Return(expr) => {
            emit_expr(expr, instructions, symbol_table, patches);
            instructions.push(Instruction::PSH);
            instructions.push(Instruction::EXIT);
        }
        // Generate conditional branch (if-else)
        // Handles: if (<condition>) { ... } else { ... }
        ASTNode::If { condition, then_branch, else_branch } => {
            emit_expr(condition, instructions, symbol_table, patches);
            let jump_false_index = instructions.len();
            instructions.push(Instruction::BZ(9999));// Patch jump to 'else' or 'after then' based on condition outcome
            get_vm_inner(then_branch, instructions, symbol_table, next_offset, patches);
            if let Some(else_branch) = else_branch {
                let jump_over_else_index = instructions.len();
                instructions.push(Instruction::JMP(9999));
                let else_start = instructions.len();
                get_vm_inner(else_branch, instructions, symbol_table, next_offset, patches);
                let after_else = instructions.len();
                instructions[jump_false_index] = Instruction::BZ(else_start);
                instructions[jump_over_else_index] = Instruction::JMP(after_else);
            } else {
                let after_then = instructions.len();
                instructions[jump_false_index] = Instruction::BZ(after_then);
            }
        }
         // Generate loop with conditional jump (while-loop)
        // Handles: while (<condition>) { ... }
        ASTNode::While { condition, body } => {
            let loop_start = instructions.len();
            emit_expr(condition, instructions, symbol_table, patches);
            let jump_if_false_index = instructions.len();
            instructions.push(Instruction::BZ(9999));
            get_vm_inner(body, instructions, symbol_table, next_offset, patches);
            instructions.push(Instruction::JMP(loop_start));
            let loop_end = instructions.len();
            instructions[jump_if_false_index] = Instruction::BZ(loop_end);
        }
        // Execute a sequence of statements in order
        ASTNode::Sequence(statements) => {
            for stmt in statements {
                get_vm_inner(stmt, instructions, symbol_table, next_offset, patches);
            }
        }
        // Handle variable declaration: allocate space and initialize
        ASTNode::Declaration(name, expr) => {
            let offset = *next_offset;
            *next_offset += 1;
            symbol_table.insert(name.clone(), offset);
            instructions.push(Instruction::LEA(offset));// Push address for variable storage
            emit_expr(expr, instructions, symbol_table, patches);// Emit expression instructions and store result
            instructions.push(Instruction::SI);
        }
        // Handle variable assignment: evaluate and store value
        ASTNode::Assignment(name, expr) => {
            if let Some(&offset) = symbol_table.get(name) {
                instructions.push(Instruction::LEA(offset));
                emit_expr(expr, instructions, symbol_table, patches);
                instructions.push(Instruction::SI);
            } else {
                panic!("Assignment to undeclared variable: {}", name);
            }
        }
        // Handle function definition: reset symbol table and compile body
        ASTNode::FunctionDef { name: _, params, body } => {
            symbol_table.clear();
            *next_offset = params.len();
            for (i, param) in params.iter().enumerate() {
                symbol_table.insert(param.clone(), i);
            }
            get_vm_inner(body, instructions, symbol_table, next_offset, patches);
        }
    }
}

// Translates a high-level expression (AST) into low-level virtual machine instructions.
// This function recursively traverses the expression tree and generates instructions
// that the virtual machine can execute. It supports arithmetic operations, comparisons,
// variable access, and function calls. For variables, it uses a symbol table to resolve
// memory offsets. For function calls, it defers jump resolution using a patch table.
fn emit_expr(
    expr: &Expr,
    instructions: &mut Vec<Instruction>,
    symbol_table: &HashMap<String, usize>,
    patches: &mut Vec<(usize, String)>,
) {
    match expr {
        Expr::Number(n) => {
            instructions.push(Instruction::IMM(*n));
        }
        Expr::Add(lhs, rhs) => {
            emit_expr(lhs, instructions, symbol_table, patches);
            emit_expr(rhs, instructions, symbol_table, patches);
            instructions.push(Instruction::ADD);
        }
        Expr::Sub(lhs, rhs) => {
            emit_expr(lhs, instructions, symbol_table, patches);
            emit_expr(rhs, instructions, symbol_table, patches);
            instructions.push(Instruction::SUB);
        }
        Expr::Mul(lhs, rhs) => {
            emit_expr(lhs, instructions, symbol_table, patches);
            emit_expr(rhs, instructions, symbol_table, patches);
            instructions.push(Instruction::MUL);
        }
        Expr::Div(lhs, rhs) => {
            emit_expr(lhs, instructions, symbol_table, patches);
            emit_expr(rhs, instructions, symbol_table, patches);
            instructions.push(Instruction::DIV);
        }
        Expr::Mod(lhs, rhs) => {
            emit_expr(lhs, instructions, symbol_table, patches);
            emit_expr(rhs, instructions, symbol_table, patches);
            instructions.push(Instruction::MOD);
        }
        Expr::Equal(lhs, rhs) => {
            emit_expr(lhs, instructions, symbol_table, patches);
            emit_expr(rhs, instructions, symbol_table, patches);
            instructions.push(Instruction::EQ);
        }
        Expr::Less(lhs, rhs) => {
            emit_expr(lhs, instructions, symbol_table, patches);
            emit_expr(rhs, instructions, symbol_table, patches);
            instructions.push(Instruction::LT);
        }
        Expr::Greater(lhs, rhs) => {
            emit_expr(lhs, instructions, symbol_table, patches);
            emit_expr(rhs, instructions, symbol_table, patches);
            instructions.push(Instruction::GT);
        }
        Expr::Variable(name) => {
            if let Some(&offset) = symbol_table.get(name) {
                instructions.push(Instruction::LEA(offset));
                instructions.push(Instruction::LI);
            } else {
                panic!("Use of undeclared variable: {}", name);
            }
        }
        Expr::Call(func_name, args) => {
            for arg in args {
                emit_expr(arg, instructions, symbol_table, patches);
            }
            let placeholder_index = instructions.len();
            instructions.push(Instruction::JSR(9999));
            patches.push((placeholder_index, func_name.clone()));
        }
        Expr::Var(name) => {
            if let Some(&offset) = symbol_table.get(name) {
                instructions.push(Instruction::LEA(offset));
                instructions.push(Instruction::LI);
            } else {
                panic!("Use of undeclared variable: {}", name);
            }
        }
    }
}

// Entry point for the compiler and virtual machine.
// This function reads a C-like source file, tokenizes it, parses it into an AST,
// translates the AST into virtual machine instructions, and then executes the program.
// Expects a single command-line argument: the path to the input C file.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input.c>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Failed to read source file");

    let tokens = tokenizer(&source);
    let ast = parse_token(&tokens);
    let program = get_vm(&ast);
    let mut vm = VM::new(program);
    vm.run();
}

// Tests for the compiler
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_basic_function() {
        let src = "int main() { return 42; }";
        let tokens = tokenizer(src);
        assert_eq!(
            tokens,
            vec![
                Token::Int,
                Token::Identifier("main".into()),
                Token::LParen,
                Token::RParen,
                Token::LBrace,
                Token::Return,
                Token::Number(42),
                Token::Semicolon,
                Token::RBrace
            ]
        );
    }

    #[test]
    fn virtual_machine_addition() {
        let prog = vec![Instruction::IMM(4), Instruction::IMM(6), Instruction::ADD, Instruction::EXIT];
        let mut vm = VM::new(prog);
        vm.run();
        assert_eq!(vm.stack, vec![10]);
    }

    #[test]
    fn vm_branch_on_zero_skips_add() {
        let prog = vec![
            Instruction::IMM(0),
            Instruction::BZ(5),
            Instruction::IMM(100),
            Instruction::IMM(200),
            Instruction::ADD,
            Instruction::IMM(9),
            Instruction::EXIT,
        ];
        let mut vm = VM::new(prog);
        vm.run();
        assert_eq!(vm.stack, vec![9]);
    }

    #[test]
    fn vm_branch_on_nonzero_executes_add() {
        let prog = vec![
            Instruction::IMM(1),
            Instruction::BNZ(5),
            Instruction::IMM(10),
            Instruction::IMM(20),
            Instruction::ADD,
            Instruction::IMM(7),
            Instruction::EXIT,
        ];
        let mut vm = VM::new(prog);
        vm.run();
        assert_eq!(vm.stack, vec![7]);
    }

    #[test]
    fn vm_simple_function_call() {
        let prog = vec![
            Instruction::JSR(4),
            Instruction::IMM(123),
            Instruction::PSH,
            Instruction::EXIT,
            Instruction::ENT(0),
            Instruction::LEV,
        ];
        let mut vm = VM::new(prog);
        vm.run();
        assert_eq!(vm.stack.last(), Some(&123));
    }

    #[test]
    fn vm_memory_store_and_load() {
        let prog = vec![
            Instruction::ENT(1),
            Instruction::LEA(0),
            Instruction::IMM(888),
            Instruction::SI,
            Instruction::LEA(0),
            Instruction::LI,
            Instruction::EXIT,
        ];
        let mut vm = VM::new(prog);
        vm.run();
        assert_eq!(vm.stack.last(), Some(&888));
    }

    #[test]
    fn parse_return_sum_expression() {
        let tokens = tokenizer("int main() { return 8 + 9; }");
        let ast = parse_token(&tokens);
        assert_eq!(
            ast,
            ASTNode::Sequence(vec![ASTNode::Return(Box::new(Expr::Add(
                Box::new(Expr::Number(8)),
                Box::new(Expr::Number(9)),
            )))]),
        );
    }

    #[test]
    fn parse_nested_expression_with_multiplication() {
        let tokens = tokenizer("int main() { return (2 + 3) * 4; }");
        let ast = parse_token(&tokens);
        assert_eq!(
            ast,
            ASTNode::Sequence(vec![ASTNode::Return(Box::new(Expr::Mul(
                Box::new(Expr::Add(
                    Box::new(Expr::Number(2)),
                    Box::new(Expr::Number(3))
                )),
                Box::new(Expr::Number(4))
            )))]),
        );
    }

    #[test]
    fn parse_if_else_flow() {
        let src = "int main() { if (3 < 4) { return 1; } else { return 0; } }";
        let tokens = tokenizer(src);
        let ast = parse_token(&tokens);
        assert_eq!(
            ast,
            ASTNode::Sequence(vec![ASTNode::If {
                condition: Box::new(Expr::Less(
                    Box::new(Expr::Number(3)),
                    Box::new(Expr::Number(4))
                )),
                then_branch: Box::new(ASTNode::Sequence(vec![ASTNode::Return(Box::new(Expr::Number(1)))])),
                else_branch: Some(Box::new(ASTNode::Sequence(vec![ASTNode::Return(Box::new(Expr::Number(0)))])))
            }]),
        );
    }

    #[test]
    fn parse_while_loop_execution() {
        let src = "int main() { while (0 < 1) { return 10; } }";
        let tokens = tokenizer(src);
        let ast = parse_token(&tokens);
        assert_eq!(
            ast,
            ASTNode::Sequence(vec![ASTNode::While {
                condition: Box::new(Expr::Less(Box::new(Expr::Number(0)), Box::new(Expr::Number(1)))),
                body: Box::new(ASTNode::Sequence(vec![ASTNode::Return(Box::new(Expr::Number(10)))]))
            }])
        );
    }

    #[test]
    fn parse_variable_assignment_and_check() {
        let tokens = tokenizer("int x = 10; if (x == 10) { return x; }");
        let expected = vec![
            Token::Int,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Number(10),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Identifier("x".to_string()),
            Token::Equal,
            Token::Number(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Identifier("x".to_string()),
            Token::Semicolon,
            Token::RBrace,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn vm_function_with_variable_return() {
        let tokens = tokenizer("int main() { int y = 7; return y; }");
        let ast = parse_token(&tokens);
        let instructions = get_vm(&ast);
        let mut vm = VM::new(instructions);
        vm.run();
        assert_eq!(vm.stack.last(), Some(&7));
    }

    #[test]
    fn parse_and_codegen_function_call() {
        let ast = ASTNode::Sequence(vec![
            ASTNode::FunctionDef {
                name: "mul".into(),
                params: vec!["x".into(), "y".into()],
                body: Box::new(ASTNode::Return(Box::new(Expr::Mul(
                    Box::new(Expr::Variable("x".into())),
                    Box::new(Expr::Variable("y".into())),
                )))),
            },
            ASTNode::Return(Box::new(Expr::Call("mul".into(), vec![
                Expr::Number(3),
                Expr::Number(4)
            ]))),
        ]);
        let instructions = get_vm(&ast);
        assert_eq!(
            instructions,
            vec![
                Instruction::ENT(2),
                Instruction::LEA(0),
                Instruction::LI,
                Instruction::LEA(1),
                Instruction::LI,
                Instruction::MUL,
                Instruction::PSH,
                Instruction::EXIT,
                Instruction::IMM(3),
                Instruction::IMM(4),
                Instruction::JSR(0),
                Instruction::PSH,
                Instruction::EXIT,
            ]
        );

        // ---------- New Tests Added Below ----------

    #[test]
    fn tokenizer_handles_empty_string() {
        let tokens = tokenizer("");
        assert!(tokens.is_empty());
    }

    #[test]
    fn parse_logical_expression() {
        let tokens = tokenizer("int main() { return 1 < 2 == 1; }");
        let ast = parse_token(&tokens);
        match ast {
            ASTNode::Sequence(v) => match &v[0] {
                ASTNode::Return(expr) => match &**expr {
                    Expr::Equal(left, right) => {
                        matches!(**left, Expr::Less(_, _));
                        matches!(**right, Expr::Number(1));
                    },
                    _ => panic!("Expected equality expression"),
                },
                _ => panic!("Expected return node"),
            },
            _ => panic!("Expected sequence node"),
        }
    }

    #[test]
    fn vm_syscall_stub_behavior() {
        let program = vec![
            Instruction::IMM(1),
            Instruction::IMM(123),
            Instruction::PRTF,
            Instruction::MALC,
            Instruction::IMM(3),
            Instruction::CLOS,
            Instruction::EXIT,
        ];
        let mut vm = VM::new(program);
        vm.run();
        assert_eq!(vm.stack.len(), 3); // Dummy values for now
    }
}

    }

    
