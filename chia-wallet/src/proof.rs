use arbitrary::{Arbitrary, Unstructured};
use clvm_traits::{from_clvm, to_clvm, FromClvm, ToClvm};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Proof {
    Lineage(LineageProof),
    Eve(EveProof),
}

impl<Node> FromClvm<Node> for Proof {
    from_clvm!(Node, f, ptr, {
        LineageProof::from_clvm(f, ptr)
            .map(Self::Lineage)
            .or_else(|_| EveProof::from_clvm(f, ptr).map(Self::Eve))
    });
}

impl<Node> ToClvm<Node> for Proof {
    to_clvm!(Node, self, f, {
        match self {
            Self::Lineage(lineage_proof) => lineage_proof.to_clvm(f),
            Self::Eve(eve_proof) => eve_proof.to_clvm(f),
        }
    });
}

impl<'a> Arbitrary<'a> for Proof {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let is_eve = u.ratio(3, 10)?;
        if is_eve {
            Ok(Self::Eve(EveProof {
                parent_coin_info: u.arbitrary()?,
                amount: u.arbitrary()?,
            }))
        } else {
            Ok(Self::Lineage(LineageProof {
                parent_coin_info: u.arbitrary()?,
                inner_puzzle_hash: u.arbitrary()?,
                amount: u.arbitrary()?,
            }))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, ToClvm, FromClvm)]
#[clvm(list)]
pub struct LineageProof {
    pub parent_coin_info: [u8; 32],
    pub inner_puzzle_hash: [u8; 32],
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, ToClvm, FromClvm)]
#[clvm(list)]
pub struct EveProof {
    pub parent_coin_info: [u8; 32],
    pub amount: u64,
}
