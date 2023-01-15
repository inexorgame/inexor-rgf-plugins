use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::NAMESPACE_NUMERIC;

properties!(NumericGateProperties, (LHS, "lhs", 0.0), (RHS, "rhs", 0.0), (RESULT, "result", 0.0));

component_ty!(COMPONENT_NUMERIC_GATE, NAMESPACE_NUMERIC, COMPONENT_NAME_NUMERIC_GATE, "numeric_gate");

entity_model!(NumericGateF64, get result f64, set lhs f64, set rhs f64);
entity_model!(NumericGateI64, get result i64, set lhs i64, set rhs i64);
