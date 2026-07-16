use crate::BitVec;

impl rkyv::Archive for BitVec {}
impl rkyv::Serialize for BitVec {}
impl rkyv::Deserialize for BitVec {}
