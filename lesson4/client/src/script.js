import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
//https://polkadot.js.org/common/util-crypto/modules/_blake2_ashex_.html
import { blake2AsHex } from '@polkadot/util-crypto';
//const { promises: { readFile } } = require('fs');
//const data = await readFile('artical.txt', 'utf8')
//它们返回 Promise 对象而不是使用回调
const fs = require('fs').promises;
const convert = (from, to) => str => Buffer.from(str, from).toString(to)
const utf8ToHex = convert('utf8', 'hex')
const hexToUtf8 = convert('hex', 'utf8')

// config
const WEB_SOCKET = 'ws://localhost:9944';

async function connect() {
  // Construct
  const wsProvider = new WsProvider(WEB_SOCKET);
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: {
      'Address': 'AccountId',
      'LookupSource': 'AccountId',
      'Price': 'u128',
    }
  });
  // Retrieve the chain & node information information via rpc calls
  //https://polkadot.js.org/api/examples/promise/01_simple_connect/
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version()
  ]);
  console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
  return api;
}

async function submitDocInfo(filePath, comment) {
  console.debug(`submitDocInfo: ${filePath}, ${comment}`);
  try {
    const api = await connect();
    const keyring = new Keyring({ type: 'sr25519' });
    // (Advanced, development-only) add with an implied dev seed and hard derivation
    //https://polkadot.js.org/api/start/keyring.html
    const alice = keyring.addFromUri('//Alice', { name: 'Alice' });
    /******
     * 学员们在这里追加逻辑
     * 把 filePath 档档案通过 hash 函数算出它的 hash 值。然后和 comment 一起提交个 extrinsics
     *   到 Substrate。
     ******/
    //function readFile(path: string | Buffer | URL | FileHandle, options: "binary" |
    //encoding: "utf8",flag: "r"}
    let data = await fs.readFile(filePath, "binary");
    data = blake2AsHex(data);
    //https://polkadot.js.org/api/api-contract/interfaces/_base_blueprint_.blueprintcreate.html#signandsend
    const txHash = await api.tx.poeModule.createClaim(data, comment).signAndSend(alice);
    console.log(`sent tx ${txHash} with proof's hash(${data}) and memo(${comment}) by ${alice.address}`);
  } catch (err) {
    console.error(`Connect to Substrate error:`, err);
    process.exit(1);
  }

  process.exit(0);
}

async function getUserDocs(acct) {
  console.debug(`getUserDocs: ${acct}`);
  try {
    const api = await connect();
    /******
     * 学员们在这里追加逻辑
     *
     * 通过用户 addr, 取得他所有的创建文件的 hash及相关资料。返回值是：
     * {
     *   "0xabcd1234...": ["my note1", 3],
     *   "0xabcd1235...": ["my note2", 5],
     *   "0xabcd1236...": ["my note3", 7],
     *   ...
     * }
     *
     * 创建:
     * $ yarn submitDocInfo package.json  package.json
     * $ yarn submitDocInfo yarn.lock
     * $ yarn submitDocInfo .gitignore .gitignore
     *
     * 查询:
     * $ yarn getUserDocs 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
     *
     * "0x002a9088460254c69151b508e1f63bfe018113018ba722a406bd06a1adf4e1d7": ["package.json", 2]
     * "0x9cdd313a8a541fffc227fcbbb885ad51acaaaeebbe11e9dde0cd7f931b1c5263": ["", 51]
     * "0x84488f9a3c3a0806c3efa4bba9bb41338b2cc971ae95058949d03be7a31d0d8e": [".gitignore", 56]
     ******/
    //AccountProofs get(fn get_account_proofs): map hasher(blake2_128_concat) T::AccountId => Vec<Vec<u8>>;
    const claims = await api.query.poeModule.accountProofs(acct);
    for (const c of claims) {// https://exploringjs.com/es6/ch_for-of.html
      let info = await api.query.poeModule.proofs(c);
      info = info.toJSON();
      const memo = hexToUtf8(info[2].slice(2));
      const blockNumber = info[1];
      console.log(`"${c}": ["${memo}", ${blockNumber}]`)
    }

  } catch (err) {
    console.error(`Connect to Substrate error:`, err);
  }

  process.exit(0);
}

function main() {
  //https://nodejs.org/en/knowledge/command-line/how-to-parse-command-line-arguments/
  const args = process.argv.slice(2, 5);
  switch (args[0]) {
    case 'submitDocInfo':
      submitDocInfo(args[1], args[2])
      break;
    case 'getUserDocs':
      getUserDocs(args[1]);
      break;
    default:
      console.error('Unknown subcommand. Please use `submitDocInfo` or `getUserDocs` only.')
  }
}

main();
