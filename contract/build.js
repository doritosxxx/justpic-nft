const { exec, spawn } = require("child_process");
const { promises: fs } = require("fs");

const build = () =>
  new Promise((resolve, reject) => {
    exec("bash build.sh", (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });

const deploy = () =>
  new Promise((resolve, reject) => {
    let contract_address = null;
    const child = spawn("near", ["dev-deploy", "out/main.wasm"]);
    child.stderr.on("data", (chunk) => {
      throw chunk.toString();
    });
    child.stdout.on("data", (chunk) => console.log(chunk.toString()));
    child.stdout.on("data", (chunk) => {
      const match = `${chunk}`.trim().match(/^Done deploying to (.+)$/);
      if (match && match[1]) {
        contract_address = match[1];
      }
    });

    child.on("close", () => {
      if (contract_address) {
        resolve(contract_address);
      } else {
        reject("address was not found");
      }
    });
  });

async function main() {
  await build();
  const address = await deploy();
  console.log(address);
  await fs.writeFile("./scripts_cache/deploy.address", address);
}

main();
