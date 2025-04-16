use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Reset,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorInstructions {
    operation: Operation,
    operating_value: f32,
}
impl CalculatorInstructions {
    #[allow(clippy::erasing_op)]
    pub fn evaluate(&self, value: f32) -> f32 {
        match &self.operation {
            Operation::Add => value + self.operating_value,
            Operation::Subtract => value - self.operating_value,
            Operation::Multiply => value * self.operating_value,
            Operation::Divide => value / self.operating_value,
            _ => value * 0.0_f32,
        }
    }
}
