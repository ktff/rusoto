#![cfg(feature = "dynamodb")]

extern crate rusoto;

use rusoto::dynamodb::{DynamoDbClient, ListTablesInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_tables() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DynamoDbClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListTablesInput::default();

    client.list_tables(&request).unwrap();
}

