const { promises: fs } = require("fs");
const { exec } = require("child_process");

const caller_address = "2281337.testnet";

function serialize_params(obj) {
  return JSON.stringify(obj);
}

// prettier-ignore
const create_call = (contract, caller, method) => (params) =>new Promise((resolve, reject) => {
    const cmd = `near call ${contract} ${method} '${serialize_params(params)}' --accountId ${caller}`;
    console.info(cmd);

    const child = exec(cmd);
    child.stdout.on("data", console.log);
    child.stdout.on("data", resolve);
    child.stdout.on("error", reject);
});

async function main() {
  const contract_address = await fs
    .readFile("./scripts_cache/deploy.address")
    .then((buffer) => buffer.toString());

  if (!contract_address) {
    throw "address is not defined";
  }

  const mint = create_call(contract_address, caller_address, "nft_mint");
  const init = create_call(contract_address, caller_address, "new");

  await init({ owner_id: caller_address });

  await mint({
    token_id: "1",
    receiver_id: caller_address,
  });
}

main();
