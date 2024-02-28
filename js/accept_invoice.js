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

 
//for accepting invoice
let accept_invoice = async (id) => {
    
    try {
      let tx = await secretjs.tx.compute.executeContract(
        {
          contract_address: contract_address,
          code_hash: contractCodeHash, // optional but way faster
          msg: {
            accept_invoice: { id: id},
          },
      
          sender: wallet.address,
          sent_funds: [{ denom: "uscrt", amount: "10" }]
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

  accept_invoice(1);
  