import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { parseAndPrintLogs } from "sol_log_bench";

import { ComputeUnitBenchmark } from "../target/types/compute_unit_benchmark";


const printBenchmark = async (txId: string, connection: anchor.web3.Connection, verbosity?: number): Promise<void> => {
  let tx = await connection.getTransaction(txId, { commitment: 'confirmed' });
  parseAndPrintLogs(tx.meta.logMessages, {verbosity: verbosity || 1});
}

describe("compute_unit_benchmark", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ComputeUnitBenchmark as Program<ComputeUnitBenchmark>;

  it("It should output an emtpy benchmark for the transaction", async () => {

    const benchTx = await program.rpc.emptyBenchmark({
      accounts: {
        signer: provider.wallet.publicKey
      },
      signers: []
    });
    await provider.connection.confirmTransaction(benchTx);
    await printBenchmark(benchTx, provider.connection)
    await printBenchmark(benchTx, provider.connection, 2)
  });


  it("It should output a nested benchmark for the transaction", async () => {

    const benchTx = await program.rpc.nestedBenchmarks({
      accounts: {
        signer: provider.wallet.publicKey
      },
      signers: []
    });
    await provider.connection.confirmTransaction(benchTx);
    await printBenchmark(benchTx, provider.connection)
    await printBenchmark(benchTx, provider.connection, 2)
  });

  it("It should output a simple benchmark for the transaction", async () => {

    const benchTx = await program.rpc.simpleBenchmark({
      accounts: {
        signer: provider.wallet.publicKey
      },
      signers: []
    });
    await provider.connection.confirmTransaction(benchTx);
    await printBenchmark(benchTx, provider.connection)
    await printBenchmark(benchTx, provider.connection, 2)
  });


});
