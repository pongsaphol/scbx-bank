import { LCDClient } from "@terra-money/terra.js";
import { ConnectedWallet } from "@terra-money/wallet-provider";
import { contractAddress, tokenAddress } from "./address";

export const getAccount = async (wallet: ConnectedWallet) => {
  const lcd = new LCDClient({
    URL: wallet.network.lcd,
    chainID: wallet.network.chainID,
  });
  return lcd.wasm.contractQuery(contractAddress, {
    get_account: {
      address: wallet.walletAddress,
    },
  });
};

export const getBalance = async (wallet: ConnectedWallet, account: String) => {
  const lcd = new LCDClient({
    URL: wallet.network.lcd,
    chainID: wallet.network.chainID,
  });
  return lcd.wasm.contractQuery(contractAddress, {
    get_balance: {
      account,
    },
  });
};

export const getTokenBalance = async (wallet: ConnectedWallet) => {
  const lcd = new LCDClient({
    URL: wallet.network.lcd,
    chainID: wallet.network.chainID,
  });
  return lcd.wasm.contractQuery(tokenAddress, {
    balance: {
      address: wallet.walletAddress,
    },
  });
};
