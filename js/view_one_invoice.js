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


  /// for query single invoice
let try_query_single_invoice = async (id) => {
    const my_query = await secretjs.query.compute.queryContract({
        contract_address: contract_address,
        code_hash: contractCodeHash,
        query: { single_invoice: {
          id: id,
          owner: wallet.address
        } },
    });

    console.log(my_query);
  };

  try_query_single_invoice(1);