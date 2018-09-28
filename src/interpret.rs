use ast::Expr;
use ast::Expr::*;
use std::str::FromStr;

// XXX When to de-sugar "+" into "add"?


pub fn eval_scal(e: &Expr) -> f64 {
    match e {
        FnApp(fname, args) => {
            let eargs: Vec<f64> = args.into_iter()
                .map(|e| { eval_scal(e) })
                .collect();
            match fname.as_str() {
                "+" => eargs[0] + eargs[1],
                "-" => eargs[0] - eargs[1],
                "*" => eargs[0] * eargs[1],
                "/" => eargs[0] / eargs[1],
                "sassy" => eargs[0] * eargs[1] + 1.0,
                _ => panic!("Operator {} not handled!", fname),
            }
        }
        Int(i) => f64::from(*i),
        Real(s) => f64::from_str(s).unwrap(),
        _ => panic!("haha"),
    }
}
