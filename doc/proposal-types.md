# Proposal types

## Cosmos

### Parameter Change Proposal

```json
[
  {
    "@type": "/cosmos.gov.v1beta1.MsgSubmitProposal",
    "content": {
      "@type": "/cosmos.params.v1beta1.ParameterChangeProposal",
      "title": "Adjust Chain Parameters",
      "description": "This proposal adjusts the voting period from 2 days to 4 days and the Trade & Earn vesting duration from 7 days to 14 days. This ensures that validators and community members have sufficient time to vote in proposals and make adjustments to Trade & Earn points. Additionally, it adjusts the default hourly funding rate cap from 0.0625% to 0.00625% to ultimately create a more healthy environment for traders.",
      "changes": [
        {
          "subspace": "exchange",
          "key": "DefaultHourlyFundingRateCap",
          "value": "\"0.000062500000000000\""
        },
        {
          "subspace": "exchange",
          "key": "TradingRewardsVestingDuration",
          "value": "\"1209600\""
        },
        {
          "subspace": "gov",
          "key": "votingparams",
          "value": "{\n        \"voting_period\": \"345600000000000\"\n    }"
        }
      ]
    },
    "initial_deposit": [
      {
        "denom": "inj",
        "amount": "1000000000000000000"
      }
    ],
    "proposer": "inj17vytdwqczqz72j65saukplrktd4gyfme5agf6c"
  }
]
```

Link : <https://www.mintscan.io/injective/tx/D15FA1386A0E4A955D9BD4E6821F47E78B1EED060C6236A7204ECFD9F97111F2?height=8982722&sector=json>

### Batch Community Pool Spend

Injective seems to not use this one, but has a similar type `/injective.exchange.v1beta1.BatchCommunityPoolSpendProposal`. Only difference i

```json
[
  {
    "@type": "/cosmos.gov.v1beta1.MsgSubmitProposal",
    "content": {
      "@type": "/cosmos.distribution.v1beta1.CommunityPoolSpendProposal",
      "title": "Activate governance discussions on the Discourse forum using community pool funds",
      "description": "## Summary\nProposal to request for 1000 ATOM from the community spending pool to be sent to a multisig who will put funds towards stewardship of the Discourse forum to make it an authoritative record of governance decisions as well as a vibrant space to draft and discuss proposals.\n## Details\nWe are requesting 1000 ATOM from the community spending pool to activate and steward the Cosmos Hub (Discourse) forum for the next six months.\n\nOff-chain governance conversations are currently highly fragmented, with no shared public venue for discussing proposals as they proceed through the process of being drafted and voted on. It means there is no record of discussion that voters can confidently point to for context, potentially leading to governance decisions becoming delegitimized by stakeholders.\n\nThe requested amount will be sent to a multisig comprising individuals (members listed below) who can ensure that the tokens are spent judiciously. We believe stewardship of the forum requires:\n\n* **Moderation**: Format, edit, and categorize posts; Standardize titles and tags; Monitor and approve new posts; Archive posts.\n* **Facilitation**: Ask clarifying questions in post threads; Summarize discussions; Provide historical precedence to discussions.\n* **Engagement**: Circulate important posts on other social channels to increase community participation; Solicit input from key stakeholders.\n* **Guidance**: Orient and assist newcomers; Guide proposers through governance process; Answer questions regarding the forum or Cosmos ecosystem.\nThe work to steward the forum will be carried out by members of [Hypha Worker Co-op](https://hypha.coop/) and individuals selected from the community to carry out scoped tasks in exchange for ATOM from this budget.\n## Multisig Members\n* Hypha: Mai Ishikawa Sutton (Hypha Co-op)\n* Validator: Daniel Hwang (Stakefish)\n* Cosmos Hub developer: Lauren Gallinaro (Interchain Berlin)\n\nWe feel the membership of the multisig should be rotated following the six-month pilot period to preserve insight from the distinct specializations (i.e., Cosmos Hub validators and developers).\n## Timeline and Deliverables\nWe estimate the total work to take 250-300 hours over six months where we hope to produce:\n* **Moving summaries:** Provide succinct summaries of the proposals and include all publicly stated reasons why various entities are choosing to vote for/against a given proposal. These summaries will be written objectively, not siding with any one entity.\n* **Validator platforms:** Create a section of the Forum where we collate all validators' visions for Cosmos Hub governance to allow them to state their positions publicly. We will work with the smaller validators to ensure they are equally represented.\n* **Regular check-ins with the Cosmonaut DAO:** Collaborate with the future Cosmonaut DAO to ensure maximal accessibility and engagement. Community management is a critical, complementary aspect of increasing participation in governance.\n* **Announcement channel:** Create a read-only announcement channel in the Cosmos Community Discord, so that new proposals and major discussions can be easily followed.\n* **Tooling friendly posts:** Tag and categorize posts so that they can be easily ingested into existing tooling that validators have setup.\n* **Neutral moderation framework:** Document and follow transparent standards for how the forum is moderated.\n\nAt the end of the period, we will produce a report reflecting on our successes and failures, and recommendations for how the work of maintaining a governance venue can be continuously sustained (e.g., through a DAO). We see this initiative as a process of discovery, where we are learning by doing.\n\nFor more context, you can read through the discussions on this [proposal on the Discourse forum](https://forum.cosmos.network/t/proposal-draft-activate-governance-discussions-on-the-discourse-forum-using-community-pool-funds/5833).\n\n## Governance Votes\nThe following items summarize the voting options and what it means for this proposal:\n**YES** - You approve this community spend proposal to deposit 1000 ATOM to a multisig that will spend them to improve governance discussions in the Discourse forum.\n**NO** - You disapprove of this community spend proposal in its current form (please indicate why in the Cosmos Forum).\n**NO WITH VETO** - You are strongly opposed to this change and will exit the network if passed.\n**ABSTAIN** - You are impartial to the outcome of the proposal.\n## Recipient\ncosmos1xf2qwf6g6xvuttpf37xwrgp08qq984244952ze\n## Amount\n1000 ATOM\n\n***Disclosure**: Hypha has an existing contract with the Interchain Foundation focused on the testnet program and improving documentation. This work is beyond the scope of that contract and is focused on engaging the community in governance.*\n\nIPFS pin of proposal on-forum: (https://ipfs.io/ipfs/Qmaq7ftqWccgYCo8U1KZfEnjvjUDzSEGpMxcRy61u8gf2Y)",
      "recipient": "cosmos1xf2qwf6g6xvuttpf37xwrgp08qq984244952ze",
      "amount": [
        {
          "denom": "uatom",
          "amount": "1000000000"
        }
      ]
    },
    "initial_deposit": [
      {
        "denom": "uatom",
        "amount": "1000000"
      }
    ],
    "proposer": "cosmos1cc5mtuje84sx5dsqw4gtr4w8lthpzw02rjjnew"
  }
]
```

Link : <https://www.mintscan.io/cosmos/tx/99CA985295052F6EB41DC61D44B56AAA8951A8BACBFA760E6D1BBA6D8043BA52?height=9762753&sector=json>

### Text

```json
[
  {
    "@type": "/cosmos.gov.v1beta1.MsgSubmitProposal",
    "content": {
      "@type": "/cosmos.gov.v1beta1.TextProposal",
      "title": "Deploy inj-interchain-persona Contract on Injective Mainnet",
      "description": "Summary:\nThis proposal seeks approval for the deployment of the inj-interchain-persona contract on the Injective mainnet. The contract allows users to securely manage and link their wallets across different blockchain networks. \n\nContract Features:\nCross-Chain Persona Management: Allows users to link multiple wallets from different blockchains, specifically Injective and MultiversX for now, under a single persona.\n\nBenefits to the Injective Ecosystem:\n1.Interoperability: The contract enhances cross-chain functionality, allowing users to manage their identities across Injective and MultiversX, increasing interconnectivity between blockchains.\n\n2.User Security: By linking wallets under a single persona, the contract strengthens the security and usability of identity management on the Injective network.\n\n3.Growth of Cross-Chain Services: As cross-chain interactions become more prevalent, this contract positions Injective as a leading hub for decentralized identity solutions, attracting new users and projects.\n\n4.Community Contribution: This open-source project will encourage continuous development and community contributions, ensuring long-term innovation and sustainability.\n\nTechnical Details:\nContract Name: inj-interchain-persona\n\nContract Version: 0.0.1\n\nCompiler Version: cosmwasm/rust-optimizer-arm64:0.16.0\n\nChecksum: 6eb0b885b220413798c50fa4e6f1f5822912f67ab9a517fa1dac1d11a011a398\n\nContract Documentation: https://github.com/Helios-Collabathon/Injective-smart-contracts"
    },
    "initial_deposit": [
      {
        "denom": "inj",
        "amount": "100000000000000000000"
      }
    ],
    "proposer": "inj1cxe65zdyly8p89n822zwd0gmquymndchhldq79"
  }
]
```

Link : <https://www.mintscan.io/injective/tx/45E6BC913C9F59E820A717D9C7F98DB8BC96D43D3CAE7E74E9E5D0AD3079FA5E?height=85192035&sector=json>

### IBC Client Update

```json
[
  {
    "@type": "/cosmos.gov.v1beta1.MsgSubmitProposal",
    "content": {
      "@type": "/ibc.core.client.v1.ClientUpdateProposal",
      "title": "Upgrade expired IBC client",
      "description": "This proposal, if it passes, will update expired IBC client towards Nois. This will re-enable IBC transactions between these two chains once clients on both sides have been updated via governance. Expired IBC client 07-tendermint-201 will be refreshed with active client 07-tendermint-231",
      "subject_client_id": "07-tendermint-201",
      "substitute_client_id": "07-tendermint-231"
    },
    "initial_deposit": [
      {
        "denom": "inj",
        "amount": "50000000000000000000"
      }
    ],
    "proposer": "inj1fqrdtx7pyps6eytn3356j9cs4f8zl0eeme75z0"
  }
]
```

Link : <https://www.mintscan.io/injective/tx/4383388EE1E5F74FFABF9C3779CACBD4C1FB842216D06D779283D79D54E23DB8?height=51589432&sector=json>

### Software Upgrade

#### Old format

```json
[
  {
    "@type": "/cosmos.gov.v1beta1.MsgSubmitProposal",
    "content": {
      "@type": "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal",
      "title": "Injective Protocol Canonical Chain Upgrade",
      "description": "This is a software upgrade proposal for the upgrade to the canonical Injective Chain. If passed, this proposal would commit the Injective Chain to halting the Injective Canary Chain application binary at approximately 14:00 UTC on Nov 8th and starting the application binary for the Injective Canonical Chain. \\n\\nMore details can be found in the long form proposal: https://github.com/InjectiveLabs/injective-chain-releases/blob/master/docs/migration/injective-canonical-chain.md",
      "plan": {
        "name": "v1.1",
        "time": "0001-01-01T00:00:00Z",
        "height": "4352000",
        "info": "",
        "upgraded_client_state": null
      }
    },
    "initial_deposit": [
      {
        "denom": "inj",
        "amount": "500000000000000000000"
      }
    ],
    "proposer": "inj1sfem88pzh8pjs4lwm7lfuf9agqnktuvz6vksfa"
  }
]
```

Link : <https://www.mintscan.io/injective/tx/34F5529B341631BACFEF34BEEC63F858DCBEC51EEBEE25F70B17CE51144E45AC?height=4275388&sector=json>

#### New format

```json
[
  {
    "@type": "/cosmos.gov.v1beta1.MsgSubmitProposal",
    "content": {
      "@type": "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal",
      "title": "Injective Avalon Upgrade (v1.11)",
      "description": "More details on breaking changes and features can be found in official release: https://github.com/InjectiveLabs/injective-chain-releases/blob/master/docs/releases/v1-11.md",
      "plan": {
        "name": "v1.11",
        "time": "0001-01-01T00:00:00Z",
        "height": "34775000",
        "info": "",
        "upgraded_client_state": null
      }
    },
    "initial_deposit": [
      {
        "denom": "inj",
        "amount": "50000000000000000000"
      }
    ],
    "proposer": "inj17vytdwqczqz72j65saukplrktd4gyfme5agf6c"
  }
]
```

Link : <https://www.mintscan.io/injective/tx/1F05CDF18E51C7C44EEBFFAC9F6F965BA8D84CFAF934194E5BC0474C6DF681AD?height=34417810&sector=json>

