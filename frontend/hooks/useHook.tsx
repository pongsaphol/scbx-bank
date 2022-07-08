import React, {
  Fragment,
  useContext,
  useState,
  useEffect,
  useRef,
} from "react";
import { Dialog, Transition } from "@headlessui/react";
import { ExclamationIcon, XIcon } from "@heroicons/react/outline";
import * as query from "../utils/query";

import {
  useWallet,
  useConnectedWallet,
  WalletStatus,
} from "@terra-money/wallet-provider";

export interface IAccount {
  name: String;
  balance: String;
}

export interface OpenError {
  title: String;
  message: String;
}

export interface IPage {
  name: "Account" | "AddAccount" | "Deposit" | "Withdraw" | "Transfer";
  account?: String;
}

export interface IHookContext {
  update: boolean;
  setUpdate: (update: boolean) => void;
  account: IAccount[];
  setAccount: (account: IAccount[]) => void;
  page: IPage;
  setPage: (page: IPage) => void;
  prefetch: () => void;
  balance: String;
}

const initialContext: IHookContext = {
  update: true,
  setUpdate: () => {},
  account: [],
  setAccount: () => {},
  page: { name: "Account" },
  setPage: () => {},
  prefetch: () => {},
  balance: "0",
};

const HookContext = React.createContext<IHookContext>(initialContext);

export const useHook = () => {
  return useContext(HookContext);
};

export const HookProvider = ({ children }) => {
  const [update, setUpdate] = useState(true);
  const [account, setAccount] = useState<IAccount[]>([]);
  const [balance, setBalance] = useState<String>("0");
  const [page, setPage] = useState<IPage>({ name: "Account" });
  console.log("PAGE", page);
  const [localError, setLocalError] = useState<OpenError>({
    title: "",
    message: "",
  });

  const connectedWallet = useConnectedWallet();

  const prefetch = async () => {
    if (connectedWallet) {
      try {
        const { account }: any = await query.getAccount(connectedWallet);
        setAccount(
          await Promise.all(
            account.map(async (item: String) => {
              try {
                const { balance }: any = await query.getBalance(
                  connectedWallet,
                  item
                );
                return { name: item, balance };
              } catch (e) {
                return { name: item, balance: "0" };
              }
            })
          )
        );
      } catch (e) {
        setAccount([]);
      }
      try {
        const { balance }: any = await query.getTokenBalance(connectedWallet);
        setBalance(balance);
      } catch (e) {
        setBalance("0");
      }
    } else {
      setAccount([]);
      setBalance("0");
    }
    setUpdate(false);
  };

  useEffect(() => {
    prefetch();
  }, [connectedWallet]);

  const cancelButtonRef = useRef(null);

  return (
    <HookContext.Provider
      value={{
        update,
        setUpdate,
        account,
        setAccount,
        page,
        setPage,
        prefetch,
        balance,
      }}
    >
      {children}
    </HookContext.Provider>
  );
};
