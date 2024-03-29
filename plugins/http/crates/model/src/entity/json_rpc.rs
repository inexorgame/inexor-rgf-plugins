use crate::ComponentJsonRpc;
use crate::ParsedUrl;
use crate::Url;
use crate::NAMESPACE_HTTP;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_result::ResultObject;
use inexor_rgf_reactive_model_api::entity_model;
use inexor_rgf_runtime_model::Action;

entity_ty!(ENTITY_TYPE_JSON_RPC, NAMESPACE_HTTP, ENTITY_TYPE_NAME_JSON_RPC, "json_rpc");

entity_model!(JsonRpc);
impl ComponentJsonRpc for JsonRpc {}
impl Url for JsonRpc {}
impl ParsedUrl for JsonRpc {}
impl Action for JsonRpc {}
impl ResultObject for JsonRpc {}
