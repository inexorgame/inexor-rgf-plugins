use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::entity_behaviour_ty;
use crate::PseudoNumberGenerator;
use crate::NAMESPACE_RANDOM;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_result::ResultNumberI64;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

entity_ty!(ENTITY_TYPE_RANDOM_I64_PSEUDO, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_I64_PSEUDO, "random_i64_pseudo");
behaviour_ty!(BEHAVIOUR_RANDOM_I64_PSEUDO, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_I64_PSEUDO, "random_i64_pseudo");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_I64_PSEUDO, ENTITY_TYPE_RANDOM_I64_PSEUDO, BEHAVIOUR_RANDOM_I64_PSEUDO);

entity_model!(RandomI64Pseudo);
impl Action for RandomI64Pseudo {}
impl PseudoNumberGenerator for RandomI64Pseudo {}
impl ResultNumberI64 for RandomI64Pseudo {}
