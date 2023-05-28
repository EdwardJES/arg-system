use std::str::FromStr;
use rustnomial::{Polynomial, Roots, Evaluable};

// goal h(x) = p(x)/t(x)
// h(x) = qoutient polynomial
// p(x) = polynomial of interest
// t(x) = cofactor polynomial (roots)

// P/V dance
// 1) P/V agree on factors of polynomial of interest, t(r)
// 2) V samples random eval point, evaluates t(r)
// 3) V evaluates t(r) and sends it to P
// 4) P calculates h(r) = p(r)/t(r)
// 5) P evaluates h(r), p(r) and sends to V
// 6) V verifies that p(r) = h(r) * t(r)
pub fn add() -> Result<(), String>{
    // polynomial string
    let poly_str = "x^3-3x^2+2x";
    // evaluation point
    let x = 5;

    // generate p(x)
    let p = Polynomial::<i32>::from_str(poly_str).unwrap();
    let roots = get_roots(p.to_string().as_str()).unwrap();
    // extract roots
    let root_vec : Vec<f64>;
    match roots {
        Roots::ManyRealRoots(mut vec) => {
            // discard last element as that is always 0
            vec.pop();
            root_vec = vec;
        }
        _ => {
            return Err("failed to generate roots".to_string());
        }
    }

    // generate t(x) eval


    return Ok(())
}

// Reminder that slices (regions of an array or vector) and str (region of strings), can be any size hence they must be passed and stored as references
fn get_roots(poly_str : &str) -> Result<Roots<f64>, <Polynomial<f64> as FromStr>::Err>{
    // to get the roots, the polynomial needs to be of the f64 type
    // create the f64 type from the poly string
    let poly_float = Polynomial::<f64>::from_str(poly_str)?;
    return Ok(poly_float.roots())
}

fn eval_roots(roots : &[f64], eval_point : i32) -> i32 {
    
    let eval = roots.iter().map(|x| *x as i32 - eval_point).product();

    return eval
}

fn format_target_polynomial(roots : &[f64]) -> String {
    let substr = "(x-";
    let mut target_poly = String::new();
    for root in roots {
        let root_int = *root as i32;
        target_poly.push_str(format!("{}{})", substr, root_int.to_string().as_str()).as_str())
    }

    // LEARN: can't retrun &str here as its technically returning a reference to a local variable
    // of course this doesn't make sense, we are borrowing a reference to a local variable
    // when the function returns to the current frame, there is no refferent to return the borrow to
    return target_poly
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       add().unwrap()
    }

    #[test]
    fn test_eval_roots() {
        let roots = vec![1.0, 2.0];
        let eval_point = 23;
        assert_eq!(462, eval_roots(&roots, eval_point));
    }

    #[test]
    fn test_format_target_poly() {
        let roots = vec![1.0, 2.0];
        assert_eq!("(x-1)(x-2)".to_string(), format_target_polynomial(&roots))
    }
}
