import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { HelloAnchor } from "../target/types/hello_anchor";
describe("prediction_market", () => {  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HelloAnchor as anchor.Program<HelloAnchor>;
  

  // Initialize a prediction market
  it("Initializes prediction market", async () => {
    const PREDICTION_MARKET_SEED = "predictionmarket";
    const [predictionMarket] = await web3.PublicKey.findProgramAddress(
      [Buffer.from(PREDICTION_MARKET_SEED)],
      program.programId
    );

  const context = {
        stateData: predictionMarket,
        rent: web3.SYSVAR_RENT_PUBKEY,
        systemProgram: web3.SystemProgram.programId,
        payer: program.provider.publicKey,
  }
    const tx = await program.methods.initializePredictionMarket().accounts(context).rpc()

    console.log("Prediction market initialized. Transaction:", tx);
  });

  // Place a bet on the prediction market
//   it("Places a bet", async () => {
//     const PREDICTION_MARKET_SEED = "predictionmarket";
//     const [predictionMarket] = await web3.PublicKey.findProgramAddress(
//       [Buffer.from(PREDICTION_MARKET_SEED)],
//       program.programId
//     );

//     const TOKENACCOUNT_SEED = "predictionmarket";
//     const [tokenaccount] = await web3.PublicKey.findProgramAddress(
//       [Buffer.from(TOKENACCOUNT_SEED)],
//       program.programId
//     );

//     const VAULT_SEED = "predictionmarket";
//     const [vault] = await web3.PublicKey.findProgramAddress(
//       [Buffer.from(VAULT_SEED)],
//       program.programId
//     );

//     enum PredictionType {
//   Higher,
//   Lower,
//   // Add more enum values as needed
// }

// const prediction_name = "Sample Prediction";
// const prediction = PredictionType.Higher; 
// const amount = new BN(100); 
// const start_time = new BN(1635648000); 
// const expiration_time = new BN(1635734400);

//       const context = {
//         stateData: predictionMarket,
//         payer: program.provider.publicKey,
//         rent: web3.SYSVAR_RENT_PUBKEY,
//         vault: vault,
//         toTokenAccount: tokenaccount,
//         systemProgram: web3.SystemProgram.programId,
//         tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
//         associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
//   }

//     const tx = await program.methods.placeBet(prediction_name, prediction, amount, start_time, expiration_time).accounts(context).rpc()

//     console.log("Bet placed. Transaction:", tx);
//   });

});
