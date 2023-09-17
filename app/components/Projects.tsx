import {
  Grid,
  Box,
  Card,
  CardActionArea,
  CardContent,
  CardMedia,
  Typography,
} from "@mui/material";
import { useState } from "react";

import comics from "../assets/data/comics.json";

import ComicModal from "./ComicModal";

function excerpt(description) {
  const maxLength = 100;
  return description.length > maxLength
    ? description.substring(0, maxLength) + "..."
    : description;
}

export default function Projects() {
  const [isComicModalOpen, setIsComicModalOpen] = useState(false);
  const [selectedComic, setSelectedComic] = useState(comics[0]);

  const handleOpenComicModal = (comic) => {
    console.log({ comic });
    
    setSelectedComic(comic);
    setIsComicModalOpen(true);
  };
  const handleCloseComicModal = () => {
    setIsComicModalOpen(false);
  };
  return (
    <Box>
      <Grid container spacing={3} justifyContent="center" alignItems="stretch">
        {comics.map((c, indx) => (
          <Grid item key={indx} xs={12} md={4}>
            <Card onClick={() => handleOpenComicModal(c)}>
              <CardActionArea>
                <CardMedia
                  component="img"
                  height="500"
                  src={`/imgs/${c.img}`}
                  alt={c.title}
                />
                <CardContent>
                  <Typography
                    gutterBottom
                    variant="h5"
                    component="div"
                    sx={{ fontFamily: "inherit" }}
                  >
                    {c.title}
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    {excerpt(c.description)}
                  </Typography>
                </CardContent>
              </CardActionArea>
            </Card>
          </Grid>
        ))}
 <ComicModal
          open={isComicModalOpen}
          data={selectedComic}
          onClose={handleCloseComicModal}
        />
      </Grid>
    </Box>
  );
}
