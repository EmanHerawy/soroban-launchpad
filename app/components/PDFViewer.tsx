import { Box, Button } from "@mui/material";
import React, { useState } from "react";
import { Document, Page, pdfjs } from "react-pdf";

// Set up the worker
pdfjs.GlobalWorkerOptions.workerSrc = `//cdnjs.cloudflare.com/ajax/libs/pdf.js/${pdfjs.version}/pdf.worker.min.js`;

interface PDFViewerProps {
  url: string;
}

const PDFViewer: React.FC<PDFViewerProps> = ({ url }) => {
  const [numPages, setNumPages] = useState<number | null>(null);
  const [pageNumber, setPageNumber] = useState<number>(1);

  function onDocumentLoadSuccess({ numPages }: { numPages: number }) {
    setNumPages(numPages);
  }

  return (
    <Box sx={wrapperStyles}>
      <Box sx={{ mb: 5, display: "flex", gap: 2 }}>
        <Button
          variant="contained"
          onClick={() => setPageNumber(pageNumber - 1)}
          disabled={pageNumber === 1}
        >
          Previous
        </Button>
        <Button
          variant="contained"
          onClick={() => setPageNumber(pageNumber + 1)}
          disabled={pageNumber === numPages || !numPages}
        >
          Next
        </Button>
      </Box>
      <Document file={url} onLoadSuccess={onDocumentLoadSuccess}>
        <Page
          renderTextLayer={false}
          pageNumber={pageNumber}
          canvasBackground="transparent"
          width={700}
          height={500}
        />
      </Document>
    </Box>
  );
};

const wrapperStyles = {
  display: "flex",
  justifyContent: "center",
  alignItems: "center",
  flexDirection: "column",
};

export default PDFViewer;
