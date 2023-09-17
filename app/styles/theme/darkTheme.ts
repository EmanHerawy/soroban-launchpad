"use client";
import { createTheme } from "@mui/material/styles";

const darkTheme = createTheme({
  palette: {
    mode: "dark",
  },
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          "font-family": "inherit",
          color: "#fff",
        },
        contained: {
          background: "var(--yellow)",
          color: "#000",
          fontWeight: "bold",
          transition: "all 200ms ease",
          boxShadow: "inset 0px -2px var(--orange)",
          ":hover": {
            background: "var(--yellow)",
            transform: "translateY(-2px)",
            boxShadow:
              "0 20px 20px -15px var(--dark-70), 0 0 5px var(--dark), inset 0px -2px var(--orange)",
          },
        },
      },
    },
  },
});

export default darkTheme;
