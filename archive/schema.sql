CREATE TABLE software_upgrade_proposals
(
    proposal_id UInt64,
    initial_deposit_denom String,
    initial_deposit_amount String,
    proposer String,
    authority String,
    title String,
    summary String,
    metadata String,
    plan_name String,
    plan_height Int64,
    plan_info String
) ENGINE = MergeTree()
ORDER BY proposal_id;
