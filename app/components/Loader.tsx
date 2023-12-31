import { Typography } from "@mui/material";
import { useState, useEffect } from "react";

const Loader = () => {
  const [dots, setDots] = useState("");

  useEffect(() => {
    const interval = setInterval(() => {
      setDots((prevDots) => (prevDots.length < 3 ? prevDots + "." : ""));
    }, 500);

    return () => clearInterval(interval);
  }, []);

  return (
    <Typography variant="h5" sx={{ fontFamily: "inherit" }}>
      Loading{dots}
    </Typography>
  );
};

export default Loader;
