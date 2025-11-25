#[derive(Debug, Clone, PartialEq)]
enum Token {
    Var(char),
    Num(i64),
    Op(char),
    LParen,
    RParen,
}

#[derive(Debug, PartialEq)]
enum Expr {
    Variable(char),
    Number(i64),
    BinOp(char, Box<Expr>, Box<Expr>),
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(a) = chars.next() {
        match a {
            a if a.is_whitespace() => continue,
            a if a.is_alphabetic() => tokens.push(Token::Var(a)),
            a if a.is_digit(10) => tokens.push(Token::Num(a.to_digit(10).unwrap() as i64)),
            '+' | '-' | '*' | '/' => tokens.push(Token::Op(a)),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            _ => panic!("Неизвестный символ: {}", a),
        }
    }

    tokens
}

fn parse(input: &str) -> Expr {
    let tokens = tokenize(input);
    let (expr, rest) = parse_expr(&tokens, 0);

    if !rest.is_empty() {
        panic!("Лишние токены в конце!");
    }
    expr
}

fn parse_expr(tokens: &[Token], min_prec: u8) -> (Expr, &[Token]) {
    let (mut expr, mut rest) = parse_primary(tokens);



    (expr, rest)
}

fn parse_primary(tokens: &[Token]) -> (Expr, &[Token]) {
    match tokens.first() {
        Some(Token::Var(b)) => (Expr::Variable(*b), &tokens[1..]),
        Some(Token::Num(n)) => (Expr::Number(*n), &tokens[1..]),
        Some(Token::LParen) => {
            let (expr, rest) = parse_expr(&tokens[1..], 0);
            match rest.first() {
                Some(Token:: RParen) => (expr, &rest[1..]),
                _ => panic!("Ожидалась закрывающая скобка")
            }
        }
        _ => panic!("Ожидалось число, переменная или ("),
    }
}

fn main() {
    // 1. Простые атомы
    assert_eq!(parse("x"), Expr::Variable('x'));
    assert_eq!(parse("4"), Expr::Number(4));
    assert_eq!(parse("  3  "), Expr::Number(3));

    // 2. Простые бинарные операции
    assert_eq!(
        parse("a + b"),
        Expr::BinOp('+', Box::new(Expr::Variable('a')), Box::new(Expr::Variable('b')))
    );

    assert_eq!(
        parse("x*y"),
        Expr::BinOp('*', Box::new(Expr::Variable('x')), Box::new(Expr::Variable('y')))
    );

    // 3. Приоритеты
    assert_eq!(
        parse("a + b * c"),
        Expr::BinOp(
            '+',
            Box::new(Expr::Variable('a')),
            Box::new(Expr::BinOp('*', Box::new(Expr::Variable('b')), Box::new(Expr::Variable('c'))))
        )
    );

    assert_eq!(
        parse("a * b + c"),
        Expr::BinOp(
            '+',
            Box::new(Expr::BinOp('*', Box::new(Expr::Variable('a')), Box::new(Expr::Variable('b')))),
            Box::new(Expr::Variable('c'))
        )
    );

    // 4. Скобки меняют приоритет
    assert_eq!(
        parse("(a + b) * c"),
        Expr::BinOp(
            '*',
            Box::new(Expr::BinOp('+', Box::new(Expr::Variable('a')), Box::new(Expr::Variable('b')))),
            Box::new(Expr::Variable('c'))
        )
    );

    // 5. Сложные цепочки
    assert_eq!(
        parse("a + b + c * d * e - f / g"),
        Expr::BinOp(
            '-',
            Box::new(Expr::BinOp(
                '+',
                Box::new(Expr::BinOp(
                    '+',
                    Box::new(Expr::Variable('a')),
                    Box::new(Expr::Variable('b'))
                )),
                Box::new(Expr::BinOp(
                    '*',
                    Box::new(Expr::BinOp('*', Box::new(Expr::Variable('c')), Box::new(Expr::Variable('d')))),
                    Box::new(Expr::Variable('e'))
                ))
            )),
            Box::new(Expr::BinOp(
                '/',
                Box::new(Expr::Variable('f')),
                Box::new(Expr::Variable('g'))
            ))
        )
    );

    // 6. Пробелы везде — не должны влиять
    assert_eq!(parse("  a  +  (  x  *  3  )  "), parse("a+(x*3)"));

    // === Ошибки: должны падать в panic! ===

    macro_rules! should_panic {
        ($input:expr) => {{
            let result = std::panic::catch_unwind(|| parse($input));
            assert!(result.is_err(), "Ожидалась паника на входе: {}", $input);
        }};
    }

    should_panic!("");                    // пустая строка
    should_panic!("a +");                 // незавершённое выражение
    should_panic!("a + b +");             // тоже
    should_panic!("(a + b");              // незакрытая скобка
    should_panic!("a + b)");              // лишняя скобка
    should_panic!("a + * b");             // два оператора подряд
    should_panic!("++a");                 // оператор в начале
    should_panic!("a b");                 // два идентификатора подряд
    should_panic!("123abc");              // число + буква без оператора
    should_panic!("a! + b");              // неизвестный символ
    should_panic!("@");                   // любой мусор
    should_panic!("   ");                 // только пробелы

    println!("Все тесты прошли!");
}
