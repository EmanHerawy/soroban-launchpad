"use client";

import { Box, Typography } from "@mui/material";

import PDFViewer from "@/components/PDFViewer";

import "./style.css";

export default function Home() {
  return (
    <main>
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
        }}
      >
        <Typography
          variant={"h2"}
          sx={{ mb: 10, fontFamily: "inherit", color: "var(--yellow)" }}
        >
          Comic
        </Typography>
        <PDFViewer url="https://ipfs.io/ipfs/QmdHt1ZkRdrkB8QoDKEL5tm5w4HThmHkaTrhUvS2va1uR7" />

        <Typography
          variant={"h2"}
          sx={{ mb: 10, mt: 10, fontFamily: "inherit", color: "var(--yellow)" }}
        >
          Playing Card
        </Typography>
        <img
          src="https://ipfs.io/ipfs/QmeT7yJ9RPcR5x9a1kXnzgJJ2BTsh6aMvBwsMCjcQk6fsx"
          alt="Playing Card"
          width={500}
          style={{ margin: "auto" }}
        />
      </Box>
    </main>
  );
}
