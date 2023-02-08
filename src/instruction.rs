use solana_program::program_error::ProgramError;

use crate::{error::TokenError};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenInstruction {
  InitializeMint.
  InitializeAccount,
  Transfer,
}

impl TokenInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
      Ok(match input[0] {
          0 => TokenInstruction::InitializeMint,
          1 => TokenInstruction::InitializeAccount,
          2 => TokenInstruction::Transfer,
          _ => return Err(TokenError::InvalidInstruction.into()),
      })
  }

  pub fn pack(&self) -> Vec<u8> {
      let mut buf = Vec::new();
      match self {
          TokenInstruction::InitializeMint => buf.push(0),
          TokenInstruction::InitializeAccount => buf.push(1),
          TokenInstruction::Transfer => buf.push(2),
      }
      buf
  }
}