use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fmt::Error,
};

#[derive(PartialEq, Debug, Clone)]
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

// Gate is the implementation for generating the V matrix
// and the gates within a circuit.
// The V matrix encodes the wirings of the circuit.
// The gates encodes the types of gates in the circuit (*, +, -)
// Both of these are independent of circuit execution
impl Gate {
    const ADD: char = '+';
    const MINUS: char = '-';
    const MUL: char = '*';

    fn new_gate_matrix(program: &str) -> Vec<Self> {
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
                        gate_type: GateType::MINUS,
                    }),
                    // TODO: add error handling here. When char is not +, *, -
                    Self::ADD => v_matrix.push(Gate {
                        gate: (l, r, o),
                        gate_type: GateType::ADDITION,
                    }),
                    _ => {
                        panic!("unsupported operator");
                    }
                }
            }
        }
        v_matrix
    }

    fn generate_v_matrix(gates: &[Self]) -> Vec<(String, String, String)> {
        gates.iter().map(|gate| gate.gate.clone()).collect()
    }

    fn generate_gates(gates: &[Self]) -> Vec<GateType> {
        gates.iter().map(|gate| gate.gate_type.clone()).collect()
    }
}

#[derive(PartialEq, Debug)]
struct Trace(i32, i32, i32);

// Trace is the implementation that generates the execution trace of the program.
// The execution trace encodes the values at each cell based on the inputs supplied
// to the program.
impl Trace {
    fn new_trace_matrix(inputs: &HashMap<String, i32>, gates: &[Gate]) -> Vec<Self> {
        let mut trace_matrix: Vec<Trace> = Vec::new();

        fn parse_cell(value: &str, inputs: &HashMap<String, i32>) -> i32 {
            if inputs.contains_key(value) {
                *inputs.get(value).unwrap()
            } else {
                value.parse::<i32>().unwrap()
            }
        }

        fn eval_v_row(inputs: [i32; 2], gate: &GateType) -> i32 {
            match gate {
                GateType::MUL => inputs[0] * inputs[1],
                GateType::ADDITION => inputs[0] + inputs[1],
                GateType::MINUS => inputs[0] - inputs[1],
            }
        }

        for i in 0..gates.len() {
            let a: i32;
            let b: i32;
            if i == 0 {
                a = parse_cell(gates[i].gate.0.as_str(), &inputs);
                b = parse_cell(gates[i].gate.1.as_str(), &inputs);
                trace_matrix.push(Trace(a, b, eval_v_row([a, b], &gates[i].gate_type)));
            } else {
                // a is now the output of the previous gate
                a = trace_matrix[i - 1].2;
                b = parse_cell(gates[i].gate.1.as_str(), &inputs);
                trace_matrix.push(Trace(a, b, eval_v_row([a, b], &gates[i].gate_type)));
            }
        }

        trace_matrix
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
pub struct Program {
    v_matrix: Vec<(String, String, String)>,
    gates: Vec<GateType>,
    trace: Vec<Trace>,
    inputs: HashMap<String, i32>,
    equation: String,
}

// Program is a structure will holds all the information to construct the Q, V and trace
impl Program {
    fn new(inputs: HashMap<String, i32>, equation: &str) -> Self {
        let char_set: HashSet<char> = equation.chars().into_iter().collect();

        let mut char_counter = 0;
        // Assert that all characters in the equation are in the input set
        for c in char_set.iter() {
            if c.is_alphabetic() {
                char_counter += 1;
                assert!(
                    inputs.contains_key(&c.to_string()),
                    "all characters in the equation must be in the input set"
                );
            }
        }

        // Assert character count matches input length
        assert!(
            char_counter == inputs.keys().len(),
            "input length and equation variable length must match"
        );

        let gate_matrix = Gate::new_gate_matrix(&equation);

        Self {
            v_matrix: Gate::generate_v_matrix(&gate_matrix),
            gates: Gate::generate_gates(&gate_matrix),
            trace: Trace::new_trace_matrix(&inputs, &gate_matrix),
            inputs,
            equation: equation.to_string(),
        }
    }

    fn evaluate_q_matrix(&self) -> Result<(), String> {
        let mut output: i32 = 0;

        fn eval_q_row(gate: QGate, trace: &Trace) -> i32 {
            // Ai(QL)i + Bi(QR)i + AiBiQm + Ci(QO)i + QCi
            trace.0 * gate.0
                + trace.1 * gate.1
                + trace.0 * trace.1 * gate.2
                + trace.2 * gate.3
                + gate.4
        }

        for i in 0..self.gates.len() {
            // For each row:
            // -> classify gate type, inclusive of constant
            // -> evaluate the row based on the trace and gate type

            // If the gate has a constant e.g
            // L R O => A B C
            // x 1 o => 1 - 0
            // B column becomes 0 and value is encoded in the Qc col of Q matrix
            let has_const = match &self.v_matrix[i] {
                (_, r, _) if r.chars().all(|c| c.is_digit(10)) => true,
                (_, _, _) => false,
            };

            output += match &self.gates[i] {
                GateType::MUL => eval_q_row(QGate::new_mul(), &self.trace[i]),
                GateType::ADDITION => match has_const {
                    true => {
                        let mut q_gate = QGate::new_add();
                        q_gate.4 = self.trace[i].1;
                        let trace = Trace(self.trace[i].0, 0, self.trace[i].2);
                        eval_q_row(q_gate, &trace)
                    }
                    false => eval_q_row(QGate::new_add(), &self.trace[i]),
                },
                _ => match has_const {
                    true => {
                        let q_gate = QGate::new_add();
                        let sub_gate = QGate(q_gate.0, q_gate.1, q_gate.2, q_gate.3, q_gate.4 + -1);
                        let trace = Trace(self.trace[i].0, 0, self.trace[i].2);
                        eval_q_row(sub_gate, &trace)
                    }
                    false => {
                        let trace = Trace(self.trace[i].0, self.trace[i].1 * -1, self.trace[i].2);
                        eval_q_row(QGate::new_add(), &trace)
                    }
                },
            };
        }

        if output == 0 {
            Ok(())
        } else {
            Err("Q matrix evaluation failed".to_string())
        }
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    const equation: &str = "e * x + x - 1";

    fn eval_q_row(gate: QGate, trace: &Trace) -> i32 {
        trace.0 * gate.0 + trace.1 * gate.1 + trace.0 * trace.1 * gate.2 + trace.2 * gate.3 + gate.4
    }

    #[test]
    fn test_eval_add_gate() {
        let gate = QGate::new_add();
        let trace = Trace(1, 2, 3);
        assert_eq!(eval_q_row(gate, &trace), 0);
    }

    #[test]
    fn test_eval_mul_gate() {
        let gate = QGate::new_mul();
        let trace = Trace(2, 5, 10);
        assert_eq!(eval_q_row(gate, &trace), 0);
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

        let output_gates = Gate::new_gate_matrix(equation);
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

        let output_gates = Gate::new_gate_matrix(equation);
        let v_matrix = Gate::generate_v_matrix(&output_gates);
        assert_eq!(expected_v_matrix[0], v_matrix[0]);
        assert_eq!(expected_v_matrix[1], v_matrix[1]);
        assert_eq!(expected_v_matrix[2], v_matrix[2]);
    }

    #[test]
    fn test_generate_trace() {
        let expected_trace = vec![Trace(2, 3, 6), Trace(6, 3, 9), Trace(9, 1, 8)];
        let gates = Gate::new_gate_matrix(equation);
        let mut inputs = HashMap::new();
        inputs.insert("e".to_string(), 2);
        inputs.insert("x".to_string(), 3);

        let trace = Trace::new_trace_matrix(&inputs, &gates);

        assert_eq!(expected_trace[0], trace[0]);
        assert_eq!(expected_trace[1], trace[1]);
        assert_eq!(expected_trace[2], trace[2]);
    }

    #[test]
    fn test_evaluate_q_matrix() {
        let mut inputs = HashMap::new();
        inputs.insert("e".to_string(), 2);
        inputs.insert("x".to_string(), 3);
        let program = Program::new(inputs, equation);

        println!("{:?}", program);

        program.evaluate_q_matrix().unwrap();
    }
}
