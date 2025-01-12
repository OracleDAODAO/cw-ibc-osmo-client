mod callback;
mod checks;
pub mod client_ibc_msg;
mod ibc_msg;

use cosmwasm_std::IbcOrder;

pub use crate::callback::ReceiveIcaResponseMsg;
pub use crate::checks::{check_order, check_version, SimpleIcaError};
pub use crate::ibc_msg::{
    BalancesResponse, DispatchResponse, IbcQueryResponse, StdAck, WhoAmIResponse,
};

#[cfg(feature = "host")]
pub mod host_ibc_msg;

pub const IBC_APP_VERSION: &str = "simple-ica-v2";
pub const APP_ORDER: IbcOrder = IbcOrder::Unordered;
// we use this for tests to ensure it is rejected
pub const BAD_APP_ORDER: IbcOrder = IbcOrder::Ordered;
