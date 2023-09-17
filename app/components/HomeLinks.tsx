import { Grid } from "@mui/material";

const links = [
  {
    title: "About",
    link: "/about",
    description:
      "Explore detailed information about the project, its mission, and vision.",
  },
  {
    title: "Team",
    link: "/team",
    description:
      "Meet the creative minds and dedicated professionals behind the project.",
  },
  {
    title: "Feedback",
    link: "/feedback",
    description:
      "Share your thoughts and insights to help us improve and grow.",
  },
  {
    title: "NFTs",
    link: "/nfts",
    description:
      "Discover our unique collection of NFTs and learn how to get involved.",
  },
];

export default function HomeLinks() {
  return (
    <Grid
      container
      spacing={3}
      justifyContent="center"
      alignItems="stretch"
      sx={{ mt: 10, mb: 10 }}
    >
      {links.map((link) => (
        <Grid item md={3} sm={6} key={link.title}>
          <a href={link.link}>
            <h2>
              {link.title} <span>-&gt;</span>
            </h2>
            <p>{link.description}</p>
          </a>
        </Grid>
      ))}
    </Grid>
  );
}
