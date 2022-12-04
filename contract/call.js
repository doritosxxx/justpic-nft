const { promises: fs } = require("fs");
const get_methods = require("./methods");

const caller_address = "2281337.testnet";

async function main() {
  const contract_address = await fs
    .readFile("./scripts_cache/deploy.address")
    .then((buffer) => buffer.toString());

  if (!contract_address) {
    throw "address is not defined";
  }

  const { mint } = get_methods(contract_address, caller_address);

  await mint({
    token_id: "4",
    receiver_id: caller_address,
  });
}

main();
