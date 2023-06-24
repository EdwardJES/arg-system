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

use std::collections::{HashMap, HashSet};

pub struct Program {
    inputs: HashMap<char, i32>,
    equation: String,
}

impl Program {
    fn new(inputs: HashMap<char, i32>, equation: &str) -> Self {
        let char_set: HashSet<char> = equation.chars().into_iter().collect();

        let mut char_counter = 0;
        // Assert that all characters in the equation are in the input set
        for c in char_set.iter() {
            if c.is_alphabetic() {
                char_counter += 1;
                assert!(
                    inputs.contains_key(c),
                    "all characters in the equation must be in the input set"
                );
            }
        }

        // Assert character count matches input length
        assert!(
            char_counter == inputs.keys().len(),
            "input length and equation variable length must match"
        );

        Self {
            inputs,
            equation: equation.to_string(),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum GateType {
    ADDITION,
    MUL,
    MINUS,
}

#[derive(Debug)]
struct Gate {
    gate: (String, String, String),
    gate_type: GateType,
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
                    Self::MUL => v_matrix.push(Gate {
                        gate: (l, r, o),
                        gate_type: GateType::MUL,
                    }),
                    Self::MINUS => v_matrix.push(Gate {
                        gate: (l, r, o),
                        gate_type: GateType::MINUS
                    }),
                    // TODO: add error handling here. When char is not +, *, -
                    _ => v_matrix.push(Gate {
                        gate: (l, r, o),
                        gate_type: GateType::ADDITION,
                    }),
                }
            }
        }
        v_matrix
    }

    fn generate_v_matrix(gates: &[Self]) -> Vec<(String, String, String)> {
        let mut v_matrix: Vec<(String, String, String)> = Vec::new();
        for gate in gates.iter() {
            v_matrix.push(gate.gate.clone());
        }

        v_matrix
    }
}

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
struct Trace(i32, i32, i32);

impl Trace {
    // new_trace_matrix will take in the inputs to the program
    // and a slice of gates.
    // e.g. inputs: e = 2, x = 3
    // gate: e * x = GO:O, MUL
    // expected trace:
    // A    B   C
    // 2    3   6
    fn new_trace_matrix(inputs : HashMap<String, i32>, gates: &[Gate]) -> Vec<Self> {
        let mut trace_matrix : Vec<Trace> = Vec::new();

        let mut gate_output : i32;
        for i in 0..gates.len() {
            let a : i32;
            let b : i32;
            if i == 0 {
                a = Self::parse_cell(gates[i].gate.0.as_str(), &inputs);
                b = Self::parse_cell(gates[i].gate.1.as_str(), &inputs);
                trace_matrix.push(Trace(a, b, Self::eval_v_row([a, b], &gates[i].gate_type)));
            } else {
                // a is now the output of the previous gate
                a = trace_matrix[i-1].2;
                b = Self::parse_cell(gates[i].gate.1.as_str(), &inputs);
                trace_matrix.push(Trace(a, b, Self::eval_v_row([a, b], &gates[i].gate_type)))
            }
        }

        trace_matrix
    }

    fn parse_cell(value : &str, inputs : &HashMap<String, i32>) -> i32 {
        if inputs.contains_key(value) {
            *inputs.get(value).unwrap()
        } else {
            value.parse::<i32>().unwrap()
        }
    }
    
    fn eval_v_row(inputs : [i32; 2], gate : &GateType) -> i32 {
        match gate {
            GateType::MUL => {
                inputs[0] * inputs[1]
            }
            GateType::ADDITION => {
                inputs[0] + inputs[1]
            },
            GateType::MINUS => {
                inputs[0] - inputs[1]
            }
        }
    }
}

fn eval_q_row(gate: QGate, trace: Trace) -> i32 {
    // Ai(QL)i + Bi(QR)i + AiBiQm + Ci(QO)i + QCi
    trace.0 * gate.0 + trace.1 * gate.1 + trace.0 * trace.1 * gate.2 + trace.2 * gate.3 + gate.4
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRAM: &str = "e * x + x - 1";

    #[test]
    fn test_new_program() {
        let mut inputs = HashMap::new();
        inputs.insert('e', 2);
        inputs.insert('x', 3);
        let expected_inputs = inputs.clone();
        let equation = "e * x + x - 1";
        let program = Program::new(inputs, equation);
        assert_eq!(program.inputs, expected_inputs);
        assert_eq!(program.equation, "e * x + x - 1");
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
                gate_type: GateType::MUL,
            },
            Gate {
                gate: ("GO:0".to_string(), "x".to_string(), "GO:1".to_string()),
                gate_type: GateType::ADDITION,
            },
            Gate {
                gate: ("GO:1".to_string(), "1".to_string(), "GO:2".to_string()),
                gate_type: GateType::MINUS,
            },
        ];

        let output_gates = Gate::new_gates(PROGRAM);
        assert_eq!(expected_gates[0].gate, output_gates[0].gate);
        assert_eq!(expected_gates[0].gate_type, output_gates[0].gate_type);
        assert_eq!(expected_gates[1].gate, output_gates[1].gate);
        assert_eq!(expected_gates[1].gate_type, output_gates[1].gate_type);
        assert_eq!(expected_gates[2].gate, output_gates[2].gate);
        assert_eq!(expected_gates[2].gate_type, output_gates[2].gate_type);
    }

    #[test]
    fn test_new_v_matrix() {
        let expected_v_matrix: Vec<(String, String, String)> = vec![
            ("e".to_string(), "x".to_string(), "GO:0".to_string()),
            ("GO:0".to_string(), "x".to_string(), "GO:1".to_string()),
            ("GO:1".to_string(), "1".to_string(), "GO:2".to_string()),
        ];

        let output_gates = Gate::new_gates(PROGRAM);
        let v_matrix = Gate::generate_v_matrix(&output_gates);
        assert_eq!(expected_v_matrix[0], v_matrix[0]);
        assert_eq!(expected_v_matrix[1], v_matrix[1]);
        assert_eq!(expected_v_matrix[2], v_matrix[2]);
    }

    #[test]
    fn test_generate_trace() {
        let gates = Gate::new_gates(PROGRAM);
        let mut inputs = HashMap::new();
        inputs.insert("e".to_string(), 2);
        inputs.insert("x".to_string(), 3);
        
        let trace = Trace::new_trace_matrix(inputs, &gates);
        println!("{:?}", trace)
    }
}
