use std::{convert::TryFrom, str::FromStr};

use serde::Deserialize;
use solana_sdk::{account::Account, pubkey::Pubkey};

#[derive(Debug, Deserialize)]
pub(crate) struct AccountChangeInfo {
  pub value: NotificationValue,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ProgramChangeInfo {}

#[derive(Debug, Deserialize)]
pub(crate) struct NotificationContext {
  pub slot: u64,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum NotificationValue {
  Account(AccountRepresentation),
  Program(u64),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccountRepresentation {
  owner: String,
  executable: bool,
  lamports: u64,
  rent_epoch: u64,
  data: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct NotificationResult {
  pub context: NotificationContext,
  pub value: NotificationValue, // HashMap<String, Value>, //NotificationValue,
}

#[derive(Debug, Deserialize)]
pub(crate) struct NotificationParams {
  pub result: NotificationResult,
  pub subscription: u64,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum SolanaMessage {
  Confirmation {
    jsonrpc: String,
    result: u64,
    id: u64,
  },
  Notification {
    jsonrpc: String,
    method: String,
    params: NotificationParams,
  },
}

impl TryFrom<AccountRepresentation> for Account {
  type Error = crate::Error;
  fn try_from(repr: AccountRepresentation) -> crate::Result<Self> {
    let data = match &repr.data[..] {
      [content, format] => match &format[..] {
        "base64" => base64::decode(&content)?,
        _ => vec![],
      },
      _ => vec![],
    };
    Ok(Account {
      lamports: repr.lamports,
      data: data,
      owner: Pubkey::from_str(&repr.owner)?,
      executable: repr.executable,
      rent_epoch: repr.rent_epoch,
    })
  }
}