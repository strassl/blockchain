extern crate serde;
extern crate serde_json;

use chain::*;

pub fn serialize(chain: &Chain) -> Vec<u8> {
    serde_json::to_vec(chain).unwrap()
}

pub fn deserialize(data: &Vec<u8>) -> Result<Chain, String> {
    serde_json::from_slice(data.as_slice()).map_err(|_| "Could not deserialize chain".to_owned())
}

#[cfg(test)]
mod test {
    use chain::{Chain, Blockchain};
    use super::{serialize, deserialize};

    #[test]
    fn serialize_deserialize_works() {
        let chain: Chain = Blockchain::init();
        let serialized = serialize(&chain);
        let deserialized = deserialize(&serialized);
        assert_eq!(chain, deserialized.unwrap());
    }
}