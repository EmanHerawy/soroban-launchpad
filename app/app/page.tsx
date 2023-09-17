"use client";

import { Box, Typography } from "@mui/material";

import HomeLinks from "../components/HomeLinks";
import Projects from "../components/Projects";
import TokenProjectLaunch from "../components/TokenProjectLaunch";
import ProjectLaunch from "../components/NFTProjectLaunch";

export default function Home() {
  return (
    <main>
      <Box sx={{ textAlign: "center", mb: 10, mt: 10 }}>
        <Typography sx={titleStyle}>SoroPad</Typography>
      </Box>
        <Box sx={{ textAlign: "center", mb: 10, mt: 10 }}>
        <Typography sx={elementTitleStyle}>Launch your Project</Typography>
      </Box>
      <ProjectLaunch />
      <TokenProjectLaunch/>
  <Box sx={{ textAlign: "center", mb: 10, mt: 10 }}>
        <Typography sx={elementTitleStyle}>Live Projects</Typography>
      </Box>
      <Projects />

      <HomeLinks />
    </main>
  );
}

const titleStyle = {
  fontFamily: "inherit",
  fontSize: { xs: "60px", md: "100px" },
  fontWeight: "bold",
  color: "var(--yellow)",
  textShadow: {
    xs: [-1, 2, 3].map((val) => `${val}px ${val}px var(--orange)`).join(", "),
    md: [-1, 2, 3, 4, 5, 6, 7, 8]
      .map((val) => `${val}px ${val}px var(--orange)`)
      .join(", "),
  },
  letterSpacing: "4px",
  lineHeight: "1.5",
  marginBottom: { xs: "20px", md: "50px" },
};
const elementTitleStyle = {
  fontFamily: "inherit",
  fontSize: { xs: "30px", md: "50px" },
  fontWeight: "bold",
  color: "var(--green)",
  textShadow: {
    xs: [-1, 2, 3].map((val) => `${val}px ${val}px var(--orange)`).join(", "),
    md: [-1, 2, 3, 4, 5, 6, 7, 8]
      .map((val) => `${val}px ${val}px var(--orange)`)
      .join(", "),
  },
  letterSpacing: "4px",
  lineHeight: "1.5",
  marginBottom: { xs: "20px", md: "50px" },
};
