# Subgraph Cosmos Software Upgrade

This document describes the software upgrade process for the Cosmos Hub. The software upgrade process is a critical part of the Cosmos Hub governance process. It allows the network to evolve by upgrading the software that powers the network. The software upgrade process is a multi-step process that involves the coordination of validators, delegators, and other network participants.

## Proposals explorers

- **Osmosis**
  - <https://www.mintscan.io/osmosis/proposals>
  - <https://govscan.live/osmosis/proposals>
- **Cosmos Hub**
  - <https://www.mintscan.io/cosmos/proposals>
  - <https://govscan.live/cosmoshub/proposals>
- **Injective**
  - <https://govscan.live/injective/proposals>

## Concepts

<https://docs.cosmos.network/v0.46/modules/gov/01_concepts.html>

The governance process is divided in a few steps that are outlined below:

- **Proposal submission**: Proposal is submitted to the blockchain with a deposit.
- **Vote**: Once deposit reaches a certain value (MinDeposit), proposal is confirmed and vote opens. Bonded Atom holders can then send TxGovVote transactions to vote on the proposal.
- **Execution:** After a period of time, the votes are tallied and depending on the result, the messages in the proposal will be executed.
