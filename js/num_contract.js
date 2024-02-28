import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const contract_wasm = fs.readFileSync("../contract.wasm");

const secretjs = new SecretNetworkClient({
    chainId: "pulsar-2",
    url: "https://api.pulsar.scrttestnet.com",
    wallet: wallet,
    walletAddress: wallet.address,
});

let contractCodeHash = process.env.CONTRACTHASH;
let contract_address = process.env.CONTRACTADDRESS;

// for query number of contract
let try_query_all_contract = async () => {
    const my_query = await secretjs.query.compute.queryContract({
        contract_address: contract_address,
        code_hash: contractCodeHash,
        query: { number_of_contract: {
          payer: wallet.address
        } },
    });
  
    console.log(my_query);
  };


  try_query_all_contract();