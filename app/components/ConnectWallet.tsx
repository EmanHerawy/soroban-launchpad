"use client";

import { Button } from "@mui/material";
import {
  isConnected,
  setAllowed,
  isAllowed,
  getPublicKey,
  signTransaction,
  signBlob,
  getUserInfo,
} from "@stellar/freighter-api";
import { useState, useEffect } from "react";
 let address: string;
  let _isConnected:boolean;
export default function ConnectWallet() {
 

  const [userIsAllowed,setUserIsAllowed ] = useState(address === undefined);
  const [connected, setConnected] = useState(address === undefined);
  const [loading, setLoading] = useState(address === undefined);
   const label = _isConnected ? "Disconnect" : "Connect Wallet";


   const addressLookup = (async () => {
  if (await isConnected()) return await getUserInfo()
   })();
   const checkStates = (async () => {
     if (await isConnected()) {
      
        _isConnected = true;
        address = await getPublicKey();
        console.log({ address });
       setConnected(true);
       setLoading(false);
       if (await isAllowed()) {
         setUserIsAllowed(true);
       } else {
          setUserIsAllowed(false);
       }
  }
   })();
  // returning the same object identity every time avoids unnecessary re-renders
const addressObject = {
  address: '',
  displayName: '',
};

const addressToHistoricObject = (address: string) => {
  addressObject.address = address;
  addressObject.displayName = address?`${address.slice(0, 4)}...${address.slice(-4)}`:address;
  return addressObject
};
  useEffect(() => {
    console.log({address});
    checkStates
    if (address !== undefined) return;

    addressLookup
      .then(user => { if (user) address = user.publicKey })
      .finally(() => { setLoading(false) });
  }, []);
  return (
        <>
      {userIsAllowed ? (
        <div >
          <div >{addressToHistoricObject(address).displayName}</div>
        </div>
      ) : (
    <Button
      onClick={setAllowed}
      disabled={loading}
      variant="contained"
      size="large"
    >
      {loading ? "Loading..." : label}
    </Button>      )}
      
    </>

  );
}
