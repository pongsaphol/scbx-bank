import { useEffect, useState, Fragment, useRef, useMemo } from "react";
import Link from "next/link";
import Layout from "../components/Layout";

import {
  useWallet,
  useConnectedWallet,
  WalletStatus,
} from "@terra-money/wallet-provider";

import * as execute from "../utils/execute";
import { useHook } from "../hooks/useHook";

const AddAccount = () => {
  const [name, setName] = useState("");
  const { setPage, setError, prefetch } = useHook();
  const connectedWallet = useConnectedWallet();
  const onClickName = async () => {
    if (connectedWallet) {
      try {
        await execute.add_account(connectedWallet, name);
        setName("");
        prefetch();
        setPage({ name: "Account" });
      } catch (e) {
        setName("");
        setError({
          title: "Account already exists",
          message: "Please use another name",
        });
      }
    }
  };
  return (
    <>
      <div className="flex flex-col w-screen items-center">
        <div className="w-full max-w-3xl my-6">
          <p className="text-lg">Create Your Bank Account</p>
        </div>
        <div className="flex flex-col w-full max-w-3xl items-end">
          <div className="flex flex-col border border-gray-300 w-full px-10 py-4 items-end">
            <div className="flex w-full items-center">
              <p className="text-lg pr-4 flex-none">Account Name: </p>
              <input
                className="border border-gray-300 rounded-md px-4 py-1 w-full"
                type="text"
                onChange={(e) => setName(e.target.value)}
                value={name}
              />
            </div>
            <button
              className="mt-6 inline-flex items-center justify-center px-2.5 py-1.5 border border-gray-300 shadow-sm text-md w-32 font-medium rounded text-gray-700 bg-white hover:bg-gray-50"
              onClick={onClickName}
              type="button"
            >
              Create
            </button>
          </div>
          <button
            className="inline-flex items-center justify-center px-2.5 py-1.5 border border-gray-300 shadow-sm text-md w-36 font-medium rounded text-gray-700 bg-white hover:bg-gray-50 my-4"
            onClick={() => setPage({ name: "Account" })}
          >
            Go Back
          </button>
        </div>
      </div>
    </>
  );
};

const AccountCard = ({ name, balance }) => {
  const { setPage } = useHook();
  return (
    <div className="flex flex-col border border-gray-300 w-full my-4">
      <div className="grid w-full h-full grid-cols-2 grid-rows-2 max-w-lg px-10 py-6">
        <p className="text-lg">Account Name: </p>
        <p className="text-lg font-bold">{name}</p>
        <p className="text-lg">Balance: </p>
        <p className="text-lg font-bold">{`${balance} TOKEN`}</p>
      </div>
      <div className="grid w-full grid-cols-3 grid-rows-1">
        <button
          className="border border-gray-300 py-2 text-lg hover:bg-gray-100"
          onClick={() => setPage({ name: "Deposit", account: name })}
        >
          Deposit
        </button>
        <button
          className="border border-gray-300 py-2 text-lg hover:bg-gray-100"
          onClick={() => setPage({ name: "Withdraw", account: name })}
        >
          Withdraw
        </button>
        <button
          className="border border-gray-300 py-2 text-lg hover:bg-gray-100"
          onClick={() => setPage({ name: "Transfer", account: name })}
        >
          Transfer
        </button>
      </div>
    </div>
  );
};

const Account = () => {
  const { update, account, setPage, balance } = useHook();
  return (
    <div className="flex flex-col w-screen items-center">
      <div className="w-full max-w-3xl my-6">
        <p className="text-lg">My Accounts: {balance} TOKEN</p>
      </div>
      <div className="flex flex-col w-full max-w-3xl items-center">
        {update ? (
          <svg
            className="animate-spin -ml-1 mr-3 h-10 w-10 text-gray-500"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              className="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            ></circle>
            <path
              className="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            ></path>
          </svg>
        ) : (
          <>
            {account.map((item: any) => (
              <AccountCard {...item} />
            ))}
            <button
              className="flex flex-col border border-gray-300 w-full my-4 h-36 items-center justify-center hover:bg-gray-100"
              onClick={() => setPage({ name: "AddAccount", account: null })}
            >
              <p className="font-semibold text-xl">+ Create Bank Account</p>
            </button>
          </>
        )}
      </div>
    </div>
  );
};

