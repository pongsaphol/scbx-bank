import { useWallet, WalletStatus } from "@terra-money/wallet-provider";
import { useHook } from "../hooks/useHook";

const format_address = (address: String) => {
  return address.slice(0, 8) + "..." + address.slice(-4);
};

const HeadButton = ({ children, onClick }) => (
  <button
    className="inline-flex items-center justify-center px-2.5 py-1.5 border border-gray-300 shadow-sm text-md w-36 font-medium rounded text-gray-700 bg-white hover:bg-gray-50"
    onClick={onClick}
  >
    {children}
  </button>
);

export const ConnectWallet = () => {
  const {
    status,
    network,
    wallets,
    availableConnectTypes,
    availableConnections,
    connect,
    disconnect,
  } = useWallet();

  const { setUpdate } = useHook();

  return (
    <div>
      {status === WalletStatus.WALLET_NOT_CONNECTED && (
        <>
          <HeadButton
            onClick={() => {
              setUpdate(true);
              connect(availableConnectTypes[0]);
            }}
          >
            Connect
          </HeadButton>
        </>
      )}
      {status === WalletStatus.WALLET_CONNECTED && (
        <HeadButton onClick={() => disconnect()}>
          {format_address(wallets[0].terraAddress)}
        </HeadButton>
      )}
    </div>
  );
};
