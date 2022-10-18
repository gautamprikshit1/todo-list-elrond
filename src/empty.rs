#![no_std]

use elrond_wasm::types::heap::String;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait TodoListContract {
    #[view(getAllItems)]
    #[storage_mapper("todo_list")]
    fn todo_list(&self) -> VecMapper<TodoItem>;

    #[view]
    fn todo_item(&self, id: u64) -> TodoItem {
        self.todo_list().get(id as usize)
    }

    #[init]
    fn init(&self, description: String, priority: Priority) {
        self.todo_list().push(&TodoItem {
            id: 1,
            description,
            status: Status::Todo,
            priority,
        });
    }

    #[endpoint]
    fn add_item(&self, description: String, priority: Priority) {
        self.todo_list().push(&TodoItem {
            id: self.todo_list().len() as u64 + 1,
            description,
            status: Status::Todo,
            priority,
        });
    }

    #[endpoint]
    fn update_item(
        &self,
        id: u64,
        description: Option<String>,
        status: Option<Status>,
        priority: Option<Priority>,
    ) {
        let entry = self.todo_list().get(id as usize);
        let updated_entry = TodoItem {
            id,
            description: description.unwrap_or(entry.description),
            status: status.unwrap_or(entry.status),
            priority: priority.unwrap_or(entry.priority),
        };
        self.todo_list().set(id as usize, &updated_entry);
    }

    #[endpoint]
    fn delete_item(&self, id: u64) {
        self.todo_list().clear_entry(id as usize);
    }
}

#[derive(TopEncode, TopDecode, NestedDecode, NestedEncode, TypeAbi, PartialEq, Clone)]
pub struct TodoItem {
    pub id: u64,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
}

#[derive(TopEncode, TopDecode, NestedDecode, NestedEncode, TypeAbi, PartialEq, Clone)]
pub enum Status {
    Todo,
    Ongoing,
    Done,
    Cancelled,
}

#[derive(TopEncode, TopDecode, NestedDecode, NestedEncode, TypeAbi, PartialEq, Clone)]
pub enum Priority {
    None,
    Low,
    Medium,
    High,
}
