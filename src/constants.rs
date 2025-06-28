use std::collections::HashMap;
use std::f64::consts;

/// Mathematical constants and their values
pub struct Constants {
    values: HashMap<String, f64>,
}

impl Constants {
    pub fn new() -> Self {
        let mut values = HashMap::new();
        
        // Mathematical constants
        values.insert("pi".to_string(), consts::PI);
        values.insert("π".to_string(), consts::PI);
        values.insert("e".to_string(), consts::E);
        values.insert("euler".to_string(), consts::E);
        
        // Degree conversion constant (π/180)
        values.insert("deg".to_string(), consts::PI / 180.0);
        values.insert("degree".to_string(), consts::PI / 180.0);
        
        // Additional useful constants
        values.insert("inf".to_string(), f64::INFINITY);
        values.insert("infinity".to_string(), f64::INFINITY);
        values.insert("nan".to_string(), f64::NAN);
        
        Self { values }
    }
    
    /// Get the value of a constant by name
    pub fn get(&self, name: &str) -> Option<f64> {
        self.values.get(name).copied()
    }
    
    /// Check if a name is a recognized constant
    pub fn is_constant(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
}

lazy_static::lazy_static! {
    pub static ref CONSTANTS: Constants = Constants::new();
}

/// Get the value of a constant by name
pub fn get_constant(name: &str) -> Option<f64> {
    CONSTANTS.get(name)
}

/// Check if a name is a recognized constant
pub fn is_constant(name: &str) -> bool {
    CONSTANTS.is_constant(name)
}

/// Convert a decimal value back to its constant name if it matches a known constant
pub fn value_to_constant_name(value: f64) -> Option<String> {
    for (name, const_value) in &CONSTANTS.values {
        if (value - const_value).abs() < f64::EPSILON {
            return Some(name.clone());
        }
    }
    None
} 