import React, { ReactNode, useEffect, useState } from "react";
import { getChainOptions, WalletProvider } from "@terra-money/wallet-provider";
import { ConnectWallet } from "../components/ConnectWallet";
import { useHook, HookProvider } from "../hooks/useHook";

import Head from "next/head";

type Props = {
  children?: ReactNode;
  title?: string;
};

const Layout = ({ children }) => {
  const [state, setState] = useState(null);

  useEffect(() => {
    (async () => {
      const tmp = await getChainOptions();
      setState(tmp);
    })();
  }, []);

  if (state === null) return <div>Loading...</div>;

  return (
    <WalletProvider {...state}>
      <Head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="initial-scale=1.0, width=device-width" />
      </Head>
      <HookProvider>
        <header>
          <nav className="flex w-screen justify-center my-6">
            <div className="flex w-full max-w-5xl justify-between">
              <p className="font-bold text-3xl">🚀10XBank</p>
              <ConnectWallet />
            </div>
          </nav>
        </header>
        {children}
      </HookProvider>
    </WalletProvider>
  );
};

export default Layout;
