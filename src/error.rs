use thiserror::Error;

#[#[derive(Error, Debug, Copy, Clone)]]
pub enum EscrowError {

    /// Invalid Instruction Error
    #[error("Invalid instruction")]
    InvalidInstruction,
}

impl From<EscrowError> for ProgramError {
    fn from(err: EscrowError) -> Self {
        ProgramError::Custom(err as u32)
    }
}
