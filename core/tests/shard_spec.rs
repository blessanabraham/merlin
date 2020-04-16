use merlin_core::timeline::ShardSpec;

#[test]
fn should_deserialize_single_dimension_shard_spec() {
    let json = r#"
    {
        "type": "single",
        "partitionNum": 1,
        "dimension": "foo"
    }
    "#;

    serde_json::from_str::<ShardSpec>(json).unwrap();
}

#[test]
fn should_deserialize_linear_shard_spec() {
    let json = r#"
    {
        "type": "linear"
    }
    "#;

    serde_json::from_str::<ShardSpec>(json).unwrap();
}

#[test]
fn should_deserialize_numbered_shard_spec() {
    let json = r#"
    {
        "type": "numbered"
    }
    "#;

    serde_json::from_str::<ShardSpec>(json).unwrap();
}

#[test]
fn should_deserialize_hash_based_numbered_shard_spec() {
    let json = r#"
    {
        "type": "hashed"
    }
    "#;

    serde_json::from_str::<ShardSpec>(json).unwrap();
}

#[test]
fn should_deserialize_numbered_overwrite_shard_spec() {
    let json = r#"
    {
        "type": "numbered_overwrite"
    }
    "#;

    serde_json::from_str::<ShardSpec>(json).unwrap();
}
