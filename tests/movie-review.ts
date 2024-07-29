import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { AnchorCounter } from "../target/types/anchor_counter";

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



  it("Movie review is added", async () => {
    const tx = await program.methods
      .addMovieReview(movie.title, movie.description, movie.rating)
      .rpc()
    
    const account = await program.account.movieAccountState.fetch(moviePda)
    expect(movie.title).to.equal(account.title)
    expect(movie.rating).to.equal(account.rating)
    expect(movie.description).to.equal(account.description)
    expect(account.reviewer.toBase58()).to.equal(provider.wallet.publicKey.toBase58())
      
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
