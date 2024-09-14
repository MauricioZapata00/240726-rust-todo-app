use async_trait::async_trait;
use domain::model::calculation::mathematical_result::MathematicalResult;

#[async_trait]
pub trait MathematicalResultRepository {
    async fn save_todo(&self, math_result: MathematicalResult) -> MathematicalResult;
}