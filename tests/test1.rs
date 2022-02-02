use RpnCalc1::eval;
//#[cfg(test)]
#[test]
fn inte_test(){
    assert_eq!(eval("5"),5);
}

#[test]
fn inte_test2(){
    assert_eq!(eval("5"),5);
}
/*
mod tests {
    use super::*;

    #[test]
    fn test_ok(){
        assert_eq!(eval("5"),5);
        assert_eq!(eval("5 5 +"),10);
        assert_eq!(eval("5 5 + 10 *"),100);
    }

    #[test]
    fn test_ok(){
        assert_eq!(eval("6 5 +"),11);
    }

    #[test]
    #[should_panic]
    fn test_ng(){
        eval("5 5 ~");
    }
}

*/