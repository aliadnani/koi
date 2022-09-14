import React from "react";
import ReactDOM from "react-dom/client";
import Koi from "./Koi";
import { ChakraProvider, extendTheme } from "@chakra-ui/react";
import "@fontsource/inter";

// Chakra
const theme = extendTheme({
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
