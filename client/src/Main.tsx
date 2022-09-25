import React from "react";
import ReactDOM from "react-dom/client";
import Koi from "./Koi";
import { ChakraProvider, extendTheme } from "@chakra-ui/react";
import "@fontsource/inter";

// Chakra
const theme = extendTheme({
  colors: {
    red: {
      "50": "#FFE5EC",
      "100": "#FFB8C9",
      "200": "#FF8AA7",
      "300": "#FF5C84",
      "400": "#FF2E62",
      "500": "#FF003F",
      "600": "#CC0033",
      "700": "#990026",
      "800": "#660019",
      "900": "#33000D",
    },
  },
  fonts: {
    heading: `'Inter', sans-serif`,
    body: `'Inter', sans-serif`,
  },
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ChakraProvider theme={theme}>
      <Koi />
    </ChakraProvider>
  </React.StrictMode>
);
