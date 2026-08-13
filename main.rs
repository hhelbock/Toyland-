//=============Tokens============
#[derive(Debug, Clone, PartialEq)]
enum Keyword{
    Func,
    If,
    Otherwise,
    While,
    End,
    Continue,
    As,
    Return,
}

#[derive(Debug, Clone, PartialEq)]
enum Type{
    Number,
    Decimal,
    String,
}

#[derive(Debug, Clone, PartialEq)]
enum Symbol{
    StartParenth,
    EndParenth,
    StartBrack,
    EndBrack,
    Assign,
    Comparison,
    LessThen,
    MoreThen,
    LessOrEqual,
    GreaterOrEqual,
    NotEqual,
    EndStatement,
    Colon,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
enum Literal{
    Number(i64),
    Decimal(f64),
    Alphabetic(String),
}

#[derive(Debug, Clone, PartialEq)]
enum Token{
    Keyword(Keyword),
    Type(Type),
    Symbol(Symbol),
    Identifier(String),
    Literal(Literal),
}

//===========Lexer==============
fn lex(source: &str)->Vec<Token>{
    let chars:Vec<char>= source.chars().collect();
    let mut tokens=Vec::new();
    let mut index=0;

    while index<chars.len(){
        let c =chars[index];

        if c.is_whitespace(){
            index+=1;
            continue;
        }
        match c{
            '('=>{
                tokens.push(Token::Symbol(Symbol::StartParenth));
                index+=1;
             },
            ')'=>{
                tokens.push(Token::Symbol(Symbol::EndParenth));
                index+=1;
            },
            '{'=>{
                tokens.push(Token::Symbol(Symbol::StartBrack));
                index+=1;
            },
            '}'=>{
                tokens.push(Token::Symbol(Symbol::EndBrack));
                index+=1;
            },
            '='=>{
                match chars.get(index+1){
                    Some('=')=>{
                        tokens.push(Token::Symbol(Symbol::Comparison));
                        index+=2;
                    },
                    _=>{
                        tokens.push(Token::Symbol(Symbol::Assign));
                        index+=1;
                    }
                }
            }
            '<'=>{
                match chars.get(index+1){
                    Some('=')=>{
                        tokens.push(Token::Symbol(Symbol::LessOrEqual));
                        index+=2;
                    },
                    _=>{
                        tokens.push(Token::Symbol(Symbol::LessThen));
                        index+=1;
                    }
                }
            }
            '>'=>{
                match chars.get(index+1){
                    Some('=')=>{
                        tokens.push(Token::Symbol(Symbol::GreaterOrEqual));
                        index+=2;
                    },
                    _=>{
                        tokens.push(Token::Symbol(Symbol::MoreThen));
                        index+=1;
                    }
                }
            }
            '!'=>{
                match chars.get(index+1){
                    Some('=')=>{
                        tokens.push(Token::Symbol(Symbol::NotEqual));
                        index+=2;
                    },
                    _=>{panic!("Unexpected Character '!' at position {}", index)},
                }
            }
            ';'=>{
                tokens.push(Token::Symbol(Symbol::EndStatement));
                index+=1;
            }
            ':'=>{
                tokens.push(Token::Symbol(Symbol::Colon));
                index+=1;
            }
            'a'..='z'|'A'..='Z'=>{
                let mut text=String::new();
                let mut j=index;

                while j<chars.len() && chars[j].is_alphanumeric(){
                    text.push(chars[j]);
                    j+=1;
                }
                match text.as_str(){
                    "Func"=>tokens.push(Token::Keyword(Keyword::Func)),
                    "If"=>tokens.push(Token::Keyword(Keyword::If)),
                    "While"=>tokens.push(Token::Keyword(Keyword::While)),
                    "Otherwise"=>tokens.push(Token::Keyword(Keyword::Otherwise)),
                    "End"=>tokens.push(Token::Keyword(Keyword::End)),
                    "Continue"=>tokens.push(Token::Keyword(Keyword::Continue)),
                    "As"=>tokens.push(Token::Keyword(Keyword::As)),
                    "Return"=>tokens.push(Token::Keyword(Keyword::Return)),
                    "Number"=>tokens.push(Token::Type(Type::Number)),
                    "Decimal"=>tokens.push(Token::Type(Type::Decimal)),
                    "String"=>tokens.push(Token::Type(Type::String)),
                    _=>tokens.push(Token::Identifier(text)),
                }
                index=j;
            }
            '0'..='9'=>{
                let mut digit=String::new();
                let mut j=index;
                let mut seen_dot=false;
                
                while j<chars.len() && (chars[j].is_ascii_digit()||(chars[j]=='.' &&!seen_dot)){
                    if chars[j]=='.'{
                        seen_dot=true;
                    }
                    digit.push(chars[j]);
                    j+=1;
                }
                if digit.contains('.'){
                    let value=digit.parse::<f64>().unwrap();
                    tokens.push(Token::Literal(Literal::Decimal(value)));
                    }else {
                    let value= digit.parse::<i64>().unwrap();
                    tokens.push(Token::Literal(Literal::Number(value)));
                    
                }
                index=j;
            }
            '"'=>{
                let mut phrase= String::new();
                let mut j=index+1;

                while j<chars.len() && chars[j]!='"'{
                    phrase.push(chars[j]);
                    j+=1;
                }
                match chars.get(j){
                    Some('"')=>{
                        tokens.push(Token::Literal(Literal::Alphabetic(phrase)));
                        index=j+1;
                        }
                    _=>{
                    panic!("expected Closing qoute!");
                    }
                }       
            }
           '+'=>{
               tokens.push(Token::Symbol(Symbol::Add));
               index+=1;
               }
           '-'=>{
               tokens.push(Token::Symbol(Symbol::Subtract));
               index+=1;
               }
           '*'=>{
               tokens.push(Token::Symbol(Symbol::Multiply));
               index+=1;
               }
           '/'=>{
               tokens.push(Token::Symbol(Symbol::Divide));
               index+=1;
               }     
           _=>{
               panic!("Unhandled Character '{}', at position {}- Not implimented yet!",c,index);
           } 
        }
        
        
    }
    return tokens;
}

//=====================Parser====================================
#[derive(Debug)]
enum Node{
    Number(i64),
    Decimal(f64),
    Alphabetic(String),
    Identifier(String),
    VariableDeclaration{
        name:String,
        var_type: Type,
        value: Box<Node>
    },
    If{
        condition: Box<Node>,
        then_branch: Vec<Node>,
        other_wise:Option<Vec<Node>>,
    },
    While{
        condition:Box<Node>,
        body:Vec<Node>,
    },
    BinaryOp{
        left_side:Box<Node>,
        operator: Symbol,
        right_side: Box<Node>,
    },
    FunctionCall{
        name: String,
        arguments: Vec<Node>,
    },
    FunctionDef{
        name:String,
        return_type:Type,
        body:Vec<Node>,
    },
    End,
    Continue,
    Return{
       value:Box<Node>,
    },
    Assignment{
        name:String,
        value:Box<Node>,
    }
}
 struct Parser{
     tokens:Vec<Token>,
     pos:usize,
 }

impl Parser{
    fn peek(&self)-> Option<&Token>{
        self.tokens.get(self.pos)
    }
    fn advance(&mut self)->Option<Token>{
        let current_token= self.tokens.get(self.pos).cloned();
        self.pos+=1;
        current_token
        }
    fn expect(&mut self,expected: Token)->Result<(), String>{
        match self.peek(){
            Some(actual) if *actual==expected=>{
                self.advance();
                Ok(())
            }
            Some(actual)=>{
                Err(format!("Expected '{:?}', but got '{:?},",expected, actual))
            }
            None=>{
                Err(format!("Expected '{:?}',but got Nothing",expected))
            }
        }
    }
    fn parse_primary(&mut self)->Result<Node, String> {
        match self.advance(){
            Some(Token::Literal(Literal::Number(n)))=>{
               Ok(Node::Number(n))
            }
            Some(Token::Literal(Literal::Decimal(n)))=>{
                Ok(Node::Decimal(n))
            }
            Some(Token::Literal(Literal::Alphabetic(s)))=>{
               Ok(Node::Alphabetic(s))
            }
            Some(Token::Identifier(name))=>{
                if self.peek()==Some(&Token::Symbol(Symbol::StartParenth)){
                    self.parse_call(name)
                } else {
                    Ok(Node::Identifier(name))
                }
            }
            Some(other)=>{
                    Err(format!("Unexpected token in expression: {:?}",other))
            }
            None=>Err("Unexpected token".to_string())
        }
    }
    fn is_operation(sym: &Symbol)->bool{
        matches!(sym,Symbol::Comparison|Symbol::LessThen|Symbol::MoreThen|Symbol::LessOrEqual
        |Symbol::GreaterOrEqual|Symbol::NotEqual|Symbol::Add|Symbol::Subtract|Symbol::Multiply
        |Symbol::Divide)
    }
    fn parse_expression(&mut self)->Result<Node, String>{
        let left=self.parse_primary()?;

        match self.peek(){
            Some(Token::Symbol(op)) if Parser::is_operation(op)=>{
                let operator= match self.advance(){
                    Some(Token::Symbol(s))=>s,
                    _=> return Err("Parser:Expected an Operator".to_string()),
                };
                let right=self.parse_primary()?;
                Ok(Node::BinaryOp{left_side: Box::new(left),operator: operator,right_side: Box::new(right) })
            }
            _=>{
               Ok(left)
            }
        }
    }
    fn parse_statement(&mut self)->Result<Node, String>{
        match self.peek(){
            Some(Token::Keyword(Keyword::If))=>{
                self.parse_if()
            }
            Some(Token::Keyword(Keyword::While))=>{
                self.parse_while()
            }
            Some(Token::Keyword(Keyword::Return))=>{
                self.parse_return()
            }
            Some(Token::Keyword(Keyword::End))=>{
                self.advance();
                self.expect(Token::Symbol(Symbol::EndStatement))?;
                Ok(Node::End)    
            }
            Some(Token::Keyword(Keyword::Continue))=>{
                self.advance();
                self.expect(Token::Symbol(Symbol::EndStatement))?;
                Ok(Node::Continue)
            }
            Some(Token::Identifier(_))=>{
                let name= match self.advance(){
                   Some(Token::Identifier(n))=>n,
                   _=> unreachable!(), 
                };
                self.parse_identifier(name)
            }
            _=>Err("Parser Error:parse_statement could not recoginze token!".to_string())
        }
    }
    fn parse_return(&mut self)->Result<Node, String>{
        self.expect(Token::Keyword(Keyword::Return))?;
        let the_value=self.parse_expression();
        self.expect(Token::Symbol(Symbol::EndStatement))?;
        Ok(Node::Return{value: Box::new(the_value?)})
    }
    fn parse_if(&mut self)->Result<Node, String>{
        self.expect(Token::Keyword(Keyword::If))?;
        self.expect(Token::Symbol(Symbol::StartParenth))?;
        let condition=self.parse_expression()?;
        self.expect(Token::Symbol(Symbol::EndParenth))?;
        self.expect(Token::Symbol(Symbol::StartBrack))?;
        let mut then_branch= Vec::new();
        while self.peek()!=Some(&Token::Symbol(Symbol::EndBrack)){
            then_branch.push(self.parse_statement()?);
        }
        self.expect(Token::Symbol(Symbol::EndBrack))?;
        let other_wise: Option<Vec<Node>>;
        if self.peek()==Some(&Token::Keyword(Keyword::Otherwise)){
            self.advance();
            self.expect(Token::Symbol(Symbol::StartBrack))?;
            let mut statement=Vec::new();
            while self.peek()!=Some(&Token::Symbol(Symbol::EndBrack)){
                statement.push(self.parse_statement()?);
            }
            other_wise=Some(statement);
            self.expect(Token::Symbol(Symbol::EndBrack))?;
        } else{
                    other_wise=None;
            }
        
        Ok(Node::If{condition: Box::new(condition),then_branch,other_wise})
        }
    fn parse_while(&mut self)->Result<Node, String>{
        self.expect(Token::Keyword(Keyword::While))?;
        self.expect(Token::Symbol(Symbol::StartParenth))?;
        let condition= self.parse_expression()?;
        self.expect(Token::Symbol(Symbol::EndParenth))?;
        self.expect(Token::Symbol(Symbol::StartBrack))?;
        let mut body= Vec::new();
        while self.peek()!=Some(&Token::Symbol(Symbol::EndBrack)){
            body.push(self.parse_statement()?);
        }
        self.expect(Token::Symbol(Symbol::EndBrack))?;
        Ok(Node::While{condition: Box::new(condition),body})   
    }
    fn peek_ahead(&self,offset:usize)->Option<&Token>{
        self.tokens.get(self.pos+offset)
    }
    fn parse_identifier(&mut self,name:String)->Result<Node, String>{
        match self.peek(){
            Some(Token::Keyword(Keyword::As))=>{
                self.parse_declaration(name)
            }
            Some(Token::Symbol(Symbol::Assign))=>{
                self.parse_assignment(name)
            }
            Some(Token::Symbol(Symbol::StartParenth))=>{
                self.parse_call_statement(name)
            }
            _=>Err("Parser Error:Parse_indentifier: Code not written for symbol after Inderifer".to_string())
        }
    }
    fn parse_declaration(&mut self,name:String)->Result<Node,String>{
        self.advance();
        let var_type:Type;
        match self.peek(){
            Some(Token::Type(Type::Number))=>{
                self.advance();
                self.expect(Token::Symbol(Symbol::Assign))?;
                var_type=Type::Number;
                let node=self.parse_expression()?;
                self.expect(Token::Symbol(Symbol::EndStatement))?;
                Ok(Node::VariableDeclaration{name,var_type,value:Box::new(node)})
                
            }
            Some(Token::Type(Type::Decimal))=>{
                self.advance();
                self.expect(Token::Symbol(Symbol::Assign))?;
                var_type=Type::Decimal;
                let node=self.parse_expression()?;
                self.expect(Token::Symbol(Symbol::EndStatement))?;
                Ok(Node::VariableDeclaration{name,var_type, value:Box::new(node)})
            }
            Some(Token::Type(Type::String))=>{
                self.advance();
                self.expect(Token::Symbol(Symbol::Assign))?;
                var_type=Type::String;
                let node=self.parse_expression()?;
                self.expect(Token::Symbol(Symbol::EndStatement))?;
                Ok(Node::VariableDeclaration{name,var_type,value:Box::new(node)})
            }
            _=>Err("Parser Error: parse_declaration failed to recognize declaration token".to_string())
        }
    }
    fn parse_assignment(&mut self,name:String)->Result<Node, String>{
        self.expect(Token::Symbol(Symbol::Assign))?;
        let value=self.parse_expression()?;
        self.expect(Token::Symbol(Symbol::EndStatement))?;
        Ok(Node::Assignment{name,value:Box::new(value)})
    }   
    fn parse_call(&mut self, name:String)->Result<Node, String>{
        self.expect(Token::Symbol(Symbol::StartParenth))?;

        let mut arguments= Vec::new();
        if self.peek() != Some(&Token::Symbol(Symbol::EndParenth)){
            let arg=self.parse_expression()?;
            arguments.push(arg);
        }
        self.expect(Token::Symbol(Symbol::EndParenth))?;
        Ok(Node::FunctionCall{name, arguments})
    }
    fn parse_call_statement(&mut self, name: String)-> Result<Node, String>{
        let call=self.parse_call(name)?;
        self.expect(Token::Symbol(Symbol::EndStatement))?;
        Ok(call)
    }       
    fn  parse_function(&mut self)-> Result<Node, String>{
        self.expect(Token::Keyword(Keyword::Func))?;
        let name=match self.advance(){
            Some(Token::Identifier(n))=>n,
            _=> return Err("Parser Error: parse function could not recognize token".to_string())
        };
        self.expect(Token::Symbol(Symbol::StartParenth))?;
        self.expect(Token::Symbol(Symbol::EndParenth))?;
        self.expect(Token::Symbol(Symbol::Colon))?;
        let return_type=match self.advance(){
            Some(Token::Type(t))=>t,
            _=> return Err("Parser Error: parse function couldnt recognize return type".to_string())
        };
        self.expect(Token::Symbol(Symbol::StartBrack))?;
        let mut body= Vec::new();
        let mut safety=0;
        while self.peek() != Some(&Token::Symbol(Symbol::EndBrack)){
            safety+=1;
            if safety>1000{
                panic!("Stuck! pos={}, current token{:?}", self.pos, self.peek());
            }
            body.push(self.parse_statement()?);
        };
        self.expect(Token::Symbol(Symbol::EndBrack))?;
        Ok(Node::FunctionDef{ name,return_type,body })
        }
    
