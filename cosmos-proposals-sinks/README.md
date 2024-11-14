## Cosmos Proposals: `Sinks`

### Subgraph (`graph_out`) includes

- `Votes`
- `Deposits`
- `GovernanceParameter`
- `Proposals`
  - `SoftwareUpgrade`
  - `ParameterChange`
  - `CommunityPoolSpend`
  - `ClientUpdate`

## Graph

```mermaid
graph TD;
  graph_out[map: graph_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> graph_out;
  block_index:map_blocks --> graph_out;
  cosmos_governance_parameters:gov_params --> graph_out;
  sf.cosmos.type.v2.Block[source: sf.cosmos.type.v2.Block] --> block_index:index_blocks;
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> block_index:index_blocks;
  block_index:map_blocks[map: block_index:map_blocks];
  block_index:map_blocks:params[params] --> block_index:map_blocks;
  sf.cosmos.type.v2.Block[source: sf.cosmos.type.v2.Block] --> block_index:map_blocks;
  cosmos_governance_parameters:pending_gov_params[store: cosmos_governance_parameters:pending_gov_params];
  cosmos_governance_parameters:events:map_events --> cosmos_governance_parameters:pending_gov_params;
  cosmos_governance_parameters:gov_params[store: cosmos_governance_parameters:gov_params];
  cosmos_governance_parameters:gov_params:params[params] --> cosmos_governance_parameters:gov_params;
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> cosmos_governance_parameters:gov_params;
  cosmos_governance_parameters:events:map_events --> cosmos_governance_parameters:gov_params;
  cosmos_governance_parameters:pending_gov_params --> cosmos_governance_parameters:gov_params;
  cosmos_governance_parameters:events:map_events[map: cosmos_governance_parameters:events:map_events];
  cosmos_governance_parameters:events:block_index:map_blocks --> cosmos_governance_parameters:events:map_events;
  sf.cosmos.type.v2.Block[source: sf.cosmos.type.v2.Block] --> cosmos_governance_parameters:events:block_index:index_blocks;
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> cosmos_governance_parameters:events:block_index:index_blocks;
  cosmos_governance_parameters:events:block_index:map_blocks[map: cosmos_governance_parameters:events:block_index:map_blocks];
  cosmos_governance_parameters:events:block_index:map_blocks:params[params] --> cosmos_governance_parameters:events:block_index:map_blocks;
  sf.cosmos.type.v2.Block[source: sf.cosmos.type.v2.Block] --> cosmos_governance_parameters:events:block_index:map_blocks;
```

## Modules

```yaml
Name: graph_out
Initial block: 0
Kind: map
Input: source: sf.substreams.v1.Clock
Input: map: block_index:map_blocks
Input: store: cosmos_governance_parameters:gov_params
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: ad1c983385173a6354e7129a518903843cb4fe13

Name: block_index:index_blocks
Initial block: 0
Kind: index
Input: source: sf.cosmos.type.v2.Block
Input: source: sf.substreams.v1.Clock
Output Type: proto:sf.substreams.index.v1.Keys
Hash: a5db3ccc9005164c6805e17ee612a40d17d3dbf9

Name: block_index:map_blocks
Initial block: 0
Kind: map
Input: params: message:cosmos.gov.v1beta1 || message:cosmos.gov.v1 || type:active_proposal || type:signal_proposal || type:inactive_proposal || type:submit_proposal || block.number:1
Input: source: sf.cosmos.type.v2.Block
Block Filter: (using *block_index:index_blocks*): `&{}`
Output Type: proto:sf.cosmos.type.v2.Block
Hash: cd0139e1363e6b96b692c5731c93e52fcbae43be

Name: cosmos_governance_parameters:pending_gov_params
Initial block: 0
Kind: store
Input: map: cosmos_governance_parameters:events:map_events
Value Type: string
Update Policy: set
Hash: 0ae11206ce892ca86cde4ca1eb789ec52ec7cb15

Name: cosmos_governance_parameters:gov_params
Initial block: 0
Kind: store
Input: params:
Input: source: sf.substreams.v1.Clock
Input: map: cosmos_governance_parameters:events:map_events
Input: store: cosmos_governance_parameters:pending_gov_params
Value Type: string
Update Policy: set
Hash: 129cbb410dbec84dcd7717edd5eead9893f1feaf

Name: cosmos_governance_parameters:events:map_events
Initial block: 0
Kind: map
Input: map: cosmos_governance_parameters:events:block_index:map_blocks
Output Type: proto:cosmos.proposals.v1.Events
Hash: 376ed13bfeff44669d625914162a50f7435310d2

Name: cosmos_governance_parameters:events:block_index:index_blocks
Initial block: 0
Kind: index
Input: source: sf.cosmos.type.v2.Block
Input: source: sf.substreams.v1.Clock
Output Type: proto:sf.substreams.index.v1.Keys
Hash: a5db3ccc9005164c6805e17ee612a40d17d3dbf9

Name: cosmos_governance_parameters:events:block_index:map_blocks
Initial block: 0
Kind: map
Input: params: message:cosmos.gov.v1beta1 || message:cosmos.gov.v1 || type:active_proposal || type:signal_proposal || type:inactive_proposal || type:submit_proposal || block.number:1
Input: source: sf.cosmos.type.v2.Block
Block Filter: (using *cosmos_governance_parameters:events:block_index:index_blocks*): `&{}`
Output Type: proto:sf.cosmos.type.v2.Block
Hash: cd0139e1363e6b96b692c5731c93e52fcbae43be
```
