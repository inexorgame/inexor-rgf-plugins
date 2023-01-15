use serde_json::json;

use crate::behaviour::entity::counter::CounterFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_arithmetic::ActionProperties::RESULT;
use crate::model_arithmetic::ActionProperties::TRIGGER;
use crate::model_arithmetic::Counter;
use crate::model_arithmetic::BEHAVIOUR_COUNTER;
use crate::model_arithmetic::ENTITY_TYPE_COUNTER;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

#[test]
fn rx_counter_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_COUNTER.clone())
        .property(RESULT, json!(0))
        .property(TRIGGER, json!(false))
        .build();

    let counter = Counter::from(reactive_instance.clone());

    let factory = CounterFactory::new(BEHAVIOUR_COUNTER.clone());
    {
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        counter.trigger(true);
        assert_eq!(1, counter.result().unwrap());
        counter.trigger(true);
        assert_eq!(2, counter.result().unwrap());
        counter.trigger(true);
        assert_eq!(3, counter.result().unwrap());
    }

    counter.trigger(true);
    assert_eq!(3, counter.result().unwrap());
}
