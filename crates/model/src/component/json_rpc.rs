use crate::model::behaviour_ty;
use crate::model::component_behaviour_ty;
use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_HTTP;

properties!(
    JsonRpcProperties,
    (METHOD, "method", "POST"),
    (URL, "url", ""),
    (JSON_RPC_VERSION, "json_rpc_version", "2.0"),
    (PARAMS, "params", {}),
    (RESULT, "result", {}),
    (ERROR, "error", {})
);

component_ty!(COMPONENT_JSON_RPC, NAMESPACE_HTTP, COMPONENT_NAME_JSON_RPC, "json_rpc");
behaviour_ty!(BEHAVIOUR_JSON_RPC, NAMESPACE_HTTP, BEHAVIOUR_NAME_JSON_RPC, "json_rpc");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_JSON_RPC, COMPONENT_JSON_RPC, BEHAVIOUR_JSON_RPC);

component_model!(
    ComponentJsonRpc,
    set method string,
    set url string,
    set request_headers object,
    set payload value,
    get response_headers object,
    get status u64
);
