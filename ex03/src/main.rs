fn eval_formula(formula: &str) -> bool {
    let mut stack: Vec<bool> = Vec::<bool>::with_capacity(formula.len() / 2);
    let mut fidx: usize = 0;
    let mut a: bool;
    let mut b: bool;

    while fidx < formula.len() {
        if formula.as_bytes()[fidx] == b'0' {
            stack.push(false);
        } else if formula.as_bytes()[fidx] == b'1' {
            stack.push(true);
        } else if formula.as_bytes()[fidx] == b'!' {
            match stack.pop() {
                Some(x) => {stack.push(!x)},
                None => {println!("Formule invalide"); return false}
            }
        } else {
            match stack.pop() {
                Some(x) => {a = x},
                None => {println!("Formule invalide"); return false}
            }
            match stack.pop() {
                Some(x) => {b = x},
                None => {println!("Formule invalide"); return false}
            }

            if formula.as_bytes()[fidx] == b'&' {
                stack.push(a && b);
            } else if formula.as_bytes()[fidx] == b'|' {
                stack.push(a | b);
            } else if formula.as_bytes()[fidx] == b'^' {
                stack.push(a ^ b);
            } else if formula.as_bytes()[fidx] == b'>' {
                stack.push(!b | a);
            } else if formula.as_bytes()[fidx] == b'=' {
                stack.push(a == b);
            }
        }
        fidx += 1;
    }
    if stack.len() != 1 {
        println!("Formule invalide");
        return false
    }
    match stack.pop() {
        Some(value) => {return value},
        None => {println!("Formule invalide"); return false}
    }
}

fn main() {
/*     eval_formula("11=");
    eval_formula("10=");
    eval_formula("01=");
    eval_formula("00="); */

    println!("{}", eval_formula("11>"));
    println!("{}", eval_formula("10>"));
    println!("{}", eval_formula("01>"));
    println!("{}", eval_formula("00>"));
/* 
    eval_formula("11^");
    eval_formula("10^");
    eval_formula("01^");
    eval_formula("00^");
    
    eval_formula("11|");
    eval_formula("10|");
    eval_formula("01|");
    eval_formula("00|");
        
    eval_formula("11&");
    eval_formula("10&");
    eval_formula("01&");
    eval_formula("00&");*/

    eval_formula("10!");
    eval_formula("00!");
    println!("{}", eval_formula("0!0!0!0!|||"));
    println!("{}", eval_formula("0000!!!!|||"));


}
