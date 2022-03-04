use anchor_lang::{prelude::*};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const PUBKEY_STR: &str = "C8mgrCncLMtpfh4QzkJsPfrWd384yQeSYpdujw4LF53T";

#[program]
pub mod compute_unit_benchmark {

    use std::str::FromStr;
    use anchor_lang::solana_program::{pubkey, pubkey::Pubkey};

    use super::*;

    pub fn baseline(_ctx: Context<EmptyBenchmark>) -> Result<()> {
        msg!("Baseline");
        Ok(())
    }

    pub fn pubkey_from_string(_ctx: Context<EmptyBenchmark>) -> Result<()> {
        let pubkey = Pubkey::from_str(PUBKEY_STR).unwrap();
        msg!("Pubkey: {}", pubkey.to_string());
        Ok(())
    }

    pub fn pubkey_from_macro(_ctx: Context<EmptyBenchmark>) -> Result<()> {
        let pubkey: Pubkey = pubkey!("C8mgrCncLMtpfh4QzkJsPfrWd384yQeSYpdujw4LF53T");
        msg!("Pubkey: {}", pubkey.to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EmptyBenchmark {}
