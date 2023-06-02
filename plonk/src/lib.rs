// Q matrix: encodes the gates of the circuit
// QL | QR | QM | QO | QC

// V matrix: encodes the wires of the circuit
// L | R | O

// PI matrix: public inputs of the circuit
// PI |

// T matrix: trace values of the circuit
// A | B | C

// example problem: e * x + x - 1
// Three gates
// Gate 1:
// e
//   - (*) -> u
// x

// Gate 2:
// u
//  - (+) -> v
// x

// Gate 3:
// v
//   - (-) -> output
// 1

// evaluation:

#[derive(Debug)]
struct Gate(i32, i32, i32, i32, i32);

impl Gate {
    fn new_mul() -> Self {
        Self(0, 0, 1, -1, 0)
    }

    fn new_add() -> Self {
        Self(1, 1, 0, -1, 0)
    }
}

#[derive(Debug)]
struct Trace(i32, i32, i32);

impl Trace {
    const ADD:  &str = "+";
    const MINUS: &str = "-";
    const MUL:   &str = "*";

    fn new(program : &str) -> Self {
        
    }

    fn new_trace(program : &str) -> Vec<Self> {
        
    }
}

fn eval_q_row(gate :Gate, trace : Trace) -> i32 {
    // Ai(QL)i + Bi(QR)i + AiBiQm + Ci(QO)i + QCi
    trace.0*gate.0 + trace.1*gate.1 + trace.0*trace.1*gate.2 + trace.2*gate.3 + gate.4
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_add_gate() {
        let gate = Gate::new_add();
        let trace = Trace(1, 2, 3);
        assert_eq!(eval_q_row(gate, trace), 0);
    }

    #[test]
    fn eval_mul_gate() {
        let gate = Gate::new_mul();
        let trace = Trace(2, 5, 10);
        assert_eq!(eval_q_row(gate, trace), 0);
    }
}
