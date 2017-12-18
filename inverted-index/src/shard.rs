pub enum ShardType {
    Primary,
    Replica,
}

pub struct Shard {
    index: String,
    shard_type: ShardType,
}

impl Into<String> for ShardType {
    fn into(self) -> String {
        match self {
            Primary => {
                "primary".to_string()
            },
            Replica => {
                "replica".to_string()
            }
        }
    }
}
impl ToString for ShardType {
    fn to_string(&self) -> String {
        match self {
            Primary => {
                "primary".to_string()
            },
            Replica => {
                "replica".to_string()
            }
        }
    }
}

impl Shard {
    fn new<S: Into<String>>(index: S, shard_type: ShardType) -> Shard {
        Shard { 
            index: index.into(),
            shard_type: shard_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_shard() {
        let s = Shard::new("TestShard", ShardType::Primary);
        assert_eq!(s.index, "TestShard");
    }
}