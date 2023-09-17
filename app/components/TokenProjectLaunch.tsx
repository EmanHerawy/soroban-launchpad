import {
  Grid,
  Box,
    Card,
    TextField,
    CardContent,
  Button,
  CardActions,
  Typography
} from "@mui/material";
import { useState } from "react";

 
 
// function excerpt(description) {
//   const maxLength = 100;
//   return description.length > maxLength
//     ? description.substring(0, maxLength) + "..."
//     : description;
// }

export default function TokenProjectLaunch() {
    const [isConnected, setIsConnected] = useState(true);
    const handleMinting = () => {
     
 }

    return (
        <Box>
            <Card>
         
                <CardContent>
                    <Typography
                        gutterBottom
                        variant="h5"
                        component="div"
                        sx={{ fontFamily: "inherit", textAlign: "center" }}
                    >
                        Launch Your Token
                    </Typography>
               
                    <Typography variant="body2" color="text.secondary"  sx={{   textAlign: "center" }}>
                        Launch your Token with no code with SoroPad
                    </Typography>


                             <Box component="form" sx={{'& > :not(style)': { m: 1, width: '25ch' },}} noValidate autoComplete="on" >
      
                                <TextField id="standard-basic" label="Name" variant="standard" />
                                <TextField id="standard-basic" label="Symbol" variant="standard" />
                                <TextField id="standard-basic" label="Decimal" variant="standard" type="number" />
                            </Box>
                </CardContent>
       <CardActions sx={{ justifyContent: "flex-end" }}>
              {isConnected ? (
                <Button
                  onClick={handleMinting}
                  variant="contained"
                  size="large"
                >
                  Create new project
                </Button>
              ) : (
                            // <ConnectWallet />
                            <></>
              )}
            </CardActions>
           
            </Card>
        </Box>);


}