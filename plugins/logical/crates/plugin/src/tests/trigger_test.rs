use serde_json::json;

use crate::behaviour::entity::trigger::TriggerFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use inexor_rgf_graph::NamespacedTypeGetter;
use inexor_rgf_model_logical::Trigger;
use inexor_rgf_model_logical::TriggerProperties::PAYLOAD;
use inexor_rgf_model_logical::BEHAVIOUR_TRIGGER;
use inexor_rgf_model_logical::ENTITY_TYPE_NAME_TRIGGER;
use inexor_rgf_model_logical::ENTITY_TYPE_TRIGGER;
use inexor_rgf_model_logical::NAMESPACE_LOGICAL;
use inexor_rgf_model_result::ResultAny;
use inexor_rgf_model_result::ResultAnyProperties::RESULT;
use inexor_rgf_model_runtime::COMPONENT_ACTION;
use inexor_rgf_runtime_model::Action;
use inexor_rgf_runtime_model::ActionProperties::TRIGGER;

#[test]
fn trigger_test() {
    let entity_ty = ENTITY_TYPE_TRIGGER.clone();
    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(TRIGGER, json!(false))
        .property(RESULT, json!(false))
        .property(PAYLOAD, json!(false))
        .component(COMPONENT_ACTION.clone())
        .build();

    let trigger = Trigger::from(reactive_instance.clone());

    assert_eq!(NAMESPACE_LOGICAL, trigger.namespace().as_str());
    assert_eq!(ENTITY_TYPE_NAME_TRIGGER, trigger.type_name().as_str());

    {
        let behaviour_ty = BEHAVIOUR_TRIGGER.clone();
        let factory = TriggerFactory::new(behaviour_ty.clone());
        let behaviour = factory.create(reactive_instance.clone());
        assert!(behaviour.is_ok());

        trigger.payload(json!("Hello"));
        trigger.trigger();
        assert_eq!("Hello", trigger.result().unwrap().as_str().unwrap());
        trigger.trigger();
        assert_eq!("Hello", trigger.result().unwrap().as_str().unwrap());
        trigger.payload(json!("World"));
        assert_eq!("Hello", trigger.result().unwrap().as_str().unwrap());
        trigger.trigger();
        assert_eq!("World", trigger.result().unwrap().as_str().unwrap());
    }
    // The behaviour has been dropped
    trigger.payload(json!("Inexor"));
    trigger.trigger();
    assert_eq!("World", trigger.result().unwrap().as_str().unwrap());
}
