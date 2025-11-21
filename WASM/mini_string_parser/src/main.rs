enum Operator {
    sum = '+',
    diff = '-',
    mult = '*',
    div = '/'
}

enum Operand {
    str,
    i32,
}

struct Children {
    token: Token,
    child: Option<Children>
}

enum Token {
    Operator,
    Operand,
    Option<Token>,
}

struct Expr {

}

fn tokenize(input: &str) -> Vec<Token> {
    todo!()
}

fn parse(input: &str) -> Expr {
    todo!()
}

fn main() {
    println!("Hello, world!");
}
