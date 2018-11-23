use std::collections::HashMap;
use std::iter::{empty, once};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Value {
    False,
    Implies,
    Bound(String),
    Unbound(String),
    Stmt(Vec<Value>),
}
impl Value {
    pub fn unbounds(&self) -> Box<dyn Iterator<Item = String>> {
        match self {
            Value::Unbound(uuid) => Box::new(once(uuid.clone())) as Box<dyn Iterator<Item = String>>,
            Value::Stmt(stmt) => Box::new(stmt.clone().into_iter().flat_map(|val| val.unbounds()))
                as Box<dyn Iterator<Item = String>>,
            _ => Box::new(empty()) as Box<dyn Iterator<Item = String>>,
        }
    }
    pub fn r#match(&self, other: &Value, binds: &mut HashMap<String, Value>) -> bool {
        match self {
            Value::Unbound(u1) => {
                if let Some(v1) = binds.get(u1) {
                    if v1 != other {
                        return false;
                    }
                } else {
                    binds.insert(u1.clone(), other.clone());
                }
            }
            Value::Stmt(s1) => match other {
                Value::Stmt(s2) => {
                    if s1.len() != s2.len() {
                        return false;
                    }
                    for pair in s1.iter().cloned().zip(s2.iter().cloned()) {
                        if !pair.0.r#match(&pair.1, binds) {
                            return false;
                        }
                    }
                }
                _ => {
                    return false;
                }
            },
            v if v == other => {}
            _ => {
                return false;
            }
        }
        true
    }
}
