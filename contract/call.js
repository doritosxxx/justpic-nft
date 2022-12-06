const { promises: fs } = require("fs");
const get_methods = require("./methods");

const caller_address = "2281337.testnet";

async function main() {
  const contract_address = await fs
    .readFile("./neardev/dev-account")
    .then((buffer) => buffer.toString());

  if (!contract_address) {
    throw "address is not defined";
  }

  const { mint, init } = get_methods(contract_address, caller_address);

  await mint({
    token_id: "4",
    receiver_id: caller_address,
  });

  // await init({
  //   owner_id: caller_address,
  // });
}

main();
