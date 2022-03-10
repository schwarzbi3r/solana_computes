use anchor_lang::{prelude::*};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const PUBKEY_STR: &str = "C8mgrCncLMtpfh4QzkJsPfrWd384yQeSYpdujw4LF53T";

#[program]
pub mod compute_unit_benchmark {

    use std::str::FromStr;
    use anchor_lang::solana_program::{pubkey, pubkey::Pubkey};

    use super::*;

    pub fn empty_benchmark(_ctx: Context<EmptyBenchmark>) -> Result<()> {
        msg!("Bench: Empty");
        solana_program::log::sol_log_compute_units();
        solana_program::log::sol_log_compute_units();
        Ok(())
    }

    pub fn simple_benchmark(_ctx: Context<EmptyBenchmark>) -> Result<()> {
        msg!("Bench: Pubkey macro");
        solana_program::log::sol_log_compute_units();
        let pubkey: Pubkey = pubkey!("C8mgrCncLMtpfh4QzkJsPfrWd384yQeSYpdujw4LF53T");
        solana_program::log::sol_log_compute_units();

        msg!("Bench: Pubkey to String");
        solana_program::log::sol_log_compute_units();
        msg!("Pubkey: {}", pubkey.to_string());
        solana_program::log::sol_log_compute_units();

        let pda_program_id: Pubkey = pubkey!("FFmsBmAiug1hFoxgQcGXufajAv6hcv1dpJ2U5r2x8dUK");
        msg!("Bench: FindProgramAddress for close bump");
        solana_program::log::sol_log_compute_units();
        solana_program::pubkey::Pubkey::find_program_address(&[b"qgjnnkdpbjr2o2e7"],&pda_program_id);
        solana_program::log::sol_log_compute_units();

        msg!("Bench: FindProgramAddress for distant bump");
        solana_program::log::sol_log_compute_units();
        solana_program::pubkey::Pubkey::find_program_address(&[b"GPPLNPqp17dRyAVZAtUebk98opJXc8ss"],&pda_program_id);
        solana_program::log::sol_log_compute_units();

        Ok(())
    }

    pub fn nested_benchmarks(ctx: Context<SingleAccountBenchmark>) -> Result<()> {
        msg!("Bench: EntireProgram");
        solana_program::log::sol_log_compute_units();

        msg!("Bench: FindProgramAddress");
        solana_program::log::sol_log_compute_units(); // Begin findProgramAddressBench
        solana_program::pubkey::Pubkey::find_program_address(&[b"foo"],&ctx.accounts.signer.key());
        solana_program::log::sol_log_compute_units(); // End findProgramAddressBench

        msg!("Bench: PubkeyFromString");
        solana_program::log::sol_log_compute_units(); // Begin pubkeyFromString
        let pubkey = Pubkey::from_str(PUBKEY_STR).unwrap();
        solana_program::log::sol_log_compute_units(); // End pubkeyFromString
        msg!("Pubkey: {}", pubkey.to_string());

        msg!("Log something else");

        solana_program::log::sol_log_compute_units();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EmptyBenchmark {}

#[derive(Accounts)]
pub struct SingleAccountBenchmark<'info> {
    pub signer: Signer<'info>,
}
