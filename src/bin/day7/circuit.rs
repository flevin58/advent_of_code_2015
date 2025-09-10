use anyhow::{Result, bail};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

macro_rules! signal_or_wire {
    ($x:expr) => {
        if let Ok(signal) = $x.parse::<u16>() {
            Type::Signal(signal)
        } else {
            Type::Wire($x.to_string())
        }
    };
}

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
            Type::None => {
                write!(f, "None")?;
            }
        }
        Ok(())
    }
}

impl Component {
    pub fn from_str(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts.len() {
            1 => {
                // When length is 1, it's a direct signal or wire assignment
                Ok(Self {
                    ctype: signal_or_wire!(parts[0]),
                    left: Type::None,
                    right: Type::None,
                })
            }

            2 => {
                // When length is 2, it must be a NOT gate
                if parts[0] == "NOT" {
                    Ok(Self {
                        ctype: Type::Gate("NOT".to_string()),
                        left: Type::None,
                        right: signal_or_wire!(parts[1]),
                    })
                } else {
                    bail!("Expected a NOT gate in '{line}'");
                }
            }

            3 => {
                // When length is 3, it must be a binary gate
                Ok(Self {
                    ctype: Type::Gate(parts[1].to_string()),
                    left: signal_or_wire!(parts[0]),
                    right: signal_or_wire!(parts[2]),
                })
            }

            _ => {
                // Any other length is invalid
                bail!("Invalid component line: {line}");
            }
        }
    }
}

#[derive(Clone)]
pub struct Wires(HashMap<String, Component>);

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

    pub fn get_wire_signal(&self, name: &str) -> Option<u16> {
        self.0
            .get(name)
            .and_then(|comp| comp.ctype.get_signal_value())
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

    pub fn from_input_data(input_data: &str) -> Result<Self> {
        let mut wires = Self::new();
        // Parse each line into a Component and add it to the Wires hashmap
        for line in input_data.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split(" -> ").map(|item| item.trim()).collect();
            let component = Component::from_str(parts[0])?;
            wires.0.insert(parts[1].to_string(), component);
        }
        Ok(wires)
    }

    pub fn all_known(&self) -> bool {
        self.0.values().all(|c| matches!(c.ctype, Type::Signal(_)))
    }

    pub fn compute(&mut self) -> Result<()> {
        while !self.all_known() {
            self.0.clone().iter().try_for_each(|(k, v)| {
                match v.ctype {
                    Type::Signal(_) => Ok(()),
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
                        Ok(())
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
                                let gate_val = match gate_name.as_str() {
                                    "AND" => left_val & right_val,
                                    "OR" => left_val | right_val,
                                    "LSHIFT" => left_val << right_val,
                                    "RSHIFT" => left_val >> right_val,
                                    _ => unreachable!(),
                                };
                                self.0.insert(
                                    k.clone(),
                                    Component {
                                        ctype: Type::Signal(gate_val),
                                        left: Type::None,
                                        right: Type::None,
                                    },
                                );
                            }
                            Ok(())
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
                                let gate_val = !right_val;
                                // Update the component to a signal
                                self.0.insert(
                                    k.clone(),
                                    Component {
                                        ctype: Type::Signal(gate_val),
                                        left: Type::None,
                                        right: Type::None,
                                    },
                                );
                            }
                            Ok(())
                        }
                        wtf => {
                            bail!("Unknown gate type: '{wtf}'");
                        }
                    },
                    Type::None => Ok(()),
                }
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Type;
    use test_case::test_case;

    #[test_case(Type::Signal(123_u16), Some(123_u16); "Signal")]
    #[test_case(Type::Wire("x".to_string()), None; "Wire")]
    #[test_case(Type::None, None; "None")]
    fn get_signal_value(ty: Type, expected: Option<u16>) {
        assert_eq!(ty.get_signal_value(), expected);
    }

    #[test]
    fn compute() {
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

        let mut wires = Wires::from_input_data(test_data).unwrap();
        wires.compute().unwrap();
        assert_eq!(wires.all_known(), true);
    }
}
