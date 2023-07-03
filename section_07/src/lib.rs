mod list {
    pub struct Tasks {
        pub item: String,
    }
}

mod things_todo;

use crate::things_todo::add_activity;
use things_todo::items_completed;
use crate::things_todo::items_completed::test::test;

fn lets_add_task() {
    let task = list::Tasks {
        item: String::from("tasks"),
    };

    add_activity();  // relative path
    items_completed::remove_taks();
    test();
}

// cargo modules generate tree
// cargo modules generate tree --types --fns --traits
