import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const contract_wasm = fs.readFileSync("../contract.wasm");

const secretjs = new SecretNetworkClient({
    chainId: "pulsar-3",
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

let codeId = 3715;
let contractCodeHash = '9e86cfe59365a51f526d8163ac7f4bb84fd28c12f2e32428716cee1b57bf74d3';

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
    
  //console.log('tx', tx);

  // Find the contract_address in the logs
  const contractAddress = tx.arrayLog.find(
    (log) => log.type === "message" && log.key === "contract_address"
  ).value;

  console.log(contractAddress);
  
    
};
  
//instantiate_contract();

let contract_address = 'secret1qctcuh4yv5cufpm9t97p8zmdn07sgtqu080cqj';

// for query single invoice
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

// for query page invoice
let try_query_page_invoice = async (page, limit) => {
  const my_query = await secretjs.query.compute.queryContract({
      contract_address: contract_address,
      code_hash: contractCodeHash,
      query: { paginated_invoice: {
        owner: wallet.address,
        page: page,
        page_size: limit
      } },
  });

  console.log(my_query);
};
  


// for query number of invoice
let try_query_all_invoice = async () => {
  const my_query = await secretjs.query.compute.queryContract({
      contract_address: contract_address,
      code_hash: contractCodeHash,
      query: { number_of_invoice: {
        owner: "secret1kycte7gyu3mw00km97w0suu9z5cvt6edqyt095"
      } },
  });

  console.log(my_query);
};

// for query admin account
let try_query_admin_account = async () => {
  const my_query = await secretjs.query.compute.queryContract({
      contract_address: contract_address,
      code_hash: contractCodeHash,
      query: { admim_wallet: {
       
      } },
  });

  console.log(my_query);
};

//for submiting invoice
let add_new_invoice = async () => {
    
  try {
    let tx = await secretjs.tx.compute.executeContract(
      {
        sender: wallet.address,
        contract_address: contract_address,
        code_hash: contractCodeHash, // optional but way faster
        msg: {
          submit_invoice: { purpose: "build contract", amount: "800", admin_charge: "50", customer_charge: "89", payer: "secret1kycte7gyu3mw00km97w0suu9z5cvt6edqyt095", days: 6, recurrent_time: 2, token: { native: "uscrt" }},
        },
        sentFunds: [], // optional
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


//for withdraw payment
let withdraw_payment = async (id) => {
    
  try {
    let tx = await secretjs.tx.compute.executeContract(
      {
        sender: wallet.address,
        contract_address: contract_address,
        code_hash: contractCodeHash, // optional but way faster
        msg: {
          withdraw_payment: { id: id},
        },
        sentFunds: [], // optional
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

let payer = "secret1py4ryg3atyz5cru2m64p0mtga5y09q5a26pa7n"

// for query single contract
let try_query_single_contract = async (id) => {
  const my_query = await secretjs.query.compute.queryContract({
      contract_address: contract_address,
      code_hash: contractCodeHash,
      query: { single_contract: {
        id: id,
        payer: wallet.address
      } },
  });

  console.log(my_query);
};

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
        sent_funds: [{ denom: "uscrt", amount: "1700" }]
      },
      {
        gasLimit: 100_000,
      }
    );
    console.log(tx);
  } catch (error) {
    //console.log(error);
  }
};
  
  
//for cancelling payment
let cancel_payment = async (id) => {
    
  try {
    let tx = await secretjs.tx.compute.executeContract(
      {
        sender: wallet.address,
        contract_address: contract_address,
        code_hash: contractCodeHash, // optional but way faster
        msg: {
          cancel_payment: { id: id},
        },
        sentFunds: [], // optional
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


//for changing admin account
let change_admin_account = async () => {
    
  try {
    let tx = await secretjs.tx.compute.executeContract(
      {
        sender: wallet.address,
        contract_address: contract_address,
        code_hash: contractCodeHash, // optional but way faster
        msg: {
          admin_update_amin: { newAdmin: "secret1kycte7gyu3mw00km97w0suu9z5cvt6edqyt095"},
        },
        sentFunds: [], // optional
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
