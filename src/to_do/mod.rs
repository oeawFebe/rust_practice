pub mod structs;
/// no pub because we only use these in this factory(interface)
use structs::pending::Pending;
use structs::done::Done;

/// We are using ENUM because we want to return two different object types
pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}
pub fn to_do_factory(item_type: &str, item_title: &str) ->
    Result<ItemTypes, &'static str> {
        if item_type == "pending" {
            let pending_item = Pending::new(item_title);
            Ok(ItemTypes::Pending(pending_item))
        }
        else if item_type == "done" {
            let done_item= Done::new(item_title);
            Ok(ItemTypes::Done(done_item))
        }
        else {
            println!("{} is item_type",item_type);
            Err("this is not accepted: {}")
        }
    }