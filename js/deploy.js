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

//console.log(secretjs);

let upload_contract = async () => {
    let tx = await secretjs.tx.compute.storeCode(
      {
        sender: wallet.address,
        wasm_byte_code: contract_wasm,
        source: "",
        builder: "",
      },
      {
        gasLimit: 4_000_000,
      }
    );

    //console.log(tx);
  
    const codeId = Number(
      tx.arrayLog.find((log) => log.type === "message" && log.key === "code_id")
        .value
    );
  
    console.log("codeId: ", codeId);
  
    // const contractCodeHash = (
    //   await secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
    // ).code_hash;
    // console.log(`Contract hash: ${contractCodeHash}`);

    try {
      
      const contractCodeHash = (
        await secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
      ).code_hash;
      console.log(`Contract hash: ${contractCodeHash}`);

    } catch (error) {
      console.log(error);
    }
    
};
  
//upload_contract();

let codeId = 21044;
let contractCodeHash = 'add0d4c751f7503a564031dab3b31007c696382d732c88fb95d4d07aee4c5fc8';

let instantiate_contract = async () => {
    // Create an instance of the Counter contract, providing a starting count
    const initMsg = { };
    let tx = await secretjs.tx.compute.instantiateContract(
      {
        code_id: codeId,
        sender: wallet.address,
        code_hash: contractCodeHash,
        init_msg: initMsg,
        label: "My payment" + Math.ceil(Math.random() * 10000),
      },
      {
        gasLimit: 400_000,
      }
    );
      
    //console.log(tx);

    //Find the contract_address in the logs
    const contractAddress = tx.arrayLog.find(
      (log) => log.type === "message" && log.key === "contract_address"
    ).value;
  
    console.log(contractAddress);
};
  
//instantiate_contract();