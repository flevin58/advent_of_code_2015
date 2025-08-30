#![allow(dead_code)]

macro_rules! signal_or_wire {
    ($x:expr) => {
        if let Ok(signal) = $x.parse::<u16>() {
            Type::Signal(signal)
        } else {
            Type::Wire($x.to_string())
        }
    };
}

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

#[derive(Clone, PartialEq)]
pub enum Type {
    Wire(String),
    Signal(u16),
    Gate(String),
    None,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Wire(name) => write!(f, "{name}"),
            Type::Signal(value) => write!(f, "{value}"),
            Type::Gate(name) => write!(f, "{name}"),
            Type::None => write!(f, "None"),
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Wire(name) => write!(f, "Wire({name})"),
            Type::Signal(value) => write!(f, "Signal({value})"),
            Type::Gate(name) => write!(f, "Gate({name})"),
            Type::None => write!(f, "None"),
        }
    }
}

impl Type {
    pub fn is_signal(&self) -> bool {
        matches!(self, Type::Signal(_))
    }

    pub fn is_wire(&self) -> bool {
        matches!(self, Type::Wire(_))
    }

    pub fn is_gate(&self) -> bool {
        matches!(self, Type::Gate(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Type::None)
    }

    pub fn is_known(&self) -> bool {
        !matches!(self, Type::None | Type::Wire(_))
    }

    pub fn get_signal_value(&self) -> Option<u16> {
        if let Type::Signal(value) = self {
            Some(*value)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Component {
    ctype: Type,
    left: Type,
    right: Type,
}

impl Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.ctype {
            Type::Gate(name) => {
                if self.left != Type::None {
                    write!(f, "{:?} ", self.left)?;
                }
                write!(f, "{name} ")?;
                if self.right != Type::None {
                    write!(f, "{:?} ", self.right)?;
                }
            }
            Type::Signal(value) => {
                write!(f, "{:?}", value)?;
            }
            Type::Wire(name) => {
                write!(f, "{:?}", name)?;
            }
            Type::None => {
                write!(f, "None")?;
            }
        }
        Ok(())
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.ctype {
            Type::Gate(name) => {
                if self.left != Type::None {
                    write!(f, "{} ", self.left)?;
                }
                write!(f, "{name} ")?;
                if self.right != Type::None {
                    write!(f, "{} ", self.right)?;
                }
            }
            Type::Signal(value) => {
                write!(f, "{value}")?;
            }
            Type::Wire(name) => {
                write!(f, "{name}")?;
            }
            _ => {
                write!(f, "Invalid Component")?;
            }
        }
        Ok(())
    }
}

impl Component {
    pub fn from_str(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts.len() {
            1 => {
                // When length is 1, it's a direct signal or wire assignment
                Self {
                    ctype: signal_or_wire!(parts[0]),
                    left: Type::None,
                    right: Type::None,
                }
            }

            2 => {
                // When length is 2, it must be a NOT gate
                assert_eq!(parts[0], "NOT", "Invalid gate format");
                Self {
                    ctype: Type::Gate("NOT".to_string()),
                    left: Type::None,
                    right: signal_or_wire!(parts[1]),
                }
            }

            3 => {
                // When length is 3, it must be a binary gate
                Self {
                    ctype: Type::Gate(parts[1].to_string()),
                    left: signal_or_wire!(parts[0]),
                    right: signal_or_wire!(parts[2]),
                }
            }

            _ => {
                // Any other length is invalid
                panic!("Invalid component line: {}", line);
            }
        }
    }

    pub fn is_gate(&self, gate_name: &str) -> bool {
        if let Type::Gate(name) = &self.ctype {
            name == gate_name
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct Wires(HashMap<String, Component>);

impl Debug for Wires {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Wires:")?;
        for (wname, wtype) in &self.0 {
            writeln!(f, "{} = {:?}", wname, wtype)?;
        }
        Ok(())
    }
}

impl Display for Wires {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Wires:")?;
        for (wname, wtype) in &self.0 {
            writeln!(f, "{:<2} = {}", wname, wtype)?;
        }
        Ok(())
    }
}

impl Wires {
    fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get_wire(&self, name: &str) -> Option<&Component> {
        self.0.get(name)
    }

    pub fn set_wire_signal(&mut self, name: &str, value: u16) {
        self.0.insert(
            name.to_string(),
            Component {
                ctype: Type::Signal(value),
                left: Type::None,
                right: Type::None,
            },
        );
    }

    pub fn from_input_data(input_data: &str) -> Self {
        let mut wires = Self::new();
        // Parse each line into a Component and add it to the Wires hashmap
        for line in input_data.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split(" -> ").map(|item| item.trim()).collect();
            let component = Component::from_str(parts[0]);
            wires.0.insert(parts[1].to_string(), component);
        }
        wires
    }

    pub fn all_known(&self) -> bool {
        self.0.values().all(|c| matches!(c.ctype, Type::Signal(_)))
    }

    pub fn compute(&mut self) {
        while !self.all_known() {
            self.0.clone().iter().for_each(|(k, v)| {
                match v.ctype {
                    Type::Signal(_) => {}
                    Type::Wire(ref wname) => {
                        if let Some(wire_comp) = self.0.get(wname) {
                            if let Type::Signal(value) = wire_comp.ctype {
                                // Update the component to a signal
                                let comp = self.0.get_mut(k).unwrap();
                                comp.ctype = Type::Signal(value);
                                comp.left = Type::None;
                                comp.right = Type::None;
                            }
                        }
                    }
                    Type::Gate(ref gate_name) => match gate_name.as_str() {
                        "AND" | "OR" | "LSHIFT" | "RSHIFT" => {
                            let left_signal = match &v.left {
                                Type::Signal(val) => Some(*val),
                                Type::Wire(wname) => self
                                    .0
                                    .get(wname)
                                    .and_then(|comp| comp.ctype.get_signal_value()),
                                _ => None,
                            };
                            let right_signal = match &v.right {
                                Type::Signal(val) => Some(*val),
                                Type::Wire(wname) => self
                                    .0
                                    .get(wname)
                                    .and_then(|comp| comp.ctype.get_signal_value()),
                                _ => None,
                            };
                            if let (Some(left_val), Some(right_val)) = (left_signal, right_signal) {
                                let result = match gate_name.as_str() {
                                    "AND" => left_val & right_val,
                                    "OR" => left_val | right_val,
                                    "LSHIFT" => left_val << right_val,
                                    "RSHIFT" => left_val >> right_val,
                                    _ => unreachable!(),
                                };
                                self.0.insert(
                                    k.clone(),
                                    Component {
                                        ctype: Type::Signal(result),
                                        left: Type::None,
                                        right: Type::None,
                                    },
                                );
                            }
                        }
                        "NOT" => {
                            let right_signal = match &v.right {
                                Type::Signal(val) => Some(*val),
                                Type::Wire(wname) => self
                                    .0
                                    .get(wname)
                                    .and_then(|comp| comp.ctype.get_signal_value()),
                                _ => None,
                            };
                            if let Some(right_val) = right_signal {
                                let result = !right_val;
                                // Update the component to a signal
                                self.0.insert(
                                    k.clone(),
                                    Component {
                                        ctype: Type::Signal(result),
                                        left: Type::None,
                                        right: Type::None,
                                    },
                                );
                            }
                        }
                        _ => {
                            println!("Unknown gate type: {}", gate_name);
                        }
                    },
                    Type::None => {}
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_signal_type() {
        use super::Type;

        let signal = Type::Signal(123);
        let wire = Type::Wire("x".to_string());
        let none = Type::None;

        assert_eq!(signal.get_signal_value(), Some(123));
        assert_eq!(wire.get_signal_value(), None);
        assert_eq!(none.get_signal_value(), None);
    }

    #[test]
    fn test_compute() {
        use super::Wires;

        let test_data = r#"
            123 -> x
            456 -> y
            x AND y -> d
            x OR y -> e
            x LSHIFT 2 -> f
            y RSHIFT 2 -> g
            NOT x -> h
            NOT y -> i
        "#;

        let mut wires = Wires::from_input_data(test_data);
        wires.compute();
        assert_eq!(wires.all_known(), true);
    }
}
