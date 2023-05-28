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

use polynomial;
use text_colorizer::Colorize;
fn main() {
    println!("Argument System");
    println!("========================");
    // define eval point for protocol
    let x = 23;
    println!("{} selects {} as the evaluation point", "V".blue(), x);

    // generate p(x)
    let poly_of_interest = "x^3-3x^2+2x";
    let (p_eval, p_roots) = polynomial::generate_p_eval_and_roots(poly_of_interest, x);
    println!("{} sets {} as p(x)", "P".green(), poly_of_interest);

    // generate t(x) 
    println!("target polynomial, t(x) = {}", polynomial::format_target_polynomial(&p_roots));
    let t_eval = polynomial::eval_roots(&p_roots, x);

    // generate h(x)
    let h_eval = p_eval / t_eval;
    println!("{} calculates h(x)={}", "P".green(), h_eval);

    // verify argument 
    println!("{} verifying h(x) = p(x)*t(x)", "V".blue());

    if p_eval == h_eval * t_eval {
        println!("Verification successful");
    } else {
        println!("Verification failed");
    }
}
