mod to_do;
/// to shorten the list, you can use interface
// use to_do::structs::done::Done;
// use to_do::structs::pending::Pending;
//---->>>>
use to_do::to_do_factory;
//<<<<----

mod state;
use std::env;
use state::read_file;
use serde_json::value::Value;
use serde_json::Map;

mod processes;
use processes::process_input;
fn main() {
    // let done: Done = Done::new("joined 1kV KUSAMA");
    // println!("{}",done.super_struct.title);
    // println!("{}",done.super_struct.status);
    // let pending: Pending = Pending::new("join 1KV DOT");
    // println!("{}",pending.super_struct.title);
    // println!("{}",pending.super_struct.status);
//---->>>>
    // let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending","make");
    // match to_do_item.unwrap() {
    //     ItemTypes::Pending(item) => item.create(&item.super_struct.title),
    //     ItemTypes::Done(item) => println!("It is a done item with the title: {}",item.super_struct.title)
    // }


//<<<<----

// testing
    // let v=json!([
    //     "test1",
    //     "test2",
    //     "test1",
    //     "test2",   
    // ]);
    // println!("{:?}",v);

    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];
    let state: Map<String, Value> = read_file("./state.json");
    let status: String;
    match &state.get(*&title) {
        Some(result) => {status = result.to_string().replace('\"',"")},
        None => {status = "pending".to_string();}
    }
    let item = to_do_factory(&status, title).expect(&status);
    process_input(item,command.to_string(),&state);
    // state.insert(title.to_string(), json!(status));
    // /// always where Cargo.toml is the "." directory
    // write_to_file("./state.json", &mut state);
}
