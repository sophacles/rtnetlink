use futures::stream::StreamExt;

use crate::{
    packet::{
        netlink::{
            header::flags::{NLM_F_ACK, NLM_F_REQUEST},
            NetlinkFlags, NetlinkMessage, NetlinkPayload,
        },
        rtnl::{address::AddressMessage, RtnlMessage},
    },
    Error, ErrorKind, Handle,
};

pub struct AddressDelRequest {
    handle: Handle,
    message: AddressMessage,
}

impl AddressDelRequest {
    pub(crate) fn new(handle: Handle, message: AddressMessage) -> Self {
        AddressDelRequest { handle, message }
    }

    /// Execute the request
    pub async fn execute(self) -> Result<(), Error> {
        let AddressDelRequest {
            mut handle,
            message,
        } = self;

        let mut req = NetlinkMessage::from(RtnlMessage::DelAddress(message));
        req.header.flags = NetlinkFlags::from(NLM_F_REQUEST | NLM_F_ACK);
        let mut response = handle.request(req)?;
        while let Some(msg) = response.next().await {
            if let NetlinkPayload::Error(e) = msg.payload {
                return Err(ErrorKind::NetlinkError(e).into());
            }
        }
        Ok(())
    }

    pub fn message_mut(&mut self) -> &mut AddressMessage {
        &mut self.message
    }
}
