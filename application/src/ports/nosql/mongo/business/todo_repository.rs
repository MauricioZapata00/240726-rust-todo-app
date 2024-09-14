use async_trait::async_trait;
use domain::model::business::task::Task;

#[async_trait]
pub trait TodoRepository {
    async fn get_all_todos(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>>;
    async fn save_todo(&self, task: Task) -> Task;
}