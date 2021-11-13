
/*
* * Solana programs are stateless by design. They don't hold data permanently like Ethereum smart contracts do.
* * Solana programs interact w/ accounts. They can read and write data to and from accounts.
* * Solana programs can also make an "account" that it can interact with.
* * In Solana, users need to pay "rent" to store data in an account, equivalent to storing it in a database. 
* * The SystemProgram is a program the creators of Solana deployed that other programs can talk to — it has an id of 11111111111111111111111111111111
*/


// import Anchor tools
use anchor_lang::prelude::*;

// information for Solana on how to run the program (gen by Anchor)
declare_id!("Bg6ant5paDG4quk5kte5ZpNiYc7s8J3F2AJcJrGnjVB9");

#[program] //let's frontend client call Solana program via fetch req
pub mod solanagif { // rust module
	use super::*;
	pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
		// Get mutable reference to the base account (&mut)
		// this enables us to make changes to the base_account
		let base_account = &mut ctx.accounts.base_account;
		base_account.total_gifs = 0;
		Ok(())
	}

	// add gif to base account
	pub fn add_gif(ctx: Context<AddGif>) -> ProgramResult {
		// Get mutable reference to the base account and increment total_gifs
		let base_account = &mut ctx.accounts.base_account;
		base_account.total_gifs += 1;
		Ok(())
	}

}

/*
 * * #[account(init, payer = user, space = 9000)] = instruct Solana how to initialize the base account
 * * init = tell Solana to create a new account owned by our current program.
 * * payer = user tells our program who is paying for this account to be created. (in this case, the user calling the program)
 * * space == 9000 tells Solana how much space to allocate for this account.
 * 
 * * pub user: Signer<'info> = data passed into the program that proves to the program the user calling this program actually owns their wallet account.
 * * pub system_program: a reference to the SystemProgram (a program that runs Solana)
 * 	 										 it's main task is to create accounts on Solana.
 */

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddGif context
// Context named AddGif has a mutable reference to the base account
#[derive(Accounts)]
pub struct AddGif<'info> {
	#[account(mut)]
	pub base_account: Account<'info, BaseAccount>,
}

// Initializing an account
// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}