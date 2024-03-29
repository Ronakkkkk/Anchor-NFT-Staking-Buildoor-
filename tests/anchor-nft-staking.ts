 import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorNftStaking } from "../target/types/anchor_nft_staking";
import { setupNft } from "./utils/setupNft";
import { expect } from "chai";

describe("anchor-nft-staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const wallet = anchor.workspace.AnchorNftStaking.provider.wallet

  const program = anchor.workspace.AnchorNftStaking as Program<AnchorNftStaking>;

  let delegatedAuthPda: anchor.web3.PublicKey
  let stakeStatePda: anchor.web3.PublicKey
  let nft: any
  let mintAuth: anchor.web3.PublicKey
  let mint: anchor.web3.PublicKey
  let tokenAddress: anchor.web3.PublicKey

  before(async() => {
    ;({nft, delegatedAuthPda, stakeStatePda, mint, mintAuth, tokenAddress} = await setupNft(program, wallet.payer))
  })


  it("Stakes", async () => {
    // Add your test here.
    await program.methods.stake().accounts({
      nftTokenAccount: nft.tokenAddress, 
      nftMint: nft.mintAddress, 
      nftEdition: nft.masterEditionAddress,
       metadataProgram: METADATA_PROGRAM_ID,
      }).rpc()

      const account = await program.account.userStakeInfo.fetch(stakeStatePda)
        expect(account.stakeState === "Staked")
    // const tx = await program.methods.initialize().rpc();
    // console.log("Your transaction signature", tx);
  });

  it("Redeems",async () => {
    await program.methods.redeem()
    .accounts({
      nftTokenAccount: nft.tokenAddress,
      stakeMint: mint,
      userStakeAta: tokenAddress
    })
    .rpc()

    const account = await program.account.userStakeInfo.fetch(stakeStatePda)
    expect(account.stakeState === "Unstaked")
    const nftTokenAccount = await getAccount(provider.connection, tokenAddress)
  });

  it("Unstakes", async () => {
    await program.methods
      .unstake()
      .accounts({
        nftTokenAccount: nft.tokenAddress,
        nftMint: nft.mintAddress,
        nftEdition: nft.masterEditionAddress,
        metadataProgram: METADATA_PROGRAM_ID,
        stakeMint: mint,
        userStakeAta: tokenAddress,
      })
      .rpc()

      const account = await program.account.userStakeInfo.fetch(stakeStatePda)
      expect(account.stakeState === "Unstaked")
  })

  

});
