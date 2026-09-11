use nova_crypto::hash_bytes;
use nova_state::State;
use nova_types::Hash;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Genesis {
    pub chain_id: u64,
    pub state_root: Hash,
}

pub fn genesis_hash(genesis: &Genesis) -> Hash {
    let mut bytes = Vec::with_capacity(40);
    bytes.extend_from_slice(&genesis.chain_id.to_le_bytes());
    bytes.extend_from_slice(&genesis.state_root);
    hash_bytes(&bytes)
}

pub struct NovaNode {
    pub state: State,
}

impl NovaNode {
    pub fn new(state: State) -> Self {
        Self { state }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_hash_is_deterministic() {
        let genesis = Genesis { chain_id: 1, state_root: [7; 32] };
        assert_eq!(genesis_hash(&genesis), genesis_hash(&genesis));
    }
}
