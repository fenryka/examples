use aws_config::{BehaviorVersion, Region};
use aws_sdk_dynamodb::types::{
    AttributeDefinition, AttributeValue, BillingMode, KeySchemaElement, KeyType,
    OnDemandThroughput, ProvisionedThroughput, ScalarAttributeType,
};
use aws_sdk_dynamodb::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_dynamo::{from_item, Item, to_item};
use std::collections::HashMap;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Outer {
    m_1: String,
    m_2: u8,
}

/// Lists your tables in DynamoDB local.
#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .test_credentials()
        .load()
        .await;
    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config)
        // Override the endpoint in the config to use a local dynamodb server.
        .endpoint_url(
            // DynamoDB run locally uses port 8000 by default.
            "http://localhost:8500",
        )
        .region(Region::new("fakeRegion"))
        .build();

    let client = Client::from_conf(dynamodb_local_config);

    let resp = client.list_tables().send().await?;

    println!("Found {} tables", resp.table_names().len());
    for name in resp.table_names() {
        println!("  {}", name);
        client.delete_table().table_name(name).send().await?;
    }

    println!("Creating");
    client
        .create_table()
        .table_name("test1")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("Col1".to_string())
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("Col2".to_string())
                .key_type(KeyType::Range)
                .build()
                .unwrap(),
        )
        //
        // S/N/B - String, Number, Binary
        //
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("Col1".to_string())
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("Col2".to_string())
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .billing_mode(BillingMode::PayPerRequest)
        .on_demand_throughput(
            OnDemandThroughput::builder()
                .max_read_request_units(-1)
                .max_write_request_units(-1)
                .build(),
        )
        // .provisioned_throughput(
        //     ProvisionedThroughput::builder().write_capacity_units(100).read_capacity_units(100).build().unwrap()
        // )
        .send()
        .await?;

    println!("Putting");
    client
        .put_item()
        .table_name("test1")
        .item("Col1", AttributeValue::S("A".to_string()))
        .item("Col2", AttributeValue::S("1".to_string()))
        .item(
            "A",
            AttributeValue::M(
                to_item(Outer {
                    m_1: "First".to_string(),
                    m_2: 100,
                })
                .unwrap(),
            ),
        )
        .send()
        .await?;

    client
        .put_item()
        .table_name("test1")
        .item("Col1", AttributeValue::S("A".to_string()))
        .item("Col2", AttributeValue::S("2".to_string()))
        .item(
            "A",
            AttributeValue::M(
                to_item(Outer {
                    m_1: "Second".to_string(),
                    m_2: 110,
                })
                .unwrap(),
            ),
        )
        .send()
        .await?;

    client
        .put_item()
        .table_name("test1")
        .item("Col1", AttributeValue::S("A".to_string()))
        .item("Col2", AttributeValue::S("3".to_string()))
        .item(
            "A",
            AttributeValue::M(
                to_item(Outer {
                    m_1: "Third".to_string(),
                    m_2: 120,
                })
                .unwrap(),
            ),
        )
        .send()
        .await?;

    client
        .put_item()
        .table_name("test1")
        .item("Col1", AttributeValue::S("B".to_string()))
        .item("Col2", AttributeValue::S("1".to_string()))
        .item(
            "A",
            AttributeValue::M(
                to_item(Outer {
                    m_1: "Fourth".to_string(),
                    m_2: 200,
                })
                .unwrap(),
            ),
        )
        .send()
        .await?;


    let result = client
        .get_item()
        .table_name("test1")
        .key("Col1", AttributeValue::S("A".to_string()))
        .key("Col2", AttributeValue::S("1".to_string()))
        .send()
        .await;

    println!("{:?}", result);

    let item = result.unwrap().item.unwrap()["A"].as_m().unwrap().clone();
    let o2: Outer = from_item(item).unwrap();

    println!("From Dynamo Query: {:?}", o2);

    let o3 = json!(o2);
    println!("Serde Value: {:?}", o3);
    let o4 : HashMap<String, AttributeValue> = to_item(o3).unwrap();
    println!("Dynamo from Serde {:?}", o4);


    let results = client
        .scan()
        .table_name("test1")
        .filter_expression("#id1.#id2 > :value")
        .expression_attribute_names("#id1", "A")
        .expression_attribute_names("#id2", "m_2")
        .expression_attribute_values(":value", AttributeValue::N("110".to_string()))
        .send()
        .await;

    println!("{:?}", results);

    Ok(())
}
