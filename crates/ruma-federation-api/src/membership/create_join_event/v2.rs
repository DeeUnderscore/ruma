//! `/v2/` ([spec])
//!
//! [spec]: https://spec.matrix.org/v1.4/server-server-api/#put_matrixfederationv2send_joinroomideventid

use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedEventId, OwnedRoomId,
};
use serde_json::value::RawValue as RawJsonValue;

use super::RoomState;

const METADATA: Metadata = metadata! {
    method: PUT,
    rate_limited: false,
    authentication: ServerSignatures,
    history: {
        1.0 => "/_matrix/federation/v2/send_join/:room_id/:event_id",
    }
};

/// Request type for the `create_join_event` endpoint.
#[request]
pub struct Request {
    /// The room ID that is about to be joined.
    ///
    /// Do not use this. Instead, use the `room_id` field inside the PDU.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// The event ID for the join event.
    #[ruma_api(path)]
    pub event_id: OwnedEventId,

    /// The PDU.
    #[ruma_api(body)]
    pub pdu: Box<RawJsonValue>,
}

/// Response type for the `create_join_event` endpoint.
#[response]
pub struct Response {
    /// Full state of the room.
    #[ruma_api(body)]
    pub room_state: RoomState,
}

impl Request {
    /// Creates a new `Request` from the given room ID, event ID and PDU.
    pub fn new(room_id: OwnedRoomId, event_id: OwnedEventId, pdu: Box<RawJsonValue>) -> Self {
        Self { room_id, event_id, pdu }
    }
}

impl Response {
    /// Creates a new `Response` with the given room state.
    pub fn new(room_state: RoomState) -> Self {
        Self { room_state }
    }
}
