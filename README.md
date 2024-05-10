![image](https://github.com/BlockImpulse/Stellar-STOCKen-CAPITAL/assets/81595884/d374b07e-0eca-4e5c-a636-7fc85c9f3753)

# Stocken Capital Stellar Smart Contracts

The project offer an innovative solution for secure and transparent financial transactions. Leveraging the power of the Stellar blockchain, Stocken Capital introduces a robust system of smart contracts designed to revolutionize how agreements are managed and transactions are executed.

The primary focus of the Stocker Capital Smart Contracts project is to streamline the creation and execution of agreements. The project includes the creation of an oracle contract that will be utilized by nodes/listeners to provide trusted responses, enhancing the reliability and accuracy of the transaction process. Additionally, the project includes the development of smart contracts that consume and interact with the oracle contract to facilitate secure and transparent transactions.

## Contracts

### Escrow

The Escrow contract initiates the process by allowing users to create proposals outlining what they can offer and what they require to enhance their projects. These proposals are registered with the Escrow contract, awaiting selection by interested parties. Once selected, an organization or user can choose a proposal through the platform, triggering the creation of a Signaturit document with predefined conditions ready for signature. Subsequently, the Escrow contract records the transaction and registers it with the Oracle, enabling each Escrow process to be identified and monitored. If the involved parties agree to and sign the document, the Oracle triggers a callback, releasing funds to the respective party and generating a NFT as proof of the transaction.

### Signaturit Oracle

The Oracle acts as the intermediary between the blockchain and the external world, specifically interfacing with the Signaturit platform in this context. It registers a Signaturit process, making it available for monitoring. When the status of the document changes, the Oracle is notified, triggering a callback to the contract that initiated the registration. Additionally, we provide an OracleImplementer interface (trait), detailing the functions the Oracle will perform and the expected callbacks. This allows implementers to customize their processes based on the Oracle's responses.

### Non Fungible Token (NFT)

Upon acceptance of the agreement by the involved parties, a unique NFT is generated containing the hash of the signed document. This serves as immutable proof that both parties have agreed to the process. Furthermore, the document hash allows retrieval of the actual document via its token URI, enabling stakeholders to access and verify the contents. Initialization includes setting the base URI, allowing customization to point to an endpoint of choice.

## Why Stellar?

The [Stellar](https://stellar.org/) network is an open-source blockchain used for a variety of payment and remittance applications. Stellar empowers builders to unlock human and economic potential. It combines a powerful, decentralized blockchain network with a global ecosystem of innovators to create opportunities as borderless as ideas. It offers the tools to make a difference in the real world through new digital asset products and services that enhance access to the global financial system.

## Project

To execute this project, your setup will need some tools:

### Setup Tools

#### Install rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Install the `wasm32-unknown-unknown` target

```shell
rustup target add wasm32-unknown-unknown
```

#### Install Soroban CLI

```shell
cargo install soroban-cli --features opt --locked
```

Stellar provide different toolings to create and interact with Smart Contracts (see their [documentation](https://developers.stellar.org/docs/smart-contracts)).

### Execute test

To execute the tests of the project, you need to run this command from project's root:

```shell
make -C contracts test
```

This will make the build and execute the test for each contract

### Special Thanks

The Stellar community has been invaluable; their support on various channels has been truly awesome. We extend our gratitude to all those who have helped and responded to our inquiries. In particular, we wish to express our deep appreciation to [@esteblock](https://github.com/esteblock) for his invaluable guidance, corrections, and assistance in connecting us with the right people.

Additionally, we extend special thanks to [@ajoseerazo](https://github.com/ajoseerazo) for aiding us in integrating the Stellar Ecosystem Proposal #30 (SEP30) for non-custodial wallets. His guidance and contributions regarding the utilization of the [django-polaris](https://django-polaris.readthedocs.io/en/stable/) package have been key in successfully implementing the various processes required for SEP30 integration.

## Contributions

Contributions to the project are highly welcomed! If there are any suggestions, fixes, or bug reports, feel free to submit a [pull request](https://github.com/BlockImpulse/Stellar-STOCKen-CAPITAL/pulls) or open an [issue](https://github.com/BlockImpulse/Stellar-STOCKen-CAPITAL/issues) on GitHub.