    fn parse_program(&mut self)-> Result<Vec<Node>,String>{
        let mut functions=Vec::new();
        while self.peek().is_some(){
            functions.push(self.parse_function()?);
        }
        Ok(functions)
    }
    fn parse(tokens: Vec<Token>)->Result<Vec<Node>, String>{
        let mut parser= Parser{tokens, pos:0};
        parser.parse_program()
    }
} 
//===================Code Generation======================

use std::collections::HashMap;

fn gen_value(node: &Node, variables: &HashMap<String, i32>,reg: &str)->String{
    match node{
        Node::Number(n)=>format!("    mov {},  {}\n",reg ,n),
        Node::Identifier(name)=>{
            match variables.get(name){
                Some(offset)=> format!("   mov {}, [rbp{}]\n",reg, offset),
                None=>panic!("Variable {} not found",name),
            }
        }
        Node::BinaryOp{left_side, operator, right_side}=>{
            let mut result= String::new();
            result.push_str(&gen_value(left_side, variables, "rax"));
            result.push_str(&gen_value(right_side, variables, "rbx"));
            match operator{
                Symbol::Add=>result.push_str("    add rax, rbx\n"),
                _=>{}
            }
            if reg!="rax"{
                result.push_str(&format!("     mov {},   rax\n",reg));
            }
            result
        }
        
        _=> panic!("gen_value unsupported type"),
    }
}
fn gen_condition(condition: &Node, variables: &HashMap<String, i32>,jump_to_if_false: &str)-> String{
    match condition{
        Node::BinaryOp{left_side, operator, right_side}=>{
            let mut result=String::new();
            result.push_str(&gen_value(left_side, variables, "rax"));
            result.push_str(&gen_value(right_side, variables, "rbx"));
            result.push_str("     cmp rax, rbx\n");
            let jump_inst= match operator{
                Symbol::Comparison=>"jne",
                Symbol::NotEqual=>"je",
                Symbol::LessThen=>"jge",
                Symbol::MoreThen=>"jle",
                Symbol::LessOrEqual=>"jg",
                Symbol::GreaterOrEqual=>"jl",
                _=>panic!("Not a comparison operator"),
            };
            result.push_str(&format!("    {} {}\n",jump_inst, jump_to_if_false));
            result
        }
        _=> panic!("Gen_condition expected a comparision!"),
    }
}

fn gen_statement(stmt: &Node, variables: &mut HashMap<String, i32>, offset: &mut i32, label_counter: &mut i32) -> String {
    let mut asm = String::new();
    match stmt {
        Node::Return { value } => {
            match &**value {
                Node::Number(n) => {
                    asm.push_str(&format!("    mov rax, {}\n", n));
                }
                Node::Identifier(name) => {
                    match variables.get(name) {
                        Some(off) => {
                            asm.push_str(&format!("    mov rax, [rbp{}]\n", off));
                        }
                        None => {
                            panic!("Variable {} not found in hashmap!", name);
                        }
                    }
                }
                _ => {}
            }
        }
        Node::VariableDeclaration { name, var_type, value } => {
            asm.push_str(&gen_value(value, variables, "rax"));
            let var_offset= match variables.get(name){
                Some(existing)=>*existing,
                None=>{
                    let new_offset=*offset*-8;
                    variables.insert(name.clone(), new_offset);
                    *offset+=1;
                    new_offset
                }
            };
            asm.push_str(&format!("    mov [rbp{}],   rax\n",var_offset));
         }
        Node::BinaryOp { left_side, operator, right_side } => {
            asm.push_str(&gen_value(left_side, variables, "rax"));
            asm.push_str(&gen_value(right_side, variables, "rbx"));

            match operator {
                Symbol::Add => asm.push_str("    add rax, rbx\n"),
                Symbol::Subtract => asm.push_str("    sub rax, rbx\n"),
                Symbol::Multiply => asm.push_str("    imul rax, rbx\n"),
                Symbol::Divide => asm.push_str("    cqo\n    idiv rbx\n"),
                _ => {}
            }
        }
        Node::If { condition, then_branch, other_wise } => {
            let else_label = format!("else_{}", label_counter);
            let end_label = format!("end_{}", label_counter);
            *label_counter += 1;

            asm.push_str(&gen_condition(condition, variables, &else_label));
            for s in then_branch {
                asm.push_str(&gen_statement(s, variables, offset, label_counter));
            }

            asm.push_str(&format!("    jmp {}\n", end_label));
            asm.push_str(&format!("{}:\n", else_label));

            match other_wise {
                Some(statements) => {
                    for s in statements {
                        asm.push_str(&gen_statement(s, variables, offset, label_counter));
                    }
                }
                None => {}
            }
            asm.push_str(&format!("{}:\n", end_label));
        }
        Node::While{condition, body}=>{
            let loop_start= format!("loop_start_{}",label_counter);
            let loop_end= format!("loop_end_{}",label_counter);
            *label_counter+=1;

            asm.push_str(&format!("{}:\n",loop_start));
            asm.push_str(&gen_condition(condition, variables, &loop_end));
            for s in body{
                asm.push_str(&gen_statement(s, variables, offset, label_counter));
            }
            asm.push_str(&format!(" jmp  {}\n",loop_start));
            asm.push_str(&format!("{}:\n",loop_end));
        }
        Node::Assignment{name, value}=>{
            asm.push_str(&gen_value(value, variables, "rax"));
            let var_offset=match variables.get(name){
                Some(exisiting)=>*exisiting,
                None=> panic!(" Assignemnt to undeclared variable: {}",name),
            };
            asm.push_str(&format!("    mov [rbp{}],   rax\n", var_offset));
        }
        _ => {}
    }
    asm
}

fn code_gen(program: Vec<Node>) -> String {
    let mut asm = String::from("global _start\n\n_start:\n");
    for func in program {
        match func {
            Node::FunctionDef { name, return_type, body } => {
                asm.push_str("    push rbp\n");
                asm.push_str("    mov rbp, rsp\n\n");
                let mut variables: HashMap<String, i32> = HashMap::new();
                let mut offset = 1;
                let mut label_counter = 1;
                for stmt in &body {
                    asm.push_str(&gen_statement(stmt, &mut variables, &mut offset, &mut label_counter));
                }


                asm.push_str("\n");
                asm.push_str("    mov rsp, rbp\n");
                asm.push_str("    pop rbp\n");
                asm.push_str("\n");
                asm.push_str("    mov rdi, rax\n");
                asm.push_str("    mov rax, 60\n");
                asm.push_str("    syscall\n");
            }
            _=>{}
            
        }
    }
    asm
}




fn main()->Result<(), String>{
    //Program goes here!!
    let program= "Func main():Number{x As Number=0; While(x<3){x=x+1;} Return x; }";
    let tokens=lex(program);
    let mut parser= Parser{tokens, pos:0};
    let tree=parser.parse_program();
    let file=code_gen(tree?);
    std::fs::write("main1.asm",file);
    Ok(())    
}
