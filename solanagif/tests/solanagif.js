// import anchor
const anchor = require('@project-serum/anchor');

// run main asynchronously
const main = async () => {
  console.log('🚀 Starting test...');
  // letting Anchor get local environment to run code locally
  anchor.setProvider(anchor.Provider.env());
  // run rust program
  const program = anchor.workspace.Solanagif;
  // local validator will mine the instructions in
  // startStuffOff() so we await it
  const tx = await program.rpc.startStuffOff();

  console.log('📝 Your transaction signature', tx);
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
