# Subgraph: `Cosmos Governance Proposals`

> Tracks governance proposals on Cosmos based networks.

## Data Includes

- `Votes`
- `Deposits`
- `GovernanceParameter`
- `Proposals`
  - `SoftwareUpgrade`
  - `ParameterChange`
  - `CommunityPoolSpend`
  - `ClientUpdate`

## Chains

- **API Key**: <https://thegraph.com/studio/apikeys/>
- **Base URL**: <https://gateway.thegraph.com/api>
- **Query URL format**: `{base_url}`/api/`{api-key}`/subgraphs/id/`{subgraph_id}`

| Chain | Subgraph ID |
| ----- | ----------- |
| Injective   | [`SUBGRAPH_ID`](https://thegraph.com/explorer/subgraphs/SUBGRAPH_ID?view=Query&chain=arbitrum-one) |
| CosmosHub   | [`SUBGRAPH_ID`](https://thegraph.com/explorer/subgraphs/SUBGRAPH_ID?view=Query&chain=arbitrum-one) |
| Osmosis   | [`SUBGRAPH_ID`](https://thegraph.com/explorer/subgraphs/SUBGRAPH_ID?view=Query&chain=arbitrum-one) |

## Proposals explorers

- **Osmosis**
  - <https://www.mintscan.io/osmosis/proposals>
  - <https://govscan.live/osmosis/proposals>
- **Cosmos Hub**
  - <https://www.mintscan.io/cosmos/proposals>
  - <https://govscan.live/cosmoshub/proposals>
- **Injective**
  - <https://www.mintscan.io/injective/proposals>
  - <https://govscan.live/injective/proposals>

## Concepts

<https://docs.cosmos.network/v0.46/modules/gov/>

The governance process is divided in a few steps that are outlined below:

- **Proposal submission**: Proposal is submitted to the blockchain with a deposit.
- **Vote**: Once deposit reaches a certain value (MinDeposit), proposal is confirmed and vote opens. Bonded Atom holders can then send TxGovVote transactions to vote on the proposal.
- **Execution:** After a period of time, the votes are tallied and depending on the result, the messages in the proposal will be executed.

## GraphQL

**SoftwareUpgradeProposals**

```graphql
query SoftwareUpgradeProposals{
  softwareUpgradeProposals(orderBy:planHeight) {
    id
    planName
    planInfo
    planHeight
    proposal{
      id
      proposer
      transaction {
        id
      }
    }
  }
}
```

**Proposals**

```
query Proposals{
  proposals(orderBy:block__number) {
    id
    block{
      number
      date
    }
    proposer
    title
    summary,
  }
}
```

**Deposits grouped by Proposals**

```graphql
query DepositsGroupedByProposals {
  proposals(orderBy:block__number) {
    id
    title
    deposits{
      depositor
      denom
      amount
      block{
        number
        date
      }
    }
  }
}
```
