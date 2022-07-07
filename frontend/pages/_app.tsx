import React from "react";
import { AppProps } from "next/app";

import "../styles/index.css";
import Layout from "../components/Layout";

function MyApp({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />;
}

export default MyApp;
