const { exec } = require("child_process");

function serialize_params(obj) {
  // TODO: cast primitives to strings.
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

function get_methods(contract_address, caller_address) {
  const mint = create_call(contract_address, caller_address, "nft_mint");
  const init = create_call(contract_address, caller_address, "new");

  return {
    mint,
    init,
  };
}

module.exports = get_methods;
