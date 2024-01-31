import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hahatoco } from "../target/types/hahatoco";
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token"

describe("hahatoco", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Hahatoco as Program<Hahatoco>;

  const movie = {
    title: "Just a test movie",
    description: "Wow what a good movie it was real great",
    rating: 5,
  }

  const state = {
    x: 8,
  }

  const [movie_pda] = anchor.web3.PublicKey.findProgramAddressSync(

    [Buffer.from(movie.title), provider.wallet.publicKey.toBuffer()],

    program.programId

  )


  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(

    [Buffer.from("mint")],

    program.programId

  )

  it("Is initialized!", async () => {
    // Add your test here.
    const tokenAccount = await getAssociatedTokenAddress(
      mint,
      provider.wallet.publicKey
    )

    const tx = await program.methods.addMovieReview(
      movie.title, movie.description, movie.rating, state.x,
    )
      .accounts({
        tokenAccount: tokenAccount,
      }).rpc();
    console.log("Your transaction signature", tx);
  });
});
