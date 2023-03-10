pub use entity::*;
pub use relation::*;

pub mod entity;
pub mod relation;

use inexor_rgf_core_model as model;
use inexor_rgf_model_result as model_result;
use inexor_rgf_model_trigger as model_trigger;

pub const NAMESPACE_DATE_TIME: &str = "date_time";
