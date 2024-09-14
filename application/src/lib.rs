pub mod ports {
    pub mod nosql{
        pub mod mongo{
            pub mod business{
                pub mod todo_repository;
            }
        }
    }
    pub mod sql{
        pub mod oracle{
            pub mod calculation{
                pub mod mathematical_result_repository;
            }
        }
    }
}
pub mod use_case {
    pub mod services{
        pub mod business{
            pub mod get_all_todos_use_case;
        }
        pub mod calculation{

        }
    }
}