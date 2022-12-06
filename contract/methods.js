const { exec } = require("child_process");

function serialize_params(obj) {
  // TODO: cast primitives to strings.
  return JSON.stringify(obj);
}

function serialize_args(obj) {
  return Object.entries(obj)
    .map(
      ([key, value]) => `${key.startsWith("--") ? key : "--" + key} ${value}`
    )
    .join(" ");
}

// prettier-ignore
const create_call = (contract, caller, method) => (params, args) => new Promise((resolve, reject) => {
    args = {
      "--accountId": caller,
      ...args,
    }
    const cmd = `near call ${contract} ${method} '${serialize_params(params)}' ${serialize_args(args)}`;
    console.info(cmd);

    const child = exec(cmd);
    child.stdout.on("data", console.log);
    child.stdout.on("data", resolve);
    child.stdout.on("error", reject);
});

// prettier-ignore
function get_methods(contract_address, caller_address) {
  const mint = create_call(contract_address, caller_address, "nft_mint");
  const init = create_call(contract_address, caller_address, "new");
  const transfer = create_call(contract_address, caller_address, "nft_transfer");

  return {
    mint,
    init,
    transfer,
  };
}

module.exports = get_methods;
