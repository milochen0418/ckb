use ckb_core::transaction::ProposalShortId as CoreProposalShortId;
use faster_hex::{hex_decode, hex_encode};
use std::fmt;

#[derive(Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct ProposalShortId(pub [u8; 10]);

impl ProposalShortId {
    pub fn new(inner: [u8; 10]) -> ProposalShortId {
        ProposalShortId(inner)
    }

    pub fn into_inner(self) -> [u8; 10] {
        self.0
    }
}

impl From<CoreProposalShortId> for ProposalShortId {
    fn from(core: CoreProposalShortId) -> ProposalShortId {
        ProposalShortId::new(core.into_inner())
    }
}

impl From<ProposalShortId> for CoreProposalShortId {
    fn from(json: ProposalShortId) -> Self {
        CoreProposalShortId::new(json.into_inner())
    }
}

struct ProposalShortIdVisitor;

impl<'b> serde::de::Visitor<'b> for ProposalShortIdVisitor {
    type Value = ProposalShortId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a 0x-prefixed hex string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() < 2 || &v[0..2] != "0x" || v.len() != 22 {
            return Err(E::invalid_value(serde::de::Unexpected::Str(v), &self));
        }
        let mut buffer = [0u8; 10]; // we checked length
        hex_decode(&v.as_bytes()[2..], &mut buffer)
            .map_err(|e| E::custom(format_args!("{:?}", e)))?;
        Ok(ProposalShortId::new(buffer))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }
}

impl serde::Serialize for ProposalShortId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buffer = [0u8; 22];
        buffer[0] = b'0';
        buffer[1] = b'x';
        hex_encode(&self.0, &mut buffer[2..])
            .map_err(|e| serde::ser::Error::custom(&format!("{}", e)))?;
        serializer.serialize_str(unsafe { ::std::str::from_utf8_unchecked(&buffer) })
    }
}

impl<'de> serde::Deserialize<'de> for ProposalShortId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ProposalShortIdVisitor)
    }
}
