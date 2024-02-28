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

//for submiting invoice
let submit_invoice = async () => {
    
    try {
      let tx = await secretjs.tx.compute.executeContract(
        {
          sender: wallet.address,
          contract_address: contract_address,
          code_hash: contractCodeHash, // optional but way faster
          msg: {
            submit_invoice: { purpose: "build contract", amount: "1", payer: wallet.address, days: 6, recurrent_time: 1, token: { native: "uscrt" }},
          },
          sent_funds: [], // optional
        },
        {
          gasLimit: 100_000,
        }
      );
      console.log(tx);
    } catch (error) {
      console.log(error);
    }
};

submit_invoice()