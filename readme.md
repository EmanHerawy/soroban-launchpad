# Soropad [![Ceasefire Now](https://badge.techforpalestine.org/ceasefire-now)](https://techforpalestine.org/learn-more)
### **"Sorapad: Your easy way to a world of tokenization on Stellar network."**

![Alt text](image.png)
## Inspiration
The inspiration behind Soropad stemmed from the desire to empower artists, content creators, and innovators in the blockchain space. We wanted to provide a user-friendly platform that enables the seamless launch of various types of tokens, including fungible tokens, non-fungible tokens (NFTs), and semi-fungible tokens. If we can make it easier for creators to bring their ideas to life on the blockchain, we can accelerate the adoption of blockchain technology and its transformative potential. Through INO " Initial NFT Offering", we can help artists monetize their work by selecting the best payment token which couldb be XLM, USDC, or any other token on the Soroban Stellar network they create,  and connect with their fans. We believe that Soropad can be a gateway to NFT stardom on Soroban Stellar.
## What it does
Soropad is a platform that enables the seamless launch of various types of tokens, including fungible tokens, non-fungible tokens (NFTs), and semi-fungible tokens. It also enables the launch of INO " Initial NFT Offering" which allows artists to monetize their work by selecting the best payment token which could be XLM, USDC, or any other token on the Soroban Stellar network they create, and connect with their fans.
## Main Complements :
1. ### **Smart contracts**
- **tokenlanchpad** : this contract provides an easy way to create and launch fungible tokens, similar to ERC20 tokens in the Ethereum ecosystem. User can create a token by selecting a name, symbol, and decimal. The token is launched on the Soroban Stellar network with a single click. 
- **nft-launchpad** : this contract provides an easy way to create and launch non-fungible tokens, similar to ERC721 tokens in the Ethereum ecosystem. User can create a token by selecting a name, symbol, and uri. The token is launched on the Soroban Stellar network with a single click. Only author caan mint the token. this would be helpful if the end user want to mint the tokens and then sell them on any markeplace or airdrop them to their fans.
- **soro-ino** : INO stands for Initial NFT Offering. This contract provides an easy way to create and launch non-fungible tokens, similar to ERC721 tokens in the Ethereum ecosystem, and sell them directly to the public. User can create a token by selecting a name, symbol, uri, sale price, sale time, pre sale price and payment token. The token is launched on the Soroban Stellar network with a single click.  author can mint the tokens and airdrop them to their fans. Or und-user can mint them any pay the minting fees. The author can set the price of the token in XLM, USDC, or any other token on the Soroban Stellar network they create. 
#### Sub Complements :
- **token** : fungible token representing stellar assets. 
- **nft_721** : NFT token equivalent to ERC721 tokens in the Ethereum ecosystem. I notice that this implementation is not yet developed in the `soroban-examples` repository. So I decided to implement it.
- **nft_1155** : NFT token equivalent to ERC1155 tokens in the Ethereum ecosystem. I notice that this implementation is not yet developed in the `soroban-examples` repository. So I decided to implement it.

2. ### **Front-end**
  - Artists/creators 
    -  connect his/her wallet to `freighter`
    -  launch their fungible token
    -  launch their nonfungible token
  -  fans
     -  connect his/her wallet to `freighter`
     -  mint nft

## Technology used
- Rust
- soroban sdk
- react 
- soroban-client
- @stellar/freighter-api

## What's next for SoroPad
- smart contracts:
  - Audit the current contracts
  - build marketplaces
- Front-end:
  - build a better UI
  - complete the smart contract integration
  - add more features
## Challenges we ran into
1. The updates to the Soroban Stellar network and the Soroban SDK were released during the hackathon. I couldn't sship my contract to testnet on the planned time due to techncial issues I had after the network updates . many hours were spent to figure out what's going wrong and many other hours to fix it. 
Unfortunately this lead to a significant delay in the development of the front-end. I couldn't apply all the planned functionalities/features to the front-end.
2. String conversion and concatenation limitation in soroban sdk. I spent many hours to figure out how to convert a string to a i128 and vice versa. I also spent many hours to figure out how to concatenate two strings.

## How to build:
Navigate to the contract directory and run the following command :
- ```cargo build --release --target wasm32-unknown-unknown``` 

or

- ```soroban contract build```


## How to test :
Navigate to the contract directory and run the following command :
```cargo test ```


## Generate bindings
Navigate to the contract directory and run the following command :
```
soroban contract bindings typescript --wasm [path to wasm file] --output-dir [output dir]--contract-id [contract id] --contract-name [contract name]

```

## setup local test node  reference: [https://soroban.stellar.org/docs/reference/rpc#standalone]

``` 
docker run --rm -it \
  -p 8000:8000 \
  --name stellar \
  stellar/quickstart:soroban-dev@sha256:a6b03cf6b0433c99f2f799b719f0faadbb79684b1b763e7674ba749fb0f648ee \
  --standalone \
  --enable-soroban-rpc

 curl "http://localhost:8000"

 soroban config network add standalone \
  --rpc-url "http://localhost:8000/soroban/rpc" \
  --network-passphrase "Standalone Network ; February 2017"

 soroban config identity generate alice

 curl "http://localhost:8000/friendbot?addr=$(soroban config identity address alice)"


 soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/[project_name].wasm \
    --source alice \
    --network standalone


   soroban contract invoke \
    --id C… \
    --source alice \
    --network standalone \
    -- \
    hello \
    --to friend```
