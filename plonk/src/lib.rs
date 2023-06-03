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

use std::collections::HashSet;

struct Program {
    inputs: Vec<i32>,
    equation: String,
}

impl Program {
    fn new(inputs :Vec<i32>, equation :&str) -> Self {
        let char_set : HashSet<char> = equation.chars().into_iter().collect();
        let mut char_counter = 0;
        for c in char_set {
            if c.is_alphabetic() {
                char_counter += 1;
            }
        }
        assert!(char_counter == inputs.len(), "Number of inputs must match number of variables in equation");

        Self {
            inputs,
            equation: equation.to_string(),
        }
    }
}

const PROGRAM: &str = "e * x + x - 1";

#[derive(Debug)]
struct QGate(i32, i32, i32, i32, i32);

impl QGate {
    fn new_mul() -> Self {
        Self(0, 0, 1, -1, 0)
    }

    fn new_add() -> Self {
        Self(1, 1, 0, -1, 0)
    }
}

#[derive(Debug)]
enum GateType {
    ADDITION,
    MUL,
}

#[derive(Debug)]
struct Gate {
    gate: (String, String, String),
    gateType: GateType,
}

impl Gate {
    const ADD: char = '+';
    const MINUS: char = '-';
    const MUL: char = '*';

    fn new_gates(program: &str) -> Vec<Self> {
        let mut v_matrix: Vec<Gate> = Vec::new();
        let program = program.replace(" ", "");
        let chars: Vec<char> = program.chars().collect();

        // itterative over all characters, where an operator is found, a new gate is created
        for i in 0..chars.len() {
            if chars[i] == Self::ADD || chars[i] == Self::MINUS || chars[i] == Self::MUL {
                // build l, r, o nodes
                let l;
                let r = chars[i + 1].to_string();
                let o = format!("GO:{}", v_matrix.len());

                if v_matrix.len() == 0 {
                    // if matrix is empty, l is the first character
                    l = chars[i - 1].to_string();
                } else {
                    // else l is the output of the previous gate
                    l = v_matrix[v_matrix.len() - 1].gate.2.clone();
                }

                match chars[i] {
                    MUL => v_matrix.push(Gate {
                        gate: (l, r, o),
                        gateType: GateType::MUL,
                    }),
                    _ => v_matrix.push(Gate {
                        gate: (l, r, o),
                        gateType: GateType::ADDITION,
                    }),
                }
            }
        }
        v_matrix
    }
}

#[derive(Debug)]
struct Trace(i32, i32, i32);

impl Trace {}

fn eval_q_row(gate: QGate, trace: Trace) -> i32 {
    // Ai(QL)i + Bi(QR)i + AiBiQm + Ci(QO)i + QCi
    trace.0 * gate.0 + trace.1 * gate.1 + trace.0 * trace.1 * gate.2 + trace.2 * gate.3 + gate.4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_program() {
        let inputs = vec![1, 2];
        let equation = "e * x + x - 1";
        let program = Program::new(inputs, equation);
        assert_eq!(program.inputs, vec![1, 2]);
        assert_eq!(program.equation, "e * x + x - 1");
    }

    #[test]
    #[allow(unconditional_panic)]
    #[should_panic(expected = "Number of inputs must match number of variables in equation")]
    fn test_new_program_panic() {
        let inputs = vec![1, 2];
        let equation = "e * x + x - 1 + y";
        Program::new(inputs, equation);
    }
    
    #[test]
    fn test_eval_add_gate() {
        let gate = QGate::new_add();
        let trace = Trace(1, 2, 3);
        assert_eq!(eval_q_row(gate, trace), 0);
    }

    #[test]
    fn test_eval_mul_gate() {
        let gate = QGate::new_mul();
        let trace = Trace(2, 5, 10);
        assert_eq!(eval_q_row(gate, trace), 0);
    }

    #[test]
    fn test_new_gates() {
        let expected_gates: Vec<Gate> = vec![
            Gate {
                gate: ("e".to_string(), "x".to_string(), "GO:0".to_string()),
                gateType: GateType::MUL,
            },
            Gate {
                gate: ("GO:0".to_string(), "x".to_string(), "GO:1".to_string()),
                gateType: GateType::ADDITION,
            },
            Gate {
                gate: ("GO:1".to_string(), "1".to_string(), "GO:2".to_string()),
                gateType: GateType::ADDITION,
            },
        ];

        assert_eq!(expected_gates[0].gate, Gate::new_gates(PROGRAM)[0].gate);
        assert_eq!(expected_gates[1].gate, Gate::new_gates(PROGRAM)[1].gate);
        assert_eq!(expected_gates[2].gate, Gate::new_gates(PROGRAM)[2].gate);
    }
}
