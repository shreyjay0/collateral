import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Collateral } from '../target/types/collateral';

describe('collateral', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Collateral as Program<Collateral>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
