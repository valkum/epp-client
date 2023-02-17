use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    common::{NoExtension, StringValue},
    message::poll,
    request::{Command, Transaction},
};

pub const XMLNS: &str = "http://www.verisign.com/epp/rgp-poll-1.0";

impl Transaction<NoExtension> for MessagePoll<'_> {}

impl<'a> Command for MessagePoll<'a> {
    type Response = MessagePollResponse;

    const COMMAND: &'static str = poll::MessagePoll::COMMAND;
}

#[derive(Debug, Default, Serialize)]
struct MessagePoll<'a>(poll::MessagePoll<'a>);

// Response

/// Type that represents the &lt;resData&gt; tag for message poll response
/// Either
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum MessagePollResponse<T = poll::MessagePollResponse> {
    RgpPoll(RgpPollData),
    Inner(T),
}

/// Type that represents the &lt;pollData xmlns:rgp-poll=""&gt; tag for message poll response
#[derive(Deserialize, Debug)]
#[serde(rename = "pollData")]
pub struct RgpPollData {
    #[serde(rename = "name")]
    pub name: StringValue<'static>,
    #[serde(rename = "rgpStatus")]
    pub rgp_status: Vec<RgpStatus>,
    #[serde(rename = "reqData")]
    pub requested_at: DateTime<Utc>,
    #[serde(rename = "reportDueDate")]
    pub report_due_at: DateTime<Utc>,
}

/// Type that represents the &lt;rgpStatus&gt; tag for domain rgp restore request response
#[derive(Deserialize, Debug)]
pub struct RgpStatus {
    /// The domain RGP status
    #[serde(rename = "s")]
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::MessagePoll;
    use super::MessagePollResponse;
    use crate::message::poll;
    use crate::response::ResultCode;

    #[test]
    fn request_rgp_poll() {
        let object = response_from_file::<MessagePoll>("response/extensions/rgp_poll_restore.xml");

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        let rgp_poll = match object.res_data.unwrap() {
            MessagePollResponse::RgpPoll(data) => data,
            _ => panic!("Unexpected response type"),
        };
        assert_eq!(rgp_poll.rgp_status[0].status, "pendingRestore".to_string());
    }

    #[test]
    fn request_inner() {
        let object = response_from_file::<MessagePoll>("response/message/poll_domain_transfer.xml");
        let inner = match object.res_data.unwrap() {
            MessagePollResponse::Inner(data) => data,
            _ => panic!("Unexpected response type"),
        };

        let transfer = match inner.message_data {
            poll::MessageData::DomainTransfer(data) => data,
            _ => panic!("Unexpected response type"),
        };
        assert_eq!(transfer.name.to_string(), "eppdev-transfer.com".to_string());
    }
}
