use async_trait::async_trait;
use domain::model::business::task::Task;
use crate::ports::nosql::mongo::business::todo_repository::TodoRepository;

pub struct GetAllTodosService<R: TodoRepository> {
    todo_repository: R,
}
#[async_trait]
pub trait GetAllTodosUseCase{
    async fn process(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>>;
}

#[async_trait]
impl<R: TodoRepository> GetAllTodosUseCase for GetAllTodosService<R> {
    async fn process(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>>{
        self.todo_repository.get_all_todos().await
    }
}