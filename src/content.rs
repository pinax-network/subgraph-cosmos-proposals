use substreams_entity_change::tables::Tables;

pub fn insert_content_entity_json(tables: &mut Tables, id: &str, type_url: &str, json_data: &str) {
    tables
        .create_row("Content", id)
        .set("typeUrl", type_url)
        .set("jsonData", json_data)
        .set("proposal", id);
}

pub fn insert_content_entity_raw_data(tables: &mut Tables, id: &str, type_url: &str, value: &str) {
    tables
        .create_row("Content", id)
        .set("typeUrl", type_url)
        .set("value", value)
        .set("proposal", id);
}
