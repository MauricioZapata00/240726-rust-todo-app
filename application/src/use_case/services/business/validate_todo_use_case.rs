use std::error::Error;
use async_trait::async_trait;
use domain::model::business::task::Task;
use crate::ports::nosql::mongo::business::todo_repository::TodoRepository;

pub struct ValidateTodoService<R: TodoRepository> {
    todo_repository: R,
}

#[async_trait]
pub trait ValidateTodoUseCase{
    async fn process(&self) -> Result<bool, Box<dyn std::error::Error>>;
}

#[async_trait]
impl<R: TodoRepository> ValidateTodoUseCase for ValidateTodoService<R> {
    async fn process(&self) -> Result<bool, Box<dyn std::error::Error>>{
        let todos_result: Result<Vec<Task>, Box<dyn Error>> = self.todo_repository
            .get_all_todos().await;
        let todos = match todos_result {
            Ok(tasks) => tasks,
            Err(current_error) => return Err(Box::from(current_error)),
        };
        Ok(true)
    }
}