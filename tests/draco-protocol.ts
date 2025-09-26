import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DracoProtocol } from "../target/types/draco_protocol";
import { Commitment, Keypair, PublicKey } from "@solana/web3.js";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import * as dotenv from "dotenv";
import * as sb from "@switchboard-xyz/on-demand";
import bs58 from "bs58";
// Load environment variables from .env file
dotenv.config();

const TOKEN_MINT = new PublicKey(process.env.TOKEN_MINT_ADDRESS || "");
if (!TOKEN_MINT) {
  throw new Error("TOKEN_MINT environment variable is required. Please run ./local_set_up.sh first.");
}

const FINISHED_LOTTERY_ID = new anchor.BN(1);
const PAY_LOTTERY_ID = new anchor.BN(2);
const LOCK_LOTTERY_ID = new anchor.BN(3);
const WRONG_COMBINATION = "W2H10C8CA";
const VALID_COMBINATION = "W2HTC8CA";
const WRONG_AMOUNT_PAY = new anchor.BN(40);
const VALID_AMOUNT_PAY = new anchor.BN(50);
const WRONG_AMOUNT_LOCK = new anchor.BN(400);
const VALID_AMOUNT_LOCK = new anchor.BN(501);

async function loadSbProgram(mainNetProvider: anchor.Provider, localProvider: anchor.Provider) {
  const sbProgramId = await sb.getProgramId(mainNetProvider.connection)
  const sbIdl = await anchor.Program.fetchIdl(sbProgramId, mainNetProvider)
  const sbProgram = new anchor.Program(sbIdl, localProvider)
  return sbProgram
}

async function setupQueue(program: anchor.Program): Promise<PublicKey> {
  const queueAccount = await sb.getDefaultQueue(
    "https://mainnet.helius-rpc.com/?api-key=2efc7d77-3ec0-491c-a0e3-1778a19bd4dd",
  )
  console.log("Queue account", queueAccount.pubkey.toString());
  try{
    await queueAccount.loadData()
  } catch (e) {
    console.error("Queue not found, ensure you are using devnet in your env");
    process.exit(1);
  }
  return queueAccount.pubkey
}

describe("- Draco Protocol Set Up", () => {
  let sbProgram: anchor.Program<anchor.Idl>
  // let queue: anchor.web3.PublicKey

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet
  const signer = wallet.payer
  const payer = wallet.publicKey
  anchor.setProvider(provider);

  const program = anchor.workspace.DracoProtocol as Program<DracoProtocol>;

  before(async () => {
    // Now wallet is available here
    const mainnetRpcUrl = "https://mainnet.helius-rpc.com/?api-key=2efc7d77-3ec0-491c-a0e3-1778a19bd4dd";
    const mainnetConnection = new anchor.web3.Connection(mainnetRpcUrl);
    
    const mainNetProvider = new anchor.AnchorProvider(
      mainnetConnection,
      wallet, // wallet is now defined above
      { commitment: "confirmed" }
    );
    const sbProgramResult = await loadSbProgram(mainNetProvider, provider)
    // const queueResult = await setupQueue(sbProgramResult)
    sbProgram = sbProgramResult;
    // queue = queueResult;
  })

  const [protocolAuthorityPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("protocol_authority")],
    program.programId
  )

  const rngKp = anchor.web3.Keypair.generate();


  it("Initializing authority", async () => {
    try {
      // Check if the lotteries authority account already exists
      await program.account.protocolAuthority.fetch(
        protocolAuthorityPDA,
      );
    } catch (e) {
      // Create the LotteriesAuthority account
      const txSig = await program.methods
        .initializeAuthority()
        .accounts({
          payer: payer,
        })
        .signers([signer])
        .rpc({ skipPreflight: true });

      // Fetch the LotteriesAuthority account that we just created
      const protocolAuthorityAccountData = await program.account.protocolAuthority.fetch(
        protocolAuthorityPDA,
      );
    }
  });

  it("Initialize Treasury", async () => {
    const txSig = await program.methods
      .initializeTreasury()
      .accounts({
        payer: payer,
        tokenMint: TOKEN_MINT,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([signer])
      .rpc({ skipPreflight: true });
  })

  it("Initialize Lottery Reward Factors", async () => {
    const txSig = await program.methods
      .initializeLotteryRewardFactors()
      .accounts({
        payer: payer,
      })
      .signers([signer])
      .rpc({ skipPreflight: true });
  })

  it("Update Lottery Reward Factors", async () => {
    const txSig = await program.methods.updateLotteryRewardFactors(
      1.0,
      0.3,
      0.5,
      [0.0, 0.0, 0.25, 0.60, 1.20],
      [0.0, 0.0, 0.50, 1.20, 2.20],
      0.2,
      0.55,
      0.9,
      50.0,
    )
    .accounts({
      payer: payer,
    })
    .signers([signer])
    .rpc({ skipPreflight: true });
  })

  it("Transfer out from six month cliff", async () => {
    try{
      const txSig = await program.methods
      .transferOutFromSixMonthCliff()
      .accounts({
        payer: payer,
        tokenMint: TOKEN_MINT,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([signer])
      .rpc({ skipPreflight: true });
    }
    catch (e) {
      console.log("Six month cliff:");
      console.log(e.msg);
    }
  })

  it("Start Lotteries", async () => {
    const finishedLotteryTxSig = await program.methods
    .startLottery(
      FINISHED_LOTTERY_ID,
      "Test Lottery PAY",
      "Test Lottery Description PAY",
      0,
      new anchor.BN(1654586517),
      new anchor.BN(1654686517),
      new anchor.BN(1000000),
      new anchor.BN(50)
    )
    .accounts({
      payer: payer,
      tokenMint: TOKEN_MINT,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .signers([signer])
    .rpc({ skipPreflight: true });

    const payLotteryTxSig = await program.methods
    .startLottery(
      PAY_LOTTERY_ID,
      "Test Lottery PAY",
      "Test Lottery Description PAY",
      0,
      new anchor.BN(Math.floor(Date.now() / 1000)),
      new anchor.BN(Math.floor(Date.now() / 1000) + 1*60),
      new anchor.BN(1000000),
      new anchor.BN(50)
    )
    .accounts({
      payer: payer,
      tokenMint: TOKEN_MINT,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .signers([signer])
    .rpc({ skipPreflight: true });

    const lockLotteryTxSig = await program.methods
    .startLottery(
      LOCK_LOTTERY_ID,
      "Test Lottery LOCK",
      "Test Lottery Description LOCK",
      1,
      new anchor.BN(1755792272),
      new anchor.BN(1756796272),
      new anchor.BN(10000),
      new anchor.BN(500)
    )
    .accounts({
      payer: payer,
      tokenMint: TOKEN_MINT,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .signers([signer])
    .rpc({ skipPreflight: true });
  })

  it("Buy Lottery Ticket Lottery for finished lottery", async () => {
    try {
    const _ = await program.methods
    .buyLotteryTicket(
      FINISHED_LOTTERY_ID,
      VALID_COMBINATION,
      VALID_AMOUNT_PAY,
    )
    .accounts({
      payer: payer,
      tokenMint: TOKEN_MINT,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .signers([signer])
    .rpc({ skipPreflight: true });
    }
    catch (e) {
      console.log(e.msg);
    }
  })

  it("Buy Lottery Ticket Invalid Combination", async () => {
    try {
    const _ = await program.methods
    .buyLotteryTicket(
      PAY_LOTTERY_ID,
      WRONG_COMBINATION,
      VALID_AMOUNT_PAY,
    )
    .accounts({
      payer: payer,
      tokenMint: TOKEN_MINT,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .signers([signer])
    .rpc({ skipPreflight: true });
    }
    catch (e) {
      console.log(e.msg);
    }
  })

  it("Buy Lottery Ticket PAY Invalid Amount", async () => {
    try {
    const _ = await program.methods
    .buyLotteryTicket(
      PAY_LOTTERY_ID,
      VALID_COMBINATION,
      WRONG_AMOUNT_PAY,
    )
    .accounts({
      payer: payer,
      tokenMint: TOKEN_MINT,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .signers([signer])
    .rpc({ skipPreflight: true });
    }
    catch (e) {
      console.log(e.msg);
    }
  })

  it("Buy Lottery Ticket PAY Valid Amount", async () => {
    const _ = await program.methods
    .buyLotteryTicket(
      PAY_LOTTERY_ID,
      VALID_COMBINATION,
      VALID_AMOUNT_PAY,
    )
    .accounts({
      payer: payer,
      tokenMint: TOKEN_MINT,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .signers([signer])
    .rpc({ skipPreflight: true });
  })

  it("Buy Lottery Ticket LOCK Invalid Amount", async () => {
    try {
    const _ = await program.methods
    .buyLotteryTicket(
      LOCK_LOTTERY_ID,
      VALID_COMBINATION,
      WRONG_AMOUNT_LOCK,
    )
    .accounts({
      payer: payer,
      tokenMint: TOKEN_MINT,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .signers([signer])
    .rpc({ skipPreflight: true });
    }
    catch (e) {
      console.log(e.msg);
    }
  })

  it("Buy Lottery Ticket LOCK Valid Amount", async () => {
    const _ = await program.methods
    .buyLotteryTicket(
      LOCK_LOTTERY_ID,
      VALID_COMBINATION,
      VALID_AMOUNT_LOCK,
    )
    .accounts({
      payer: payer,
      tokenMint: TOKEN_MINT,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .signers([signer])
    .rpc({ skipPreflight: true });
  })

  it("Commit and reveal a lottery combination", async () => {

    console.log("Waiting for PAY lottery to finish...");

    await new Promise(resolve => setTimeout(resolve, 1.2*60*1000));

    const queue = new anchor.web3.PublicKey("A43DyUGA7s8eXPxqEjJY6EBu1KKbNgfxF8h17VAHn13w");
    let queueAccount: sb.Queue;
    try {
      queueAccount = new sb.Queue(sbProgram, queue)
    } catch (err) {
      console.log("Queue account not found");
      return
    }

    try {
      await queueAccount.loadData();
    } catch (err) {
      console.log("Queue account not found");
      process.exit(1);
    }

    const [randomness, ix] = await sb.Randomness.create(sbProgram, rngKp, queue);
    console.log("Created randomness account..");
    console.log("Randomness account", randomness.pubkey.toBase58());
    console.log("rkp account", rngKp.publicKey.toBase58());

    const createRandomnessTx = await sb.asV0Tx({
      connection: connection,
      ixs: [ix],
      payer: wallet.publicKey,
      signers: [wallet.payer, rngKp],
      computeUnitPrice: 75_000,
      computeUnitLimitMultiple: 1.3,
    });

    const blockhashContext = await connection.getLatestBlockhashAndContext();
  
    const createRandomnessSignature = await connection.sendTransaction(createRandomnessTx);
    await connection.confirmTransaction({
      signature: createRandomnessSignature,
      blockhash: blockhashContext.value.blockhash,
      lastValidBlockHeight: blockhashContext.value.lastValidBlockHeight
    });
    console.log(
      "Transaction Signature for randomness account creation: ",
      createRandomnessSignature
    );
    const sbCommitIx = await randomness.commitIx(queue);

    console.log("sbCommitIx:", sbCommitIx);

    const lotteryCommitIx = await program.methods.commitLotteryRandomness(PAY_LOTTERY_ID).accounts({
      payer: payer,
      randomnessAccount: randomness.pubkey,
    })
    .instruction();

    console.log("lotteryCommitIx:", lotteryCommitIx);

    const commitTx = await sb.asV0Tx({
      connection: sbProgram.provider.connection,
      ixs: [sbCommitIx, lotteryCommitIx],
      payer: wallet.publicKey,
      signers: [wallet.payer],
      computeUnitPrice: 75_000,
      computeUnitLimitMultiple: 1.3,
    });

    console.log("commitTx:", commitTx);

    const commitSignature = await connection.sendTransaction(commitTx);

    console.log("commitSignature:", commitSignature);

    await connection.confirmTransaction({
      signature: commitSignature,
      blockhash: blockhashContext.value.blockhash,
      lastValidBlockHeight: blockhashContext.value.lastValidBlockHeight
    });

    console.log(
      "Transaction Signature for commit: ",
      commitSignature
    );

    const sbRevealIx = await randomness.revealIx();
    const lotteryRevealIx = await program.methods.revealLotteryRandomness(PAY_LOTTERY_ID).accounts({
      payer: payer,
      randomnessAccount: randomness.pubkey,
    })
    .instruction();

    console.log("lotteryRevealIx:", lotteryRevealIx);

    const revealTx = await sb.asV0Tx({
      connection: sbProgram.provider.connection,
      ixs: [sbRevealIx, lotteryRevealIx],
      payer: wallet.publicKey,
      signers: [wallet.payer],
      computeUnitPrice: 75_000,
      computeUnitLimitMultiple: 1.3,
    });

    console.log("revealTx:", revealTx);

    const revealSignature = await connection.sendTransaction(revealTx);
    await connection.confirmTransaction({
      signature: revealSignature,
      blockhash: blockhashContext.value.blockhash,
      lastValidBlockHeight: blockhashContext.value.lastValidBlockHeight
    });

    console.log(
      "Transaction Signature for reveal: ",
      revealSignature
    );
  })

  it("Claim Lottery Prize for finished lottery", async () => {
    const txSig = await program.methods
      .claimLotteryPrizeForCombination(
        PAY_LOTTERY_ID,
        VALID_COMBINATION,
      )
      .accounts({
        payer: payer,
        tokenMint: TOKEN_MINT,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([signer])
      .rpc({ skipPreflight: true });
  })

  it("Close Lottery not old enough", async () => {
    try{
      const txSig = await program.methods
        .closeLottery(
          PAY_LOTTERY_ID
        )
        .accounts({
          payer: payer,
          tokenMint: TOKEN_MINT,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .signers([signer])
        .rpc({ skipPreflight: true });
    } catch (e) {
      console.log(e.msg);
    }
  })

  it("Close Lottery", async () => {
    const txSig = await program.methods
      .closeLottery(
        FINISHED_LOTTERY_ID
      )
      .accounts({
        payer: payer,
        tokenMint: TOKEN_MINT,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([signer])
      .rpc({ skipPreflight: true });
  })
});