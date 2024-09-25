# Proposal types

## Cosmos

### Parameter Change Proposal

#### ParameterChangeProposal

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

#### MsgUpdateParams

This one might be absurd to implement. As they don't use an array for parameters, we'd have to make a Protobuf for every parameter update possible. Luckily this message seems to be very rare, could only find one in the Cosmos proposals list.

```json
[
  {
    "@type": "/cosmos.gov.v1.MsgSubmitProposal",
    "messages": [
      {
        "@type": "/cosmwasm.wasm.v1.MsgUpdateParams",
        "authority": "cosmos10d07y265gmmuvt4z0w9aw880jnsr700j6zn9kn",
        "params": {
          "code_upload_access": {
            "permission": "AnyOfAddresses",
            "addresses": [
              "cosmos1559zgk3mxm00qtr0zu2x5n4rh5vw704qaqj6ap"
            ]
          },
          "instantiate_default_permission": "Everybody"
        }
      }
    ],
    "initial_deposit": [
      {
        "denom": "uatom",
        "amount": "25000000"
      }
    ],
    "proposer": "cosmos1559zgk3mxm00qtr0zu2x5n4rh5vw704qaqj6ap",
    "metadata": "Authorize DAO DAO Deployment",
    "title": "Authorize DAO DAO Deployment",
    "summary": "## Introduction\n\nWith CosmWasm recently live on the Cosmos Hub, we find ourselves at a pivotal moment in the Hub’s journey: the first ever contract upload authorization proposal. As Carter Woetzel of the AADAO Strategy Committee noted, this proposal “...is a testament to the evolving capabilities of the Cosmos Hub.”\n\nThis moment represents more than just a software upgrade: it signifies the Hub’s commitment to building state-of-the-art governance systems and advancing decentralized organization and decision-making together.\n\nIn order to launch smart contracts on a permissioned CosmWasm chain, like the Hub, governance must vote to authorize the deployer. This proposal authorizes our wallet (cosmos1559zgk3mxm00qtr0zu2x5n4rh5vw704qaqj6ap) to deploy contracts which can be instantiated by anyone. If this proposal passes, we will swiftly deploy the latest audited version of the contracts, and Cosmos Hub DAOs will be live on daodao.zone.\n\n## About DAO DAO\n\nWith DAO DAO, Cosmos projects and communities have immediate access to best-in-class DAO tooling without needing to build their own governance modules, smart contracts, UI, and more. Try it today: [https://daodao.zone](https://daodao.zone/)\n\nOur mission is to build better, more flexible governance systems. Getting governance right is crucial to the success of decentralized communities, protocols, and entire blockchains, and DAO DAO makes it as easy as clicking a few buttons to experiment with and scale governance infrastructure.\n\nToday, over $300M is managed by 5,248 DAO DAO DAOs across 11 chains (and growing). Our smart contracts and UI have been battle tested by 13,047 proposals encompassing 33,232 unique DAO members for over 2 years. Over $1M (and many, many hours) have been put into the necessary research and development, as well as the perfection of each feature, with a solid chunk devoted to numerous security audits.\n\nAmong other use cases, DAO DAO is a critical governance component in the upcoming Hydro platform. This is one of the key reasons why the Atom Accelerator DAO is supporting this deployment of DAO DAO.\n\nThe biggest features we aim to bring to the Cosmos Hub:\n\n- **Token** DAOs and **Membership** (multisig-like) DAOs\n- **Beautiful UI** for making proposals, voting, and exploring DAOs\n- **Cross-Chain Accounts** via Polytone\n- **Vesting** and **Retroactive Payment** solutions\n- Sophisticated organizational structures using **SubDAOs** and **Authz**\n\nAnd much more:\n\n- Treasury spends, swaps, and historical graphs\n- Staking management\n- Veto and approval flows\n- NFT creation and management\n- On-chain, governance gated key / value store\n- Proposal inbox\n- Customizable DAO widgets\n- DAO Press: publishing tools for a DAO-run blog, official communications, and more\n- Bulk actions import\n- Smart contract management (instantiate, execute, migrate, update admin)\n- DAO-run validators (check out ours here: https://daodao.zone/dao/juno185hgkqs8q8ysnc8cvkgd8j2knnq2m0ah6ae73gntv9ampgwpmrxqc5vwdr)\n- Push and email notifications\n- Mobile-friendly UI\n- Member and proposal vote CSV downloads\n- Chain governance proposal UI\n- Embedded Apps UI\n- etc.\n\nOur immediate roadmap includes:\n\n- Staking rewards\n- Voting delegations\n- Treasury rebalancing management\n- Voting with locked vesting tokens\n- Built-in forums\n- …and more\n\n## Polytone Interchain Accounts and Queries\n\nIn addition to DAO DAO contracts, the Cosmos Hub will receive [Polytone](https://github.com/DA0-DA0/polytone) to allow for accounts, smart contracts, or DAOs on the Hub to control accounts on other chains, and vice versa.\n\nWe are excited for the new possibilities that will be unlocked for developers and the wider Cosmos community. For example, a smart contract (or DAO) on another CosmWasm chain can control an account on the Hub to stake and interact with dApps. Cosmos teams can also use it to manage outposts across the Interchain. Moreover, Polytone includes an easy to use Interchain Queries implementation, allowing for smart contracts on other chains to reliably query information from the Hub.\n\n## Audits\n\n- [DAO DAO v1](https://github.com/oak-security/audit-reports/blob/master/DAO%20DAO/2022-06-22%20Audit%20Report%20-%20DAO%20DAO%20v1.0.pdf)\n- [DAO DAO v2](https://github.com/oak-security/audit-reports/blob/master/DAO%20DAO/2023-02-06%20Audit%20Report%20-%20DAO%20DAO%202%20v1.0.pdf)\n- [Polytone (smart contract interchain accounts)](https://github.com/oak-security/audit-reports/blob/master/Polytone/2023-06-05%20Audit%20Report%20-%20Polytone%20v1.0.pdf)\n- [Vesting and Payroll Factory](https://github.com/oak-security/audit-reports/blob/master/DAO%20DAO/2023-03-22%20Audit%20Report%20-%20DAO%20DAO%20Vesting%20and%20Payroll%20Factory%20v1.0.pdf)\n- [Token Factory and NFT upgrades](https://github.com/oak-security/audit-reports/blob/master/DAO%20DAO/2023-10-16%20Audit%20Report%20-%20DAO%20DAO%20Updates%20v1.0.pdf)\n- [Veto upgrade](https://github.com/oak-security/audit-reports/blob/master/DAO%20DAO/2024-01-10%20Audit%20Report%20-%20DAO%20DAO%20Veto%20v1.0.pdf)\n\n## Codebases\n\n**DAO Contracts:** https://github.com/DA0-DA0/dao-contracts\n\n**DAO DAO UI:** https://github.com/DA0-DA0/dao-dao-ui\n\n**Polytone:** https://github.com/DA0-DA0/polytone\n\n## Vote options\n\nThe following items summarize the voting options and what they mean for this proposal.\n\n### Yes\n\nYou agree that this proposal was created by the DAO DAO team, and you wish to allow our wallet (cosmos1559zgk3mxm00qtr0zu2x5n4rh5vw704qaqj6ap) to deploy the DAO DAO CosmWasm smart contracts on the Cosmos Hub.\n\n### No\n\nYou do not wish to authorize our wallet (cosmos1559zgk3mxm00qtr0zu2x5n4rh5vw704qaqj6ap) to deploy the DAO DAO CosmWasm smart contracts on the Cosmos Hub, or you believe this proposal or wallet to not belong to the DAO DAO team.\n\n### Abstain\n\nYou wish to contribute to the quorum but formally decline to vote either for or against the proposal.\n\n### NoWithVeto\n\nYou believe this proposal either (1) is deemed to be spam, i.e., irrelevant to the Cosmos Hub, (2) disproportionately infringes on minority interests, or (3) violates or encourages violation of the rules of engagement as currently set out by Cosmos Hub governance. If the number of NoWithVeto votes is greater than 1/3 of total votes, the proposal is rejected and the deposit is burned.",
    "expedited": false
  }
]
```

Link : <https://www.mintscan.io/cosmos/tx/7EC27512903D376C6E6DB88F2F9C35EE3897441F25228882F24ECA3F3D2A2543?height=21462152&sector=json>


### Community Pool Spend

#### CommunityPoolSpendProposal

Injective seems to not use this one, but has a similar type `/injective.exchange.v1beta1.BatchCommunityPoolSpendProposal`.

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


#### MsgCommunityPoolSpend

```json
"body" : {
  "messages": [
    {
      "@type": "/cosmos.gov.v1.MsgSubmitProposal",
      "messages": [
        {
          "@type": "/cosmos.distribution.v1beta1.MsgCommunityPoolSpend",
          "authority": "cosmos10d07y265gmmuvt4z0w9aw880jnsr700j6zn9kn",
          "recipient": "cosmos1eq62mta47ltpmncknzf70v3z70vn834fxxq5ra",
          "amount": [
            {
              "denom": "uatom",
              "amount": "24850000000"
            }
          ]
        }
      ],
      "initial_deposit": [
        {
          "denom": "uatom",
          "amount": "25000000"
        }
      ],
      "proposer": "cosmos1mtdkdv8cf0sdyugffzf80h80fs65qw2xmm3smj",
      "metadata": "Permissionless ICS 3rd Party Audit",
      "title": "Permissionless ICS 3rd Party Audit",
      "summary": "**Authors:** [Simply Staking](https://simplystaking.com/)\n\nTL;DR - Simply Staking will commission [Zellic](https://www.zellic.io/) to conduct a third-party audit of the Permissionless ICS feature. This will follow a similar format and process as our third-party audit conducted last year on Replicated Security ([Prop 687](https://www.mintscan.io/cosmos/proposals/687)) and this year on Hydro ([Prop 927](https://www.mintscan.io/cosmos/proposals/927)), and Interchain Security (ICS) using the Inactive Validator Set ([Prop 943](https://www.mintscan.io/cosmos/proposals/943))\n\n## Background\n\nThis proposal aims to use community pool funds to commission a third-party audit for Permissionless ICS. Permissionless ICS will allow anyone to create an opt-in consumer chain, without a governance proposal. This will allow chains to launch more quickly and with less friction.\n\nFor more information, we advise you to review the [CHIPs forum post](https://forum.cosmos.network/t/proposal-945-voting-chips-signaling-phase-permissionless-ics/14215).\n\nAs we saw in one of our proposals regarding an audit of key Cosmos Infrastructure (ICS) in Proposal #687, it Is always key to get a second set (or more) of auditors who had no involvement in the designing and building of the code to audit the codebase. This will allow for unbiased vulnerabilities to be disclosed (if any).\n\n## Details of Funding Request\n\nZellic, one of the most reputable auditors in the space, will conduct this audit. With the audit scope already known to the auditor, they (Zellic) have presented a quote and timeline for the audit. Zellic is seeking $90,000 for the audit of the Permissionless ICS codebase with an estimated 3.6 engineer-weeks over the course of a 2.4 calendar-week period by 2 Zellic security engineers.\n\nWe believe that the terms and quotes presented by Zellic are fair and ideal. It is a relatively small request for an audit of this importance.\n\n## Management\n\nSince this is a community pool spending proposal, we want to ensure the community that the funds will arrive at the designated recipient by creating a multi-sig.\n\nThe multisig should be comprised of various reputable parties:\n\n* Damien, Simply Staking\n* Jehan, Informal, Inc\n* Brian, Informal, Inc\n\nThe multi-sig address is: cosmos1eq62mta47ltpmncknzf70v3z70vn834fxxq5ra\n\n## **Breakdown of Fees**\n\nWe (Simply Staking) will be the main point of contact with Zellic, meaning we will handle all things related to answering their questions and queries. We will also act as the main coordinator for building and maintaining the multisig to ensure a smooth transfer of funds from the multisig address to the designated recipient (Zellic). For the work with Zellic and the multi-sig coordination, we seek a compensation fee of around 10% of the audit quote.\n\n## **Funding**\n\nZellic Quote: $90,000 + 25% price buffer to account for the volatility of the ATOM token during the voting period: $112,500\n\nSimply Staking Fees: $9,000\n\n* Community consensus via forum and on-chain proposals\n* Sourcing vendor quotes\n* Coordinating vendor payments and milestones\n* Multi-sig coordination\n\nTotal ask ~ $121,500 ~ 24,850 ATOM (at $4.89 as of 27/08)\n\nAll leftover funds will be sent back to the community pool. \n\n## **Governance votes**\n\nThe following items summarize the voting options and what it means for this proposal:\n\nYES - You agree that this external audit should be funded.\n\nNO - You disagree that this external audit should be funded.\n\nNO WITH VETO - A ‘NoWithVeto’ vote indicates a proposal either (1) is deemed to be spam, i.e., irrelevant to Cosmos Hub, (2) disproportionately infringes on minority interests, or (3) violates or encourages violation of the rules of engagement as currently set out by Cosmos Hub governance. If the number of ‘NoWithVeto’ votes is greater than a third of total votes, the proposal is rejected and the deposits are burned.\n\nABSTAIN - You wish to contribute to the quorum but you formally decline to vote either for or against the proposal.",
      "expedited": false
    }
  ],
  "memo": "",
  "timeout_height": "0",
  "extension_options": [],
  "non_critical_extension_options": []
}
```





Link : <https://www.mintscan.io/cosmos/tx/99CA985295052F6EB41DC61D44B56AAA8951A8BACBFA760E6D1BBA6D8043BA52?height=9762753&sector=json>

### Text

Here it's tricky. Both the `summary` and `description` fields can either be an array of strings or a string... nice!

```json
{
  "messages": [
    {
      "@type": "/cosmos.gov.v1.MsgSubmitProposal",
      "messages": [
        {
          "@type": "/cosmos.gov.v1.MsgExecLegacyContent",
          "content": {
            "@type": "/cosmos.gov.v1beta1.TextProposal",
            "title": "Declaration of No Confidence in the Interchain Foundation (ICF) Leadership: A Call to Action",
            "description": [
              "## Summary\n\nThis proposal calls for a vote of no confidence in the ICF leadership, grounded by serious concerns over governance failures, undue influences, neglect of purpose and duty.\n\nWhether established or new, leadership needs awareness if it is to be accountable for the legacy, reputation, and future it stewards. No confidence sends a clear zero-tolerance message to the ICF Foundation Council about where they stand, reasons why, and what must be rectified.\n\nHowever, this proposal is not an indictment; it’s more importantly, a decisive call to action.\n\nAn opportunity for stakeholders to assert constructive asks utilizing reasonable demands and timelines to achieve the transparency that previous calls for accountability failed to secure.\n\nA YES vote for this proposal means signaling support for:\n\n- A demand for Comprehensive Annual Reports for fiscal years 2017-2023 within 60 days.\n- [Specific guidelines](https://github.com/gaiaus/ICF/blob/main/reform/audit_scope_reference) for the content and structure of these Reports.\n- Contingency measures, such as directly petitioning the Supervisory Authority (FSAF) to pursue an audit on the ICF, if needed.\n\n### Links:\n\n[Full text](https://github.com/gaiaus/ICF/blob/main/reform/call_to_action%20english) of the proposal.\n\n[Forum Discussion](https://forum.cosmos.network/t/proposal-last-call-declaration-of-no-confidence-in-icf-leadership-call-to-action/14233)\n\n[Translations](https://github.com/gaiaus/ICF/tree/main/reform): _Chinese (Smpl, Trad), Korean, Japanese, Russian, and French_\n\n## 1. **Preamble**\n\n1.1 **ESTABLISHMENT**\n\nWHEREAS, the Interchain Foundation (ICF) was established on March 07, 2017, as a Swiss foundation (Stiftung) under CHE-199.569.367; and\n\n1.2 **LEGAL CONTEXT**\n\nWHEREAS, pursuant to Swiss law, a Stiftung may be formed solely for a specific, not-for-profit purpose and is subject to regulatory supervision by the Federal Supervisory Authority for Foundations (FSAF/ESA); and\n\n1.3 **MANDATED PURPOSE**\n\nWHEREAS, the ICF's mandated purpose is _\"promoting and developing new technologies and applications, especially in the fields of new open and decentralized software architectures. A dominating but not exclusive focus is set on the promotion and development of the Cosmos Network, the Polkadot Protocol and the related technologies as well as to conduct the necessary fundraising\"_; and\n\n1.4 **FUNDRAISING**\n\nWHEREAS, the ICF conducted an ICO for ATOM in April 2017, raising $17 million, to fund its endowment; and\n\n1.5 **ATOM ALLOCATION**\n\nWHEREAS, the ICF was given 20,277,188.49 ATOM, 10% of the Genesis supply, with the mandate to use these tokens for the interconnected purposes of the Cosmos Plan and its mandate established via charter; and\n\n1.6 **BENEFICIARIES**\n\nWHEREAS, ATOM ICO investors, token holders, validators, developers, and contributors who participate in Cosmos Hub governance are general beneficiaries of the ICF's activities, and hereafter collectively referred to as \"stakeholders\"; and\n\n1.7 **FIDUCIARY DUTY**\n\nWHEREAS, the ICF's ongoing fiduciary duty is evidenced by two key events: the ICO of ATOM tokens and the subsequent allocation of ATOM tokens at Genesis; and\n\n1.8 **RESPONSIBILITY**\n\nWHEREAS, the ICF bears the key responsibility of coordinating distributed development for decentralized Cosmos architectures, necessitating efficient resource allocation, good governance, legal compliance, and stakeholder engagement; and\n\n## 2. **Grounds for No Confidence**\n\nWHEREAS, the grounds for the declaration of no confidence and the \"Call to Action\" include, inter alia:\n\n2.1 **DEVIATION FROM MANDATED PURPOSE**\n\nWHEREAS, prima facie evidence suggests deviation and/or impermissible mutation of the ICF’s mandated purpose; and\n\n2.2 **BREACH OF FIDUCIARY DUTY**\n\nWHEREAS, the ICF has observably failed to uphold its fiduciary duties to ATOM investors and the Cosmos Hub, demonstrating indifference to the vital support ATOM still requires; and\n\nWHEREAS, the failure to prioritize ATOM, despite having received ATOM, BTC, and ETH tokens to fund its endowment and key activities, represents a fundamental breach of trust, dereliction of purpose and core responsibilities; and\n\n2.3 **GOVERNANCE FAILURES**\n\nWHEREAS, lack of competence and expertise in foundation management has led to governance deficiencies indicated by:\n\na. Allegations of anti-competitive practices and abuse of dominant position by members of the Foundation Council (FC), potentially violating applicable competition laws and undermining principles of fair governance;\n\nb. Governance instability highlighted by the resignation and replacement of two FC Presidents within an eleven-month period;\n\nc. Inadequate checks and balances, and ineffective oversight mechanisms; and\n\n2.4 **CONFLICTS OF INTEREST**\n\nWHEREAS, unmitigated conflicts of interest impair critical functions in resource allocation and decision-making, particularly affecting grants, vendor agreements, investments, and token delegations; and\n\n2.5 **LACK OF TRANSPARENCY AND ACCOUNTABILITY**\n\nWHEREAS, the ICF claims comprehensive statutory reporting to the FSAF, including \"ordinary audits,\" but the confidentiality of these filings prevents stakeholder verification; and\n\nWHEREAS, despite the ICF's compliance with Swiss reporting requirements, its nearly eight years of non-disclosure and obfuscation have resulted in significant information asymmetry, demonstrating a failure to fully meet its transparency and accountability obligations to stakeholders; and\n\nWHEREAS, this deliberate withholding of information appears strategically aimed at frustrating stakeholder demands for reform, creating systemic information barriers that disadvantage stakeholders and threaten the long-term viability of the Cosmos Hub and Cosmos project; and\n\n2.6 **FINANCIAL MISMANAGEMENT**\n\nWHEREAS, the ICF has shows signs of imprudent Treasury management, with funding disruptions likely stemming from poor risk management, diversification, and liquidity strategies; and\n\nWHEREAS, the ICF's asset reporting fails to meet reasonable disclosure standards; and\n\nWHEREAS, the ICF's failure to provide stakeholders with timely, comprehensive reports on significant asset changes and management strategies undermines accountability and breaches its fiduciary duty as a not-for-profit purpose foundation; and\n\nWHEREAS, there are concerns about potential misuse of funds, including allegations of misappropriation and embezzlement, which, if proven, would violate civil and criminal laws; and\n\n2.7 **EROSION OF STAKEHOLDER TRUST**\n\nWHEREAS, all of the above-enumerated concerns extend beyond ethical considerations and/or violations of the ICF charter, but may also possibly contravene the Swiss Civil Code, Swiss Code of Obligations, Swiss Criminal Code, and Swiss Foundation laws; and\n\nWHEREAS, these urgent issues necessitate immediate and decisive stakeholder action;\n\n## 3. Call to Action\n\nNOW, THEREFORE, We, the stakeholders of the Cosmos Hub, being duly empowered and justified by the foregoing, do hereby resolve to address the ICF’s systemic failures through the following:\n\n3.1 **DECLARATION OF NO CONFIDENCE**\n\nWe hereby DECLARE formally our vote of no confidence in the ICF Foundation Council (FC) leadership;\n\n3.2 **PETITION FOR COMPREHENSIVE ANNUAL REPORTS**\n\nWe hereby PETITION the ICF to publish Comprehensive Annual Reports (hereinafter \"Reports\") for fiscal years 2017-2023, as follows:\n\n3.2.1 Individual Reports for each fiscal year; no consolidated Reports.\n\n3.2.2 Each Report shall include:\n\na. All financial and operational information necessary to understand ICF's activities and programs\n\nb. A reference to the appended [Audit Scope Reference](https://github.com/gaiaus/ICF/blob/main/reform/audit_scope_reference) Document, which confirms:\n\n- Areas included in the annual audit\n- Areas not covered and why\n- Alternative evaluation measures for uncovered areas\n\nc. Relevant audit findings, as deemed appropriate by ICF\n\nd. Additional information f",
              "or comprehensive understanding of ICF operations\n\n3.2.3 In cases where certain information cannot be disclosed due to confidentiality constraints, the ICF shall:\n\na. State the nature of the information that cannot be disclosed\n\nb. Provide the legal/regulatory basis for non-disclosure\n\nc. Offer any permissible summary or redacted version of information that can be shared\n\n_The above-described reporting structure defines the [desired scope for the requested Comprehensive Annual Reports](https://github.com/gaiaus/ICF/blob/main/reform/audit_scope_reference)_.\n\n_Including relevant audit findings, as deemed appropriate by the ICF, further enhances transparency while maintaining necessary confidentiality_.\n\n3.3 **TIMELINE FOR DELIVERY**\n\nWe CALL FOR the Reports to be published within 60 days of this proposal's passage.\n\n3.4 **CONTINGENCY MEASURES**\n\nWe ESTABLISH:\n\n3.4.1 In the event Reports are not delivered within 60 days, the stakeholders will petition the FSAF directly for an appropriate audit.\n\n3.4.2 If discovered that the ICF has misrepresented the scope of its annual audits, stakeholders will report the misrepresentation and petition the FSAF for an audit and investigation, irrespective of the 60-day waiting period.\n\n3.5 **AFFIRMATION OF INTENT**\n\nWe AFFIRM this action aims to realign the ICF with its mission and restore stakeholder trust.\n\n## 4. Voting Options\n\n**YES** - Expresses no confidence in ICF leadership and supports the petition for reports and potential FSAF involvement.\n\n**NO** - Does not approve this proposal.\n\n**ABSTAIN** - Contributes to quorum without voting for or against.\n\n**NO WITH VETO** - Indicates the proposal is spam, infringes on minority interests, or violates governance rules. If exceeding 1/3 of total votes, the proposal is rejected and deposits burned.\n"
            ]
          },
          "authority": "cosmos10d07y265gmmuvt4z0w9aw880jnsr700j6zn9kn"
        }
      ],
      "initial_deposit": [
        {
          "denom": "uatom",
          "amount": "250000000"
        }
      ],
      "proposer": "cosmos122waxj6n64r8gy7k7g07ktzfnw557zph4wkhyh",
      "metadata": "Declaration of No Confidence in the Interchain Foundation (ICF) Leadership, A Call to Action",
      "title": "Declaration of No Confidence in the Interchain Foundation (ICF) Leadership: A Call to Action",
      "summary": [
        "## Summary\n\nThis proposal calls for a vote of no confidence in the ICF leadership, grounded by serious concerns over governance failures, undue influences, neglect of purpose and duty.\n\nWhether established or new, leadership needs awareness if it is to be accountable for the legacy, reputation, and future it stewards. No confidence sends a clear zero-tolerance message to the ICF Foundation Council about where they stand, reasons why, and what must be rectified.\n\nHowever, this proposal is not an indictment; it’s more importantly, a decisive call to action.\n\nAn opportunity for stakeholders to assert constructive asks utilizing reasonable demands and timelines to achieve the transparency that previous calls for accountability failed to secure.\n\nA YES vote for this proposal means signaling support for:\n\n- A demand for Comprehensive Annual Reports for fiscal years 2017-2023 within 60 days.\n- [Specific guidelines](https://github.com/gaiaus/ICF/blob/main/reform/audit_scope_reference) for the content and structure of these Reports.\n- Contingency measures, such as directly petitioning the Supervisory Authority (FSAF) to pursue an audit on the ICF, if needed.\n\n### Links:\n\n[Full text](https://github.com/gaiaus/ICF/blob/main/reform/call_to_action%20english) of the proposal.\n\n[Forum Discussion](https://forum.cosmos.network/t/proposal-last-call-declaration-of-no-confidence-in-icf-leadership-call-to-action/14233)\n\n[Translations](https://github.com/gaiaus/ICF/tree/main/reform): _Chinese (Smpl, Trad), Korean, Japanese, Russian, and French_\n\n## 1. **Preamble**\n\n1.1 **ESTABLISHMENT**\n\nWHEREAS, the Interchain Foundation (ICF) was established on March 07, 2017, as a Swiss foundation (Stiftung) under CHE-199.569.367; and\n\n1.2 **LEGAL CONTEXT**\n\nWHEREAS, pursuant to Swiss law, a Stiftung may be formed solely for a specific, not-for-profit purpose and is subject to regulatory supervision by the Federal Supervisory Authority for Foundations (FSAF/ESA); and\n\n1.3 **MANDATED PURPOSE**\n\nWHEREAS, the ICF's mandated purpose is _\"promoting and developing new technologies and applications, especially in the fields of new open and decentralized software architectures. A dominating but not exclusive focus is set on the promotion and development of the Cosmos Network, the Polkadot Protocol and the related technologies as well as to conduct the necessary fundraising\"_; and\n\n1.4 **FUNDRAISING**\n\nWHEREAS, the ICF conducted an ICO for ATOM in April 2017, raising $17 million, to fund its endowment; and\n\n1.5 **ATOM ALLOCATION**\n\nWHEREAS, the ICF was given 20,277,188.49 ATOM, 10% of the Genesis supply, with the mandate to use these tokens for the interconnected purposes of the Cosmos Plan and its mandate established via charter; and\n\n1.6 **BENEFICIARIES**\n\nWHEREAS, ATOM ICO investors, token holders, validators, developers, and contributors who participate in Cosmos Hub governance are general beneficiaries of the ICF's activities, and hereafter collectively referred to as \"stakeholders\"; and\n\n1.7 **FIDUCIARY DUTY**\n\nWHEREAS, the ICF's ongoing fiduciary duty is evidenced by two key events: the ICO of ATOM tokens and the subsequent allocation of ATOM tokens at Genesis; and\n\n1.8 **RESPONSIBILITY**\n\nWHEREAS, the ICF bears the key responsibility of coordinating distributed development for decentralized Cosmos architectures, necessitating efficient resource allocation, good governance, legal compliance, and stakeholder engagement; and\n\n## 2. **Grounds for No Confidence**\n\nWHEREAS, the grounds for the declaration of no confidence and the \"Call to Action\" include, inter alia:\n\n2.1 **DEVIATION FROM MANDATED PURPOSE**\n\nWHEREAS, prima facie evidence suggests deviation and/or impermissible mutation of the ICF’s mandated purpose; and\n\n2.2 **BREACH OF FIDUCIARY DUTY**\n\nWHEREAS, the ICF has observably failed to uphold its fiduciary duties to ATOM investors and the Cosmos Hub, demonstrating indifference to the vital support ATOM still requires; and\n\nWHEREAS, the failure to prioritize ATOM, despite having received ATOM, BTC, and ETH tokens to fund its endowment and key activities, represents a fundamental breach of trust, dereliction of purpose and core responsibilities; and\n\n2.3 **GOVERNANCE FAILURES**\n\nWHEREAS, lack of competence and expertise in foundation management has led to governance deficiencies indicated by:\n\na. Allegations of anti-competitive practices and abuse of dominant position by members of the Foundation Council (FC), potentially violating applicable competition laws and undermining principles of fair governance;\n\nb. Governance instability highlighted by the resignation and replacement of two FC Presidents within an eleven-month period;\n\nc. Inadequate checks and balances, and ineffective oversight mechanisms; and\n\n2.4 **CONFLICTS OF INTEREST**\n\nWHEREAS, unmitigated conflicts of interest impair critical functions in resource allocation and decision-making, particularly affecting grants, vendor agreements, investments, and token delegations; and\n\n2.5 **LACK OF TRANSPARENCY AND ACCOUNTABILITY**\n\nWHEREAS, the ICF claims comprehensive statutory reporting to the FSAF, including \"ordinary audits,\" but the confidentiality of these filings prevents stakeholder verification; and\n\nWHEREAS, despite the ICF's compliance with Swiss reporting requirements, its nearly eight years of non-disclosure and obfuscation have resulted in significant information asymmetry, demonstrating a failure to fully meet its transparency and accountability obligations to stakeholders; and\n\nWHEREAS, this deliberate withholding of information appears strategically aimed at frustrating stakeholder demands for reform, creating systemic information barriers that disadvantage stakeholders and threaten the long-term viability of the Cosmos Hub and Cosmos project; and\n\n2.6 **FINANCIAL MISMANAGEMENT**\n\nWHEREAS, the ICF has shows signs of imprudent Treasury management, with funding disruptions likely stemming from poor risk management, diversification, and liquidity strategies; and\n\nWHEREAS, the ICF's asset reporting fails to meet reasonable disclosure standards; and\n\nWHEREAS, the ICF's failure to provide stakeholders with timely, comprehensive reports on significant asset changes and management strategies undermines accountability and breaches its fiduciary duty as a not-for-profit purpose foundation; and\n\nWHEREAS, there are concerns about potential misuse of funds, including allegations of misappropriation and embezzlement, which, if proven, would violate civil and criminal laws; and\n\n2.7 **EROSION OF STAKEHOLDER TRUST**\n\nWHEREAS, all of the above-enumerated concerns extend beyond ethical considerations and/or violations of the ICF charter, but may also possibly contravene the Swiss Civil Code, Swiss Code of Obligations, Swiss Criminal Code, and Swiss Foundation laws; and\n\nWHEREAS, these urgent issues necessitate immediate and decisive stakeholder action;\n\n## 3. Call to Action\n\nNOW, THEREFORE, We, the stakeholders of the Cosmos Hub, being duly empowered and justified by the foregoing, do hereby resolve to address the ICF’s systemic failures through the following:\n\n3.1 **DECLARATION OF NO CONFIDENCE**\n\nWe hereby DECLARE formally our vote of no confidence in the ICF Foundation Council (FC) leadership;\n\n3.2 **PETITION FOR COMPREHENSIVE ANNUAL REPORTS**\n\nWe hereby PETITION the ICF to publish Comprehensive Annual Reports (hereinafter \"Reports\") for fiscal years 2017-2023, as follows:\n\n3.2.1 Individual Reports for each fiscal year; no consolidated Reports.\n\n3.2.2 Each Report shall include:\n\na. All financial and operational information necessary to understand ICF's activities and programs\n\nb. A reference to the appended [Audit Scope Reference](https://github.com/gaiaus/ICF/blob/main/reform/audit_scope_reference) Document, which confirms:\n\n- Areas included in the annual audit\n- Areas not covered and why\n- Alternative evaluation measures for uncovered areas\n\nc. Relevant audit findings, as deemed appropriate by ICF\n\nd. Additional information f",
        "or comprehensive understanding of ICF operations\n\n3.2.3 In cases where certain information cannot be disclosed due to confidentiality constraints, the ICF shall:\n\na. State the nature of the information that cannot be disclosed\n\nb. Provide the legal/regulatory basis for non-disclosure\n\nc. Offer any permissible summary or redacted version of information that can be shared\n\n_The above-described reporting structure defines the [desired scope for the requested Comprehensive Annual Reports](https://github.com/gaiaus/ICF/blob/main/reform/audit_scope_reference)_.\n\n_Including relevant audit findings, as deemed appropriate by the ICF, further enhances transparency while maintaining necessary confidentiality_.\n\n3.3 **TIMELINE FOR DELIVERY**\n\nWe CALL FOR the Reports to be published within 60 days of this proposal's passage.\n\n3.4 **CONTINGENCY MEASURES**\n\nWe ESTABLISH:\n\n3.4.1 In the event Reports are not delivered within 60 days, the stakeholders will petition the FSAF directly for an appropriate audit.\n\n3.4.2 If discovered that the ICF has misrepresented the scope of its annual audits, stakeholders will report the misrepresentation and petition the FSAF for an audit and investigation, irrespective of the 60-day waiting period.\n\n3.5 **AFFIRMATION OF INTENT**\n\nWe AFFIRM this action aims to realign the ICF with its mission and restore stakeholder trust.\n\n## 4. Voting Options\n\n**YES** - Expresses no confidence in ICF leadership and supports the petition for reports and potential FSAF involvement.\n\n**NO** - Does not approve this proposal.\n\n**ABSTAIN** - Contributes to quorum without voting for or against.\n\n**NO WITH VETO** - Indicates the proposal is spam, infringes on minority interests, or violates governance rules. If exceeding 1/3 of total votes, the proposal is rejected and deposits burned.\n"
      ],
      "expedited": false
    }
  ],
  "memo": "",
  "timeout_height": "0",
  "extension_options": [],
  "non_critical_extension_options": []
}
```

