# Subgraph Limitations: Governance Parameters

## Overview
This document outlines the capabilities and limitations of tracking governance parameters within the Cosmos-based blockchain Governance Proposals subgraph. Understanding these limitations is crucial for developers integrating this subgraph into their applications.

## Governance Parameter Tracking

### Initial Parameter Configuration
The subgraph tracks governance parameters by starting with genesis parameters specific to each chain (e.g., Cosmos Hub, Injective). These parameters must be formatted as a JSON string and provided to the subgraph's configuration.

#### Example Configuration (Injective)

The following json string is passed in the [substreams.yaml file for the Injective subgraph](https://github.com/pinax-network/subgraph-cosmos-proposals/blob/main/subgraphs/injective/substreams.yaml) : 

```json
{
    "deposit_params": {
        "min_deposit": [
            {
                "denom": "inj",
                "amount": "500000000000000000000"
            }
        ],
        "expedited_min_deposit": [
            {
                "denom": "uatom",
                "amount": "50000000"
            }
        ],
        "max_deposit_period": "172800000000000"
    },
    "voting_params": {
        "voting_period": "172800000000000",
        "expedited_voting_period": "86400000000000"
    },
    "tally_params": {
        "quorum": "0.334000000000000000",
        "expedited_quorum": "0.5",
        "threshold": "0.500000000000000000",
        "expedited_threshold": "0.667000000000000000",
        "veto_threshold": "0.334000000000000000"
    }
}
```

Note that **expedited** parameters were not included in the chain's original genesis file but were introduced later to support the handling of expedited proposals. These parameters were retroactively added to the "genesis" JSON string to ensure compatibility. 

It is important to note that the `uatom` denomination is used for the `expedited_min_deposit`. This denomination is specific to Cosmos Hub and not native to Injective. This suggests that Injective reused Cosmos Hub's expedited parameters during implementation. For more details, refer to [Injective's documentation](https://docs.injective.network/developers/modules/core/gov#parameters).

### Chain Parameter Updates: Capabilities and Limitations

Chain parameters can be modified through two different mechanisms on-chain, but the subgraph only tracks one of these:

1. ✅ **Parameter Change Proposals** (Tracked)

   - Automatically tracked by the subgraph
   - Creates new `GovernanceParameter` entities upon successful proposals
   - Maintains accurate linkage between current parameters and active proposals
   - Enables correct calculation of:
     - Quorum requirements
     - Threshold values
     - Veto thresholds
     - Minimum deposit amounts
     - Deposit and voting periods
     - Time-based constraints (deposit_end_time, voting_end_time)

2. ❌ **Software Upgrade Proposals** (Not Tracked)
   - Can modify chain parameters as part of the upgrade
   - The subgraph cannot detect or track these changes
   - Creates potential discrepancies between subgraph and on-chain states
   - Example: [Injective's Proposal #314](https://www.mintscan.io/injective/proposals/314) changed the `min_deposit` from 500 INJ to 100 INJ, but this change was not reflected in the subgraph

## Important Considerations

### Parameter Tracking Limitations
The subgraph's inability to track parameter changes implemented through software upgrades can lead to discrepancies between:
- The subgraph's represented state of governance parameters
- The actual on-chain parameter values

### Current Known Issues
- For Injective:
  - Discrepancy exists in the `min_deposit` parameter
  - Other parameters remain accurately tracked
- Similar discrepancies may exist for other Cosmos-based chains

### Usage Recommendations
Given these limitations, developers should:
- Exercise caution when relying on the subgraph's parameter values for critical operations
- Implement additional validation for sensitive operations
- Consider cross-referencing parameters with direct chain queries when absolute accuracy is required

## Future Improvements
We are actively working on enhancing the subgraph's parameter tracking capabilities. Future updates may include:
- Support for detecting parameter changes through software upgrades
- Improved parameter validation mechanisms
- Additional cross-referencing capabilities with on-chain data

For the most up-to-date implementation details, please refer to the [subgraph configuration in the repository](https://github.com/pinax-network/subgraph-cosmos-proposals/blob/main/subgraphs/injective/substreams.yaml).