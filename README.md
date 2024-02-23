# cleanskyinitiative


Project Name: CleanSky Initiative

About Us:

Name: Ibrahim Halil Doğan

Project Details:
CleanSky Initiative is a sustainability platform designed to operate on the Internet Computer. This platform is intended to raise awareness about eco-friendly lifestyles and encourage collaboration for a sustainable world.

Development Plan:
Design of Smart Contract Functions and Variables: Basic smart contract functions and variables such as user profiles, article writing functions, and data access will be designed.


Project Name: CleanSky Initiative

About Us:

Name: Ibrahim Halil Doğan

Project Details:
CleanSky Initiative is a sustainability platform designed to operate on the Internet Computer. This platform is intended to raise awareness about eco-friendly lifestyles and encourage collaboration for a sustainable world.

Vision:
CleanSky Initiative aims to inform people about sustainability using collected data and to bring them together to act together. Our goal is to create a platform where users can share their ideas, write articles, and communicate with each other, taking a step towards a sustainable world.

Development Plan:
Design of Smart Contract Functions and Variables: Basic smart contract functions and variables such as user profiles, article writing functions, and data access will be designed.

Smart Contract Development: The designed smart contracts will be coded in the Rust language and tested using the Internet Computer Protocol.

User Interface Development: A web-based user interface will be created. Users will be able to create accounts, write articles, and interact with other users.

Integration of Interface and Smart Contracts: The user interface will be integrated with smart contracts to enable users to perform transactions on the platform.

Beta Testing and Feedback: The platform will undergo beta testing, and user feedback will be collected.

Deployment: After completing the tests, the platform will be deployed to the main network and made available for use.

Personal Story Summary:
My goal is to create a sustainability platform on the Internet Computer that brings people together to exchange ideas about sustainability. By raising awareness and inspiring action, I aim to collectively create a more sustainable world.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
