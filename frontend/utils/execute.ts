import { LCDClient, MsgExecuteContract, Fee, Tx } from "@terra-money/terra.js";
import { ConnectedWallet } from "@terra-money/wallet-provider";
import { contractAddress, tokenAddress } from "./address";

// ==== utils ====

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
const until = Date.now() + 1000 * 60 * 60;
const untilInterval = Date.now() + 1000 * 60;

const _exec = (msg: any) => async (wallet: ConnectedWallet) => {
  const lcd = new LCDClient({
    URL: wallet.network.lcd,
    chainID: wallet.network.chainID,
  });
  console.log("msg", msg);

  const { result } = await wallet.post({
    msgs: [new MsgExecuteContract(wallet.walletAddress, contractAddress, msg)],
  });

  while (true) {
    try {
      return await lcd.tx.txInfo(result.txhash);
    } catch (e) {
      if (Date.now() < untilInterval) {
        await sleep(500);
      } else if (Date.now() < until) {
        await sleep(1000 * 10);
      } else {
        throw new Error(
          `Transaction queued. To verify the status, please check the transaction hash: ${result.txhash}`
        );
      }
    }
  }
};

const _exec_token = (msg: any) => async (wallet: ConnectedWallet) => {
  const lcd = new LCDClient({
    URL: wallet.network.lcd,
    chainID: wallet.network.chainID,
  });
  console.log("msg", msg);

  const { result } = await wallet.post({
    msgs: [new MsgExecuteContract(wallet.walletAddress, tokenAddress, msg)],
  });

  while (true) {
    try {
      return await lcd.tx.txInfo(result.txhash);
    } catch (e) {
      if (Date.now() < untilInterval) {
        await sleep(500);
      } else if (Date.now() < until) {
        await sleep(1000 * 10);
      } else {
        throw new Error(
          `Transaction queued. To verify the status, please check the transaction hash: ${result.txhash}`
        );
      }
    }
  }
};

const to_base64 = (obj: any) => btoa(JSON.stringify(obj));

// ==== execute contract ====

export const add_account = async (wallet: ConnectedWallet, name: String) =>
  _exec({ create_account: { account_name: name } })(wallet);

export const deposit = async (
  wallet: ConnectedWallet,
  balance: Number,
  account: String
) =>
  _exec_token({
    send: {
      amount: balance.toString(),
      contract:
        "terra1ll73q330keasce8l9vrcg5064tdlzzm0ljtyd3xrhruqcu006s7qp63mxh",
      msg: to_base64({ deposit: { account } }),
    },
  })(wallet);

export const withdraw = async (
  wallet: ConnectedWallet,
  amount: Number,
  account: String
) => _exec({ withdraw: { amount: amount.toString(), account } })(wallet);

export const transfer = async (
  wallet: ConnectedWallet,
  amount: Number,
  from: String,
  to: String
) => _exec({ transfer: { amount: amount.toString(), from, to } })(wallet);
