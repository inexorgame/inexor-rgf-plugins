use std::sync::Arc;

use serde_json::json;

use crate::behaviour::entity::operation::function::*;
use crate::behaviour::entity::operation::properties::ArithmeticOperationProperties;
use crate::behaviour::entity::operation::ArithmeticOperation;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::ReactiveEntityInstance;
use crate::reactive::Operation;

#[test]
fn arithmetic_operation_behaviour_test() {
    let lhs: f64 = -10.0;
    assert_eq!(-9.0, test_arithmetic_operation_behaviour(FN_INCREMENT, lhs).unwrap());
    assert_eq!(-11.0, test_arithmetic_operation_behaviour(FN_DECREMENT, lhs).unwrap());
}

fn test_arithmetic_operation_behaviour(f: ArithmeticOperationFunction<f64>, v: f64) -> Option<f64> {
    let b = create_arithmetic_operation_behaviour(f);
    b.lhs(json!(v));
    b.result().as_f64()
}

fn create_arithmetic_operation_behaviour(f: ArithmeticOperationFunction<f64>) -> ArithmeticOperation<'static> {
    ArithmeticOperation::new(create_arithmetic_operation_entity(), f)
}

fn create_arithmetic_operation_entity() -> Arc<ReactiveEntityInstance> {
    ReactiveEntityInstanceBuilder::new("abs")
        .property(ArithmeticOperationProperties::LHS.as_ref(), json!(ArithmeticOperationProperties::LHS.default_value()))
        .property(ArithmeticOperationProperties::RESULT.as_ref(), json!(ArithmeticOperationProperties::RESULT.default_value()))
        .build()
}
