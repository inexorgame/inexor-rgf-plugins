use crate::behaviour::entity::random_bool::RandomBoolFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_random::RandomBool;
use crate::model_random::RandomBoolProperties::RESULT;
use crate::model_random::RandomBoolProperties::TRIGGER;
use crate::model_random::BEHAVIOUR_RANDOM_BOOL;
use crate::model_random::ENTITY_TYPE_RANDOM_BOOL;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

#[test]
fn random_bool_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_RANDOM_BOOL.clone())
        .property_with_default(&TRIGGER)
        .property_with_default(&RESULT)
        .build();
    let random_bool = RandomBool::from(reactive_instance.clone());

    // No behaviour -> no change
    for _ in 0..1000 {
        random_bool.trigger();
        assert!(!random_bool.result().unwrap());
    }

    // With behaviour
    {
        let factory = RandomBoolFactory::new(BEHAVIOUR_RANDOM_BOOL.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        let mut has_true = false;
        let mut has_false = false;
        for _ in 0..1000 {
            random_bool.trigger();
            if random_bool.result().unwrap() {
                has_true = true;
            } else {
                has_false = true;
            }
            if has_true && has_false {
                break;
            }
        }
        assert!(has_true && has_false);
    }
}
