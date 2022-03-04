import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ComputeUnitBenchmark } from "../target/types/compute_unit_benchmark";

const parseCompute = (logs: string[]): number => {
    if (logs.length >= 2) {
      let match = logs.at(-2).match(/consumed ([0-9]+)/)
      if (match.length > 1) {
        return parseInt(match[1], 10);
      }
    }
    throw Error("Unable to parse log message for compute")
}

const benchmark = async (txId: string, provider: anchor.Provider): Promise<number> => {
  await provider.connection.confirmTransaction(txId);
  let tx = await provider.connection.getTransaction(txId, { commitment: 'confirmed' });
  return parseCompute(tx.meta.logMessages);
}

const printBenchmark = (baseline: number, computes: [string, number][]): void => {
  computes.sort((a, b) => a[1] - b[1]).forEach(([name, amount]) => {
    console.log(`${name} used ${amount} of compute units (+${amount-baseline})`)
  })
}

describe("compute_unit_benchmark", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ComputeUnitBenchmark as Program<ComputeUnitBenchmark>;

  let baseline = 0;

  before(async () => {
    // Get a baseline benchmark
    const txId = await program.rpc.baseline({});
    baseline = await benchmark(txId, provider)
    console.log("Baseline compute units: ", baseline)
  })

  it("It should measure a benchmark for pubkey parsing", async () => {
    const pubkeyFromString = await benchmark(await program.rpc.pubkeyFromString({}), provider);
    const pubkeyFromMacro = await benchmark(await program.rpc.pubkeyFromMacro({}), provider);

    printBenchmark(baseline, [["Pubkey from string", pubkeyFromString], ["Pubkey from macro", pubkeyFromMacro]])
  });

});
