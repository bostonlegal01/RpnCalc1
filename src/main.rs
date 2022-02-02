fn add(x:i32,y:i32) -> i32 {
    x + y
}
fn sub(x:i32,y:i32) -> i32 {
    x - y
}
fn multi(x:i32,y:i32) -> i32 {
    x * y
}
fn divi(x:i32,y:i32) -> i32 {
    x / y
}
fn eval(formula:&str) -> i32 {
    let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
    let res =eval_inner(&mut tokens);
    res
}
fn eval_inner(tokens:&mut Vec<&str>) -> i32 {
    let mut stack = Vec::new();

    while let Some(token)= tokens.pop(){
        if let Ok(x) = token.parse::<i32>(){
            stack.push(x);
        } else {
            let y = stack.pop().expect("invalid syntax");
            let x = stack.pop().expect("invalid syntax");

            let fnc = match token {
                "+" => add,
                "-" => sub,
                "*" => multi,
                "/" => divi,
                _ => panic!("invalid token"),
            };

            stack.push(fnc(x,y));
            /*

            
            let res = match token {
                "+" => x + y,
                "-" => x - y,
                "*" => x * y,
                "/" => x / y,
                _ => panic!("invalid token"),
            }
            */
            //stack.push(res);
        }
    }

    if stack.len() == 1 {
        stack[0]

    }else {
        panic!("invalid syntax");
    }
}

fn main() {
    let formula = "3 3 + 3 *";
    let res = eval(formula);
    println!("res: {}", res);
}
