const assert = require('assert');
const anchor = require('@project-serum/anchor');
const { describe, it } = require('mocha');

const { SystemProgram } = anchor.web3;

describe('mycalculaterdapp', () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const calculator = anchor.web3.Keypair.generate();
  const program = anchor.workspace.Mycalculatordapp;

  it('Create a calculator', async () => {
    const message = 'Hi Fakhri';

    await program.rpc.create(message, {
      accounts: {
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [calculator]
    })

    const account = await program.account.calculator.fetch(calculator.publicKey);

    assert.ok(account.greeting === message);
  });
})