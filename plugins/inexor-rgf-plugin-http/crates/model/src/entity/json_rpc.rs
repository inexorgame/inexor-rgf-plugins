use crate::model::entity_model;
use crate::model::entity_ty;
use crate::Action;
use crate::ComponentJsonRpc;
use crate::NAMESPACE_HTTP;

entity_ty!(ENTITY_TYPE_JSON_RPC, NAMESPACE_HTTP, ENTITY_TYPE_NAME_JSON_RPC, "json_rpc");

entity_model!(JsonRpc);
impl ComponentJsonRpc for JsonRpc {}
impl Action for JsonRpc {}
