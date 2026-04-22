fn eval_formula(formula: &str) {
    let mut stack = Vec::<bool>::with_capacity(formula.len() / 2 - 1);
    let mut fidx = 0;
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
                None => {println!("la stack est vide"); return}
            }
        } else {
            match stack.pop() {
                Some(x) => {a = x},
                None => {println!("la stack est vide"); return}
            }
            match stack.pop() {
                Some(x) => {b = x},
                None => {println!("la stack est vide"); return}
            }

            if formula.as_bytes()[fidx] == b'&' {
                stack.push(a && b);
            } else if formula.as_bytes()[fidx] == b'|' {
                stack.push(a | b);
            } else if formula.as_bytes()[fidx] == b'^' {
                stack.push(a ^ b);
            } else if formula.as_bytes()[fidx] == b'>' {
                stack.push(a > b);
            } else if formula.as_bytes()[fidx] == b'=' {
                stack.push(a == b);
            }
        }
        fidx += 1;
    }
    if stack.len() != 1 {
        println!("il y a un probleme non ?");
        return ;
    }
    match stack.pop() {
        Some(value) => {println!("value: {}", value)},
        None => {println!("il y a un probleme non ?")}
    }
}

fn main() {
/*     eval_formula("11=");
    eval_formula("10=");
    eval_formula("01=");
    eval_formula("00="); */

    eval_formula("11>");
    eval_formula("10>");
    eval_formula("01>");
    eval_formula("00>");
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

}