Link : <https://www.mintscan.io/cosmos/tx/4C13E04710C6E0E2CE9961F208B7B9033ABCDCE9352CE77AEFC4B2CCF7365C6C?height=21857893&sector=json>

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

#### Software Upgrade Proposal

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

#### Message Software Upgrade

```json
[
  {
    "@type": "/cosmos.gov.v1.MsgSubmitProposal",
    "messages": [
      {
        "@type": "/cosmos.upgrade.v1beta1.MsgSoftwareUpgrade",
        "authority": "cosmos10d07y265gmmuvt4z0w9aw880jnsr700j6zn9kn",
        "plan": {
          "name": "v20",
          "time": "0001-01-01T00:00:00Z",
          "height": "22443500",
          "info": "{ \"binaries\": { \"darwin/amd64\":\"https://github.com/cosmos/gaia/releases/download/v20.0.0/gaiad-v20.0.0-rc0-darwin-amd64?checksum=sha256:18b1d9c1d816511f202f897ce036fd2a9a8ca9f005e4b48a91cac73250a4c6a1\", \"darwin/arm64\":\"https://github.com/cosmos/gaia/releases/download/v20.0.0/gaiad-v20.0.0-rc0-darwin-arm64?checksum=sha256:284465eac33bd82752766ee47b77041e7b08dc85a501f58eda12d7fe72f1ddfe\", \"linux/amd64\":\"https://github.com/cosmos/gaia/releases/download/v20.0.0/gaiad-v20.0.0-rc0-linux-amd64?checksum=sha256:913ddb55467aa092088e8c10ffdc7d53bb4e70867d1f84c8e002698ec0f77e79\"}}\t\t\t\t\t",
          "upgraded_client_state": null
        }
      }
    ],
    "initial_deposit": [
      {
        "denom": "uatom",
        "amount": "500000000"
      }
    ],
    "proposer": "cosmos1tvqum6psu8waphaatuau2gxpaay4z7zlntmhkv",
    "metadata": "{\"title\":\"Gaia v20 Software Upgrade\",\"summary\":\"### Background\\n\\nThe **Gaia v20** release is a major release that will follow the standard governance process by initially submitting [this post](https://forum.cosmos.network/t/proposal-xxx-draft-gaia-v20-upgrade/14466) on the Cosmos Hub forum. After collecting forum feedback (~ 1 week) and adapting the proposal as required, a governance proposal will be sent to the Cosmos Hub for voting. The on-chain voting period typically lasts one week.\\n\\nOn governance vote approval, validators will be required to update the Cosmos Hub binary at the halt-height specified in the on-chain proposal.\\n\\n### Release Binary & Upgrade Resources\\n\\nIMPORTANT: Note that Gaia v20.0.0 binary MUST be used.\\n\\n- The release can be found [here](https://github.com/cosmos/gaia/releases/tag/v20.0.0).\\n- The changelog can be found [here](https://github.com/cosmos/gaia/blob/v20.0.0/CHANGELOG.md).\\n- The upgrade guide can be found [here](https://github.com/cosmos/gaia/blob/release/v20.x/UPGRADING.md).\\n\\nUPGRADE NOTES: \\n- Relayer Operators for the Cosmos Hub and consumer chains will also need to update to use [Hermes v1.10.2](https://github.com/informalsystems/hermes/releases/tag/v1.10.2) or higher. You may need to restart your relayer software after a major chain upgrade.\\n\\n### Release Contents\\n\\nThis release adds the following major features:\\n\\n- **Permissionless ICS** (as per [prop 945](https://www.mintscan.io/cosmos/proposals/945)):\\n    -  enables users to *permissionlessly* launch opt-in Consumer Chains on the Cosmos Hub. Given that validators are free to choose whether they want to run a given opt-in Consumer Chain, it is only natural to also enable projects to launch as opt-in Consumer Chains by simply submitting transactions to the Cosmos Hub and, thus, avoiding the need to go through the process of governance. Note that topN Consumer Chains will still need to go through governance. \\n- **ICS with Inactive Validators** (as per [prop 930](https://www.mintscan.io/cosmos/proposals/930)):\\n    - enables validators from outside the Hub’s active set to validate on Consumer Chains. This feature brings the following benefits — it reduces the entry barrier for projects to launch as Consumer Chains since more validators will be allowed to opt in;  it enables validators outside the Hub’s active set to compete by providing their services to interesting projects; it reduces the risk of all the validators of a Consumer Chain opting out, which would require the chain to leave ICS.\\n- **Removal of Unbonding Pausing from ICS** (as described by [ADR 018](https://cosmos.github.io/interchain-security/adrs/adr-018-remove-vscmatured)):\\n    - reduces the complexity of the ICS protocol and removes the dependency between the liveness of undelegation operations on the Cosmos Hub and the liveness of consumer chains.\\n\\nThe release also bumps the following dependencies:\\n\\n- CosmWasm/wasmd to [v0.53.0](https://github.com/CosmWasm/wasmd/releases/tag/v0.53.0).\\n- ibc-go to [v8.5.1](https://github.com/cosmos/ibc-go/releases/tag/v8.5.1).\\n\\n### Testing and Testnets\\n\\nThe v20 release has gone through rigorous testing, including e2e tests, and integration tests. In addition, v20 has been independently tested by the team at Hypha Co-op.\\n\\nValidators and node operators have joined a public testnet to participate in a test upgrade to a release candidate before the Cosmos Hub upgrades to the final release. You can find the relevant information (genesis file, peers, etc.) to join the [Release testnet](https://github.com/cosmos/testnets/tree/master/public)  (theta-testnet-001), or the [Interchain Security testnet](https://github.com/cosmos/testnets/tree/master/interchain-security) (provider).\\n\\n\\n### Potential risk factors\\n\\nAlthough very extensive testing and simulation will have taken place there always exists a risk that the Cosmos Hub might experience problems due to potential bugs or errors from the new features. In the case of serious problems, validators should stop operating the network immediately.\\n\\nCoordination with validators will happen in the [#cosmos-hub-validators-verified](https://discord.com/channels/669268347736686612/798937713474142229) channel of the Cosmos Network Discord to create and execute a contingency plan. Likely this will be an emergency release with fixes or the recommendation to consider the upgrade aborted and revert back to the previous release of gaia (v19.2.0).\\n\\n\\n### Governance votes\\n\\nThe following items summarize the voting options and what it means for this proposal:\\n\\n**YES** - You agree that the Cosmos Hub should be updated with this release.\\n\\n**NO** - You disagree that the Cosmos Hub should be updated with this release.\\n\\n**NO WITH VETO** - A ‘NoWithVeto’ vote indicates a proposal either (1) is deemed to be spam, i.e., irrelevant to Cosmos Hub, (2) disproportionately infringes on minority interests, or (3) violates or encourages violation of the rules of engagement as currently set out by Cosmos Hub governance. If the number of ‘NoWithVeto’ votes is greater than a third of total votes, the proposal is rejected and the deposits are burned.\\n\\n**ABSTAIN** - You wish to contribute to the quorum but you formally decline to vote either for or against the proposal.\"}",
    "title": "Gaia v20 Software Upgrade",
    "summary": "### Background\n\nThe **Gaia v20** release is a major release that will follow the standard governance process by initially submitting [this post](https://forum.cosmos.network/t/proposal-xxx-draft-gaia-v20-upgrade/14466) on the Cosmos Hub forum. After collecting forum feedback (~ 1 week) and adapting the proposal as required, a governance proposal will be sent to the Cosmos Hub for voting. The on-chain voting period typically lasts one week.\n\nOn governance vote approval, validators will be required to update the Cosmos Hub binary at the halt-height specified in the on-chain proposal.\n\n### Release Binary & Upgrade Resources\n\nIMPORTANT: Note that Gaia v20.0.0 binary MUST be used.\n\n- The release can be found [here](https://github.com/cosmos/gaia/releases/tag/v20.0.0).\n- The changelog can be found [here](https://github.com/cosmos/gaia/blob/v20.0.0/CHANGELOG.md).\n- The upgrade guide can be found [here](https://github.com/cosmos/gaia/blob/release/v20.x/UPGRADING.md).\n\nUPGRADE NOTES: \n- Relayer Operators for the Cosmos Hub and consumer chains will also need to update to use [Hermes v1.10.2](https://github.com/informalsystems/hermes/releases/tag/v1.10.2) or higher. You may need to restart your relayer software after a major chain upgrade.\n\n### Release Contents\n\nThis release adds the following major features:\n\n- **Permissionless ICS** (as per [prop 945](https://www.mintscan.io/cosmos/proposals/945)):\n    -  enables users to *permissionlessly* launch opt-in Consumer Chains on the Cosmos Hub. Given that validators are free to choose whether they want to run a given opt-in Consumer Chain, it is only natural to also enable projects to launch as opt-in Consumer Chains by simply submitting transactions to the Cosmos Hub and, thus, avoiding the need to go through the process of governance. Note that topN Consumer Chains will still need to go through governance. \n- **ICS with Inactive Validators** (as per [prop 930](https://www.mintscan.io/cosmos/proposals/930)):\n    - enables validators from outside the Hub’s active set to validate on Consumer Chains. This feature brings the following benefits — it reduces the entry barrier for projects to launch as Consumer Chains since more validators will be allowed to opt in;  it enables validators outside the Hub’s active set to compete by providing their services to interesting projects; it reduces the risk of all the validators of a Consumer Chain opting out, which would require the chain to leave ICS.\n- **Removal of Unbonding Pausing from ICS** (as described by [ADR 018](https://cosmos.github.io/interchain-security/adrs/adr-018-remove-vscmatured)):\n    - reduces the complexity of the ICS protocol and removes the dependency between the liveness of undelegation operations on the Cosmos Hub and the liveness of consumer chains.\n\nThe release also bumps the following dependencies:\n\n- CosmWasm/wasmd to [v0.53.0](https://github.com/CosmWasm/wasmd/releases/tag/v0.53.0).\n- ibc-go to [v8.5.1](https://github.com/cosmos/ibc-go/releases/tag/v8.5.1).\n\n### Testing and Testnets\n\nThe v20 release has gone through rigorous testing, including e2e tests, and integration tests. In addition, v20 has been independently tested by the team at Hypha Co-op.\n\nValidators and node operators have joined a public testnet to participate in a test upgrade to a release candidate before the Cosmos Hub upgrades to the final release. You can find the relevant information (genesis file, peers, etc.) to join the [Release testnet](https://github.com/cosmos/testnets/tree/master/public)  (theta-testnet-001), or the [Interchain Security testnet](https://github.com/cosmos/testnets/tree/master/interchain-security) (provider).\n\n\n### Potential risk factors\n\nAlthough very extensive testing and simulation will have taken place there always exists a risk that the Cosmos Hub might experience problems due to potential bugs or errors from the new features. In the case of serious problems, validators should stop operating the network immediately.\n\nCoordination with validators will happen in the [#cosmos-hub-validators-verified](https://discord.com/channels/669268347736686612/798937713474142229) channel of the Cosmos Network Discord to create and execute a contingency plan. Likely this will be an emergency release with fixes or the recommendation to consider the upgrade aborted and revert back to the previous release of gaia (v19.2.0).\n\n\n### Governance votes\n\nThe following items summarize the voting options and what it means for this proposal:\n\n**YES** - You agree that the Cosmos Hub should be updated with this release.\n\n**NO** - You disagree that the Cosmos Hub should be updated with this release.\n\n**NO WITH VETO** - A ‘NoWithVeto’ vote indicates a proposal either (1) is deemed to be spam, i.e., irrelevant to Cosmos Hub, (2) disproportionately infringes on minority interests, or (3) violates or encourages violation of the rules of engagement as currently set out by Cosmos Hub governance. If the number of ‘NoWithVeto’ votes is greater than a third of total votes, the proposal is rejected and the deposits are burned.\n\n**ABSTAIN** - You wish to contribute to the quorum but you formally decline to vote either for or against the proposal.",
    "expedited": true
  }
]
```

Link : <https://www.mintscan.io/injective/tx/1F05CDF18E51C7C44EEBFFAC9F6F965BA8D84CFAF934194E5BC0474C6DF681AD?height=34417810&sector=json>

