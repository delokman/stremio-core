use crate::types::*;
use serde_derive::*;

#[derive(Debug, Deserialize, Serialize)]
// @TODO some generic way to do actions; perhaps enums should be avoided
// or alternatively we'd use a lot of From and Into in order to have separate events for the
// middlwares
pub enum Action {
    // @TODO args
    Init,
    Open,
    // @TODO those are temporary events, remove them
    LoadCatalogs,
    CatalogReceived(Result<CatalogResponse, ()>),
}
// Middleware actions: AddonRequest, AddonResponse


