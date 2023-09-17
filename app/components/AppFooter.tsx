import Button from "@mui/material/Button";
import Divider from "@mui/material/Divider";
import Stack from "@mui/material/Stack";
import Box from "@mui/material/Stack";
import NextLink from "next/link";

const AppFooter = () => {
  return (
    <Box sx={{ p: 5, alignItems: "center", borderTop: 1 }}>
      <Stack
        direction="row"
        spacing={1}
        divider={<Divider orientation="vertical" flexItem />}
      >
        <NextLink href="/" passHref>
          <Button>Home</Button>
        </NextLink>
        <NextLink href="/about" passHref>
          <Button>About</Button>
        </NextLink>
      </Stack>
    </Box>
  );
};

export default AppFooter;
