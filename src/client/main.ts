import {establishConnection, establishPayer, checkProgram, transfer, reportGreetings} from './transference';
  
async function main() {
  // Establish connection to the cluster
  await establishConnection();
  // Determine who pays for the fees
  await establishPayer();
  // Check if the program has been deployed
  await checkProgram();
  // Transfer to an account
  await transfer();
  // Find out how many times that account has been greeted
  await reportGreetings();
}
  
main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  }
);