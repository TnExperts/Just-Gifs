// import Anchor tools
use anchor_lang::prelude::*;

// information for Solana on how to run the program (gen by Anchor)
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program] //let's frontend client call Solana program via fetch req
pub mod solanagif { // rust module
	use super::*;
	pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
		Ok(())
	}
}

#[derive(Accounts)]
pub struct StartStuffOff {}
