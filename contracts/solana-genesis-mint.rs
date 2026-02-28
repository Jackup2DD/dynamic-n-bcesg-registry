// Solana NFT Minting Contract for Dynamic N Bc.ESGs Genesis Node
use anchor_lang::prelude::*;

declare_id!("BesgDynamicMint00000000000000000000000000000001");

#[program]
pub mod dynamic_n_nft_mint {
    use super::*;

    pub fn mint_genesis_node(
        ctx: Context<MintGenesisNode>,
        serial_number: String,
        valuation_usd: u64,
    ) -> Result<()> {
        require!(serial_number == "BC-ESGs-00000001", CustomError::InvalidSerialNumber);
        require!(valuation_usd == 5_250_000_000_000, CustomError::InvalidValuation);

        let genesis_node = &mut ctx.accounts.genesis_node;
        genesis_node.serial_number = serial_number;
        genesis_node.minted_at = Clock::get()?.unix_timestamp;
        genesis_node.authority = ctx.accounts.authority.key();
        genesis_node.status = "MINTED".to_string();

        emit!(MintEvent {
            serial_number: genesis_node.serial_number.clone(),
            authority: ctx.accounts.authority.key(),
            timestamp: genesis_node.minted_at,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintGenesisNode<'info> {
    #[account(init, payer = authority, space = 256)]
    pub genesis_node: Account<'info, GenesisNode>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GenesisNode {
    pub serial_number: String,
    pub minted_at: i64,
    pub authority: Pubkey,
    pub status: String,
}

#[event]
pub struct MintEvent {
    pub serial_number: String,
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[error_code]
pub enum CustomError {
    #[msg("Invalid serial number")]
    InvalidSerialNumber,
    #[msg("Invalid valuation")]
    InvalidValuation,
}
