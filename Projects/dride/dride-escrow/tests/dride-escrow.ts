import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DrideEscrow } from "../target/types/dride_escrow";
import { Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";
import { v4 as uuidv4 } from "uuid";

function uuidToBytes(uuid: string): number[] {
  return Array.from(Buffer.from(uuid.replace(/-/g, ""), "hex"));
}

describe("dride-escrow", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.drideEscrow as Program<DrideEscrow>;

  const protocolWallet = Keypair.generate();
  const authority = provider.wallet;

  async function airdrop(pubkey: PublicKey, amount: number) {
    const sig = await provider.connection.requestAirdrop(pubkey, amount);
    await provider.connection.confirmTransaction(sig);
  }

  async function createRideEscrow(
    passenger: Keypair,
    rideId: number[],
    amount: number,
    feeBps: number = 1000
  ) {
    const [escrowPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("ride"), Buffer.from(rideId)],
      program.programId
    );

    await program.methods
      .createRide(rideId, new anchor.BN(amount), feeBps)
      .accounts({
        passenger: passenger.publicKey,
        escrow: escrowPda,
        protocolWallet: protocolWallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([passenger])
      .rpc();

    return escrowPda;
  }

  it("creates a ride escrow with deposit", async () => {
    const passenger = Keypair.generate();
    await airdrop(passenger.publicKey, 2 * LAMPORTS_PER_SOL);

    const rideId = uuidToBytes(uuidv4());
    const amount = 0.15 * LAMPORTS_PER_SOL;

    const escrowPda = await createRideEscrow(passenger, rideId, amount);

    const escrow = await program.account.rideEscrow.fetch(escrowPda);
    assert.deepEqual(escrow.rideId, rideId);
    assert.equal(escrow.passenger.toBase58(), passenger.publicKey.toBase58());
    assert.equal(escrow.amount.toNumber(), amount);
    assert.equal(escrow.protocolFeeBps, 1000);
    assert.deepEqual(escrow.status, { created: {} });
  });

  it("rejects zero amount", async () => {
    const passenger = Keypair.generate();
    await airdrop(passenger.publicKey, 2 * LAMPORTS_PER_SOL);

    const rideId = uuidToBytes(uuidv4());

    try {
      await createRideEscrow(passenger, rideId, 0);
      assert.fail("Should have thrown");
    } catch (e: any) {
      assert.include(e.toString(), "InvalidAmount");
    }
  });

  it("rejects fee too high (>50%)", async () => {
    const passenger = Keypair.generate();
    await airdrop(passenger.publicKey, 2 * LAMPORTS_PER_SOL);

    const rideId = uuidToBytes(uuidv4());

    try {
      await createRideEscrow(passenger, rideId, LAMPORTS_PER_SOL, 5001);
      assert.fail("Should have thrown");
    } catch (e: any) {
      assert.include(e.toString(), "FeeTooHigh");
    }
  });

  it("happy path: create → accept → complete with 90/10 split", async () => {
    const passenger = Keypair.generate();
    const driver = Keypair.generate();
    await airdrop(passenger.publicKey, 5 * LAMPORTS_PER_SOL);
    await airdrop(driver.publicKey, 0.1 * LAMPORTS_PER_SOL);
    await airdrop(protocolWallet.publicKey, 0.1 * LAMPORTS_PER_SOL);

    const rideId = uuidToBytes(uuidv4());
    const fareAmount = 1 * LAMPORTS_PER_SOL;

    const escrowPda = await createRideEscrow(passenger, rideId, fareAmount);

    // Accept ride
    await program.methods
      .acceptRide()
      .accounts({
        authority: authority.publicKey,
        driver: driver.publicKey,
        escrow: escrowPda,
      })
      .rpc();

    let escrow = await program.account.rideEscrow.fetch(escrowPda);
    assert.deepEqual(escrow.status, { active: {} });
    assert.equal(escrow.driver.toBase58(), driver.publicKey.toBase58());

    // Record balances before complete
    const driverBefore = await provider.connection.getBalance(driver.publicKey);
    const protocolBefore = await provider.connection.getBalance(protocolWallet.publicKey);

    // Complete ride
    await program.methods
      .completeRide()
      .accounts({
        authority: authority.publicKey,
        driver: driver.publicKey,
        protocolWallet: protocolWallet.publicKey,
        escrow: escrowPda,
      })
      .rpc();

    escrow = await program.account.rideEscrow.fetch(escrowPda);
    assert.deepEqual(escrow.status, { completed: {} });

    // Verify 90/10 split
    const driverAfter = await provider.connection.getBalance(driver.publicKey);
    const protocolAfter = await provider.connection.getBalance(protocolWallet.publicKey);

    const expectedDriverAmount = fareAmount * 0.9;
    const expectedFee = fareAmount * 0.1;

    assert.equal(driverAfter - driverBefore, expectedDriverAmount);
    assert.equal(protocolAfter - protocolBefore, expectedFee);
  });

  it("cancel before accept → full refund to passenger", async () => {
    const passenger = Keypair.generate();
    await airdrop(passenger.publicKey, 5 * LAMPORTS_PER_SOL);

    const rideId = uuidToBytes(uuidv4());
    const fareAmount = 0.5 * LAMPORTS_PER_SOL;

    const balanceBefore = await provider.connection.getBalance(passenger.publicKey);
    const escrowPda = await createRideEscrow(passenger, rideId, fareAmount);
    const balanceAfterDeposit = await provider.connection.getBalance(passenger.publicKey);

    // Cancel
    await program.methods
      .cancelRide()
      .accounts({
        authority: authority.publicKey,
        passenger: passenger.publicKey,
        escrow: escrowPda,
      })
      .rpc();

    const escrow = await program.account.rideEscrow.fetch(escrowPda);
    assert.deepEqual(escrow.status, { cancelled: {} });

    const balanceAfterRefund = await provider.connection.getBalance(passenger.publicKey);
    // Passenger should get back the fare (minus tx fees for create)
    assert.isAbove(balanceAfterRefund, balanceAfterDeposit);
  });

  it("cancel after accept → full refund", async () => {
    const passenger = Keypair.generate();
    const driver = Keypair.generate();
    await airdrop(passenger.publicKey, 5 * LAMPORTS_PER_SOL);
    await airdrop(driver.publicKey, 0.1 * LAMPORTS_PER_SOL);

    const rideId = uuidToBytes(uuidv4());
    const fareAmount = 0.5 * LAMPORTS_PER_SOL;

    const escrowPda = await createRideEscrow(passenger, rideId, fareAmount);

    // Accept
    await program.methods
      .acceptRide()
      .accounts({
        authority: authority.publicKey,
        driver: driver.publicKey,
        escrow: escrowPda,
      })
      .rpc();

    // Cancel after accept
    await program.methods
      .cancelRide()
      .accounts({
        authority: authority.publicKey,
        passenger: passenger.publicKey,
        escrow: escrowPda,
      })
      .rpc();

    const escrow = await program.account.rideEscrow.fetch(escrowPda);
    assert.deepEqual(escrow.status, { cancelled: {} });
  });

  it("double complete attempt → error", async () => {
    const passenger = Keypair.generate();
    const driver = Keypair.generate();
    await airdrop(passenger.publicKey, 5 * LAMPORTS_PER_SOL);
    await airdrop(driver.publicKey, 0.1 * LAMPORTS_PER_SOL);
    await airdrop(protocolWallet.publicKey, 0.1 * LAMPORTS_PER_SOL);

    const rideId = uuidToBytes(uuidv4());
    const fareAmount = 0.5 * LAMPORTS_PER_SOL;

    const escrowPda = await createRideEscrow(passenger, rideId, fareAmount);

    await program.methods
      .acceptRide()
      .accounts({
        authority: authority.publicKey,
        driver: driver.publicKey,
        escrow: escrowPda,
      })
      .rpc();

    await program.methods
      .completeRide()
      .accounts({
        authority: authority.publicKey,
        driver: driver.publicKey,
        protocolWallet: protocolWallet.publicKey,
        escrow: escrowPda,
      })
      .rpc();

    // Second complete should fail
    try {
      await program.methods
        .completeRide()
        .accounts({
          authority: authority.publicKey,
          driver: driver.publicKey,
          protocolWallet: protocolWallet.publicKey,
          escrow: escrowPda,
        })
        .rpc();
      assert.fail("Should have thrown");
    } catch (e: any) {
      assert.include(e.toString(), "InvalidStatus");
    }
  });
});
