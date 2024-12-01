use serde::{Deserialize, Serialize};

/// Structure representing the input for calculator operations
/// Expects two floating-point numbers as input parameters
#[derive(Deserialize)]
pub struct CalculatorInput {
    pub a: f32,  // First operand
    pub b: f32,  // Second operand
}

/// Structure representing the output of calculator operations
/// Contains the result and the name of the operation performed
#[derive(Serialize)]
pub struct CalculatorOutput {
    pub result: f32,     // Operation result
    pub operation: String, // Name of the operation performed
}

/// Calculator implementation providing basic arithmetic operations
pub struct Calculator;

impl Calculator {
    /// Adds two numbers and returns the result
    /// 
    /// # Arguments
    /// * `input` - CalculatorInput containing two numbers to add
    pub fn add(input: CalculatorInput) -> CalculatorOutput {
        CalculatorOutput {
            result: input.a + input.b,
            operation: "add".to_string(),
        }
    }
    
    /// Subtracts the second number from the first
    /// 
    /// # Arguments
    /// * `input` - CalculatorInput containing two numbers to subtract
    pub fn sub(input: CalculatorInput) -> CalculatorOutput {
        CalculatorOutput {
            result: input.a - input.b,
            operation: "sub".to_string(),
        }
    }
    
    /// Multiplies two numbers
    /// 
    /// # Arguments
    /// * `input` - CalculatorInput containing two numbers to multiply
    pub fn mul(input: CalculatorInput) -> CalculatorOutput {
        CalculatorOutput {
            result: input.a * input.b,
            operation: "mul".to_string(),
        }
    }
    
    /// Divides the first number by the second
    /// Note: Does not handle division by zero
    /// 
    /// # Arguments
    /// * `input` - CalculatorInput containing two numbers to divide
    pub fn div(input: CalculatorInput) -> CalculatorOutput {
        CalculatorOutput {
            result: input.a / input.b,
            operation: "div".to_string(),
        }
    }

    /// Raises the first number to the power of the second number
    /// Note: Converts second number to integer, might lose precision
    /// 
    /// # Arguments
    /// * `input` - CalculatorInput where a is the base and b is the exponent
    pub fn pow(input: CalculatorInput) -> CalculatorOutput {
        let mut result = 1.0;
        for _ in 0..input.b as i32 {
            result *= input.a;
        }
        CalculatorOutput {
            result,
            operation: "pow".to_string(),
        }
    }
}
