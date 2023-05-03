use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_file::File;
use crate::model_result::ResultObject;
use crate::model_runtime::Action;
use crate::NAMESPACE_CONFIG;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_CONFIG_FILE, NAMESPACE_CONFIG, ENTITY_TYPE_NAME_CONFIG_FILE, "config_file");

entity_model!(ConfigFile);
impl Action for ConfigFile {}
impl File for ConfigFile {}
impl ResultObject for ConfigFile {}
