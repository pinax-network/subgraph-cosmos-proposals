# Calculating proposal status

## Total Voting Power

Total voting power represents the aggregate amount of staked ATOM tokens that are eligible to participate in governance decisions. It's important to note that:

- Only bonded (staked) ATOM tokens contribute to voting power[1].
- Liquid (unstaked) ATOM tokens are not part of the total voting power and cannot participate in voting[1].

### Calculating Total Voting Power

To calculate the total voting power, you would need to:

1. Determine the end of the voting period (typically 14 days after the proposal enters the voting phase)[1].
2. Sum up all staked ATOM tokens across all validators and delegators at that specific point in time.

It's worth noting that:

- Validators not in the active set (jailed, tombstoned, or outside the top validators) and their delegators' ATOM do not count towards the total voting power at the end of the voting period[1].
- The total voting power can fluctuate during the voting period as tokens are staked or unstaked.

### Importance in Governance

Understanding total voting power is crucial because:

- It determines the quorum (minimum participation) required for a valid vote, which is set at 40% of the total voting power[1].
- It's used to calculate the thresholds for proposal passage (>50% of participating voting power) and veto (<33.4% of participating voting power)[1].

### Practical Considerations

For an individual wanting to calculate the total voting power:

1. You would need access to blockchain data at the block height corresponding to the end of the voting period.
2. Sum the staked ATOM balances of all active validators and their delegators.
3. Exclude any staked ATOM from validators outside the active set.

In practice, this calculation is typically performed automatically by the blockchain software and governance interfaces, as it requires a comprehensive view of the network's state at a specific point in time.

Citations:
[1] https://hub.cosmos.network/main/governance/process

[2] https://github.com/cosmos/cosmos-sdk/blob/main/x/gov/README.md

[3] https://github.com/cosmos/cosmos/blob/master/GOVERNANCE_DOC.md

[4] https://interchaininfo.zone/resources/a-beginners-guide-to-cosmos-governance

[5] https://docs.exocore.network/whitepaper/restaking-mechanism/voting-power-calculation