const Deposit = ({ account }) => {
  const [balance, setBalance] = useState(null);
  const { setPage, setError, prefetch } = useHook();
  const connectedWallet = useConnectedWallet();
  const onClickName = async () => {
    if (connectedWallet) {
      try {
        await execute.deposit(connectedWallet, balance, account);
        setBalance(null);
        prefetch();
        setPage({ name: "Account" });
      } catch (e) {
        setBalance(null);
        setError({
          title: "Deposit Error",
          message: "Insufficient balance",
        });
      }
    }
  };
  return (
    <div className="flex flex-col w-screen items-center">
      <div className="w-full max-w-3xl my-6">
        <p className="text-lg">Deposit:</p>
      </div>
      <div className="flex flex-col w-full max-w-3xl items-end">
        <div className="flex flex-col border border-gray-300 w-full my-4 items-end">
          <div className="grid w-full h-full grid-cols-3 grid-rows-2 px-10 py-6">
            <p className="text-lg">To: </p>
            <p className="text-lg font-bold col-span-2">{account}</p>
            <p className="text-lg">Amount: </p>
            <div className="flex w-full col-span-2">
              <input
                type="number"
                className="border border-gray-300 rounded-md px-4 py-1 w-full"
                value={balance}
                onChange={(e) => setBalance(+e.target.value)}
              />
              <p className="text-lg flex items-center pl-4">TOKEN</p>
            </div>
          </div>
          <button
            className="mb-6 mr-10 inline-flex items-center justify-center px-2.5 py-1.5 border border-gray-300 shadow-sm text-md w-32 font-medium rounded text-gray-700 bg-white hover:bg-gray-50"
            onClick={onClickName}
            type="button"
          >
            Deposit
          </button>
        </div>
        <button
          className="inline-flex items-center justify-center px-2.5 py-1.5 border border-gray-300 shadow-sm text-md w-36 font-medium rounded text-gray-700 bg-white hover:bg-gray-50 my-4"
          onClick={() => setPage({ name: "Account" })}
        >
          Go Back
        </button>
      </div>
    </div>
  );
};

const Withdraw = ({ account }) => {
  const [balance, setBalance] = useState(null);
  const { setPage, setError, prefetch } = useHook();
  const connectedWallet = useConnectedWallet();
  const onClickName = async () => {
    if (connectedWallet) {
      try {
        await execute.withdraw(connectedWallet, balance, account);
        setBalance(null);
        prefetch();
        setPage({ name: "Account" });
      } catch (e) {
        setBalance(null);
        setError({
          title: "Withdraw Error",
          message: "Insufficient balance",
        });
      }
    }
  };
  return (
    <div className="flex flex-col w-screen items-center">
      <div className="w-full max-w-3xl my-6">
        <p className="text-lg">Withdraw:</p>
      </div>
      <div className="flex flex-col w-full max-w-3xl items-end">
        <div className="flex flex-col border border-gray-300 w-full my-4 items-end">
          <div className="grid w-full h-full grid-cols-3 grid-rows-2 px-10 py-6">
            <p className="text-lg">From: </p>
            <p className="text-lg font-bold col-span-2">{account}</p>
            <p className="text-lg">Amount: </p>
            <div className="flex w-full col-span-2">
              <input
                type="number"
                className="border border-gray-300 rounded-md px-4 py-1 w-full"
                value={balance}
                onChange={(e) => setBalance(+e.target.value)}
              />
              <p className="text-lg flex items-center pl-4">TOKEN</p>
            </div>
          </div>
          <button
            className="mb-6 mr-10 inline-flex items-center justify-center px-2.5 py-1.5 border border-gray-300 shadow-sm text-md w-32 font-medium rounded text-gray-700 bg-white hover:bg-gray-50"
            onClick={onClickName}
            type="button"
          >
            Withdraw
          </button>
        </div>
        <button
          className="inline-flex items-center justify-center px-2.5 py-1.5 border border-gray-300 shadow-sm text-md w-36 font-medium rounded text-gray-700 bg-white hover:bg-gray-50 my-4"
          onClick={() => setPage({ name: "Account" })}
        >
          Go Back
        </button>
      </div>
    </div>
  );
};

const Transfer = ({ account: accountFrom }) => {
  const [balance, setBalance] = useState(null);
  const [accountTo, setAccountTo] = useState(null);
  const { setPage, setError, prefetch, account } = useHook();
  const connectedWallet = useConnectedWallet();
  const feeText = useMemo(() => {
    if (balance) {
      for (let i = 0; i < account.length; i++) {
        if (account[i].name === accountTo) {
          return "no fee";
        }
      }
      const fee = (balance - (balance % 100)) / 100;
      return `fee 1% = ${fee} TOKEN | Receive = ${balance - fee} TOKEN`;
    }
    return "";
  }, [balance, accountTo]);
  const onClickName = async () => {
    if (connectedWallet) {
      try {
        await execute.transfer(
          connectedWallet,
          balance,
          accountFrom,
          accountTo
        );
        setBalance(null);
        prefetch();
        setPage({ name: "Account" });
      } catch (e) {
        setBalance(null);
        setError({
          title: "Transfer Error",
          message: "Please check To account or balance",
        });
      }
    }
  };
  return (
    <div className="flex flex-col w-screen items-center">
      <div className="w-full max-w-3xl my-6">
        <p className="text-lg">Transfer:</p>
      </div>
      <div className="flex flex-col w-full max-w-3xl items-end">
        <div className="flex flex-col border border-gray-300 w-full my-4 items-end">
          <div className="grid w-full h-full grid-cols-3 grid-rows-3 px-10 py-6 gap-y-2">
            <p className="text-lg">From: </p>
            <p className="text-lg font-bold col-span-2">{accountFrom}</p>
            <p className="text-lg">To: </p>
            <input
              type="text"
              className="border border-gray-300 rounded-md px-4 py-1 w-full col-span-2"
              value={accountTo}
              onChange={(e) => setAccountTo(e.target.value)}
            />
            <p className="text-lg">Amount: </p>
            <div className="flex w-full col-span-2">
              <input
                type="number"
                className="border border-gray-300 rounded-md px-4 py-1 w-full"
                value={balance}
                onChange={(e) => setBalance(+e.target.value)}
              />
              <p className="text-lg flex items-center pl-4">TOKEN</p>
            </div>
          </div>
          <div className="flex justify-between w-full pl-10 items-end">
            <p className="text-yellow-800 mb-6">{feeText}</p>
            <button
              className="mb-6 mr-10 inline-flex items-center justify-center px-2.5 py-1.5 border border-gray-300 shadow-sm text-md w-32 font-medium rounded text-gray-700 bg-white hover:bg-gray-50"
              onClick={onClickName}
              type="button"
            >
              Transfer
            </button>
          </div>
        </div>
        <div className="flex justify-between w-full pl-4">
          <ul className="list-disc text-yellow-800">
            <li>1% fee, If you transfer to other account</li>
            <li>No fee, If you tranfer to your account</li>
          </ul>
          <button
            className="inline-flex items-center justify-center px-2.5 py-1.5 border border-gray-300 shadow-sm text-md w-36 font-medium rounded text-gray-700 bg-white hover:bg-gray-50 my-4"
            onClick={() => setPage({ name: "Account" })}
          >
            Go Back
          </button>
        </div>
      </div>
    </div>
  );
};

const IndexPage = () => {
  const { page } = useHook();
  return (
    <>
      {page.name === "Account" ? (
        <Account />
      ) : page.name === "Deposit" ? (
        <Deposit account={page.account} />
      ) : page.name === "AddAccount" ? (
        <AddAccount />
      ) : page.name === "Withdraw" ? (
        <Withdraw account={page.account} />
      ) : (
        <Transfer account={page.account} />
      )}
    </>
  );
};

const Mock = () => (
  <Layout>
    <IndexPage />
  </Layout>
);

export default Mock;
