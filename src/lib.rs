pub mod module_a;

//pub mod test;
pub fn eval(formula:&str) -> i32 {
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
                    "+" => |x,y|x+y,
                    "-" => |x,y|x-y,
                    "*" => |x,y|x*y,
                    "/" => |x,y|x/y,
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok(){
        assert_eq!(eval("5"),5);
        assert_eq!(eval("5 5 +"),10);
        assert_eq!(eval("5 5 + 10 *"),100);
    }

    #[test]
    fn test_ok2(){
        assert_eq!(eval("6 5 +"),11);
    }

    #[test]
    #[should_panic]
    fn test_ng(){
        eval("5 5 ~");
    }
}
