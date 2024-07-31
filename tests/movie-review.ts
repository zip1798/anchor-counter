import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { AnchorCounter } from "../target/types/anchor_counter";
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token";

describe("movie-review", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorCounter as Program<AnchorCounter>;
  
  const movie = {
    title: "Just a test movie",
    description: "Wow what a good movie it was real ....",
    rating: 5,
  }

  const [moviePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(movie.title), provider.wallet.publicKey.toBuffer()],
      program.programId
  )

  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  )

  it("Initializes the reward token", async () => {
    const tx = await program.methods
      .initializeTokenMint().rpc()
  })


  it("Movie review is added", async () => {
    const tokenAccount = await getAssociatedTokenAddress(
      mint,
      provider.wallet.publicKey
    )

    const tx = await program.methods
      .addMovieReview(movie.title, movie.description, movie.rating)
      .accounts({
        tokenAccount: tokenAccount,
      })
      .rpc()
    
    const account = await program.account.movieAccountState.fetch(moviePda)
    expect(movie.title).to.equal(account.title)
    expect(movie.rating).to.equal(account.rating)
    expect(movie.description).to.equal(account.description)
    expect(account.reviewer.toBase58()).to.equal(provider.wallet.publicKey.toBase58())

    const userAta = await getAccount(provider.connection, tokenAccount)
    expect(Number(userAta.amount)).to.equal((10*10)^6)
      
  });


  it("Movie review is updated", async () => {
    const newDescription = "Wow this is new and better"
    const newRating = 4

    const tx = await program.methods
      .updateMovieReview(movie.title, newDescription, newRating)
      .rpc()
    
    const account = await program.account.movieAccountState.fetch(moviePda)
    expect(movie.title).to.equal(account.title)
    expect(newRating).to.equal(account.rating)
    expect(newDescription).to.equal(account.description)
    expect(account.reviewer.toBase58()).to.equal(provider.wallet.publicKey.toBase58())
  })

  it("Deletes a mouvie review", async () => {
    const tx = await program.methods
      .deleteMovieReview(movie.title)
      .rpc()
  })

});
