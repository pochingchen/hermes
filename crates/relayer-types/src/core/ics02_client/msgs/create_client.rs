//! Definition of domain type message `MsgCreateClient`.

use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgCreateClient as RawMsgCreateClient;
use ibc_proto::Protobuf;

use crate::core::ics02_client::error::Error;
use crate::signer::Signer;
use crate::tx_msg::Msg;

pub const TYPE_URL: &str = "/ibc.core.client.v1.MsgCreateClient";

/// A type of message that triggers the creation of a new on-chain (IBC) client.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgCreateClient {
    pub client_state: Any,
    pub consensus_state: Any,
    pub signer: Signer,
}

impl MsgCreateClient {
    pub fn new(client_state: Any, consensus_state: Any, signer: Signer) -> Result<Self, Error> {
        Ok(MsgCreateClient {
            client_state,
            consensus_state,
            signer,
        })
    }
}

impl Msg for MsgCreateClient {
    type ValidationError = crate::core::ics24_host::error::ValidationError;
    type Raw = RawMsgCreateClient;

    fn route(&self) -> String {
        crate::keys::ROUTER_KEY.to_string()
    }

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgCreateClient> for MsgCreateClient {}

impl TryFrom<RawMsgCreateClient> for MsgCreateClient {
    type Error = Error;

    fn try_from(raw: RawMsgCreateClient) -> Result<Self, Error> {
        let raw_client_state = raw
            .client_state
            .ok_or_else(Error::missing_raw_client_state)?;

        let raw_consensus_state = raw
            .consensus_state
            .ok_or_else(Error::missing_raw_client_state)?;

        MsgCreateClient::new(
            raw_client_state,
            raw_consensus_state,
            raw.signer.parse().map_err(Error::signer)?,
        )
    }
}

impl From<MsgCreateClient> for RawMsgCreateClient {
    fn from(ics_msg: MsgCreateClient) -> Self {
        RawMsgCreateClient {
            client_state: Some(ics_msg.client_state),
            consensus_state: Some(ics_msg.consensus_state),
            signer: ics_msg.signer.to_string(),
        }
    }
}
