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
let try_query_page_contract = async (page, pageSize) => {
    const my_query = await secretjs.query.compute.queryContract({
        contract_address: contract_address,
        code_hash: contractCodeHash,
        query: { paginated_contract: {
          payer: wallet.address,
          page: page,
          page_size: pageSize
        } },
    });
  
    console.log(my_query);
    console.log(my_query[0][1].invoice);

  };

  try_query_page_contract(0, 7);
