extern crate teleport;


use teleport::{push_attribute, AttributeValue, Attribute};


pub fn main() {
   let class = AttributeValue::List(vec![
      AttributeValue::StaticString("first"),
      AttributeValue::StaticString("second")
   ]);

   let attr = Attribute {
      name: "class",
      value: class,
   };

   let mut rendered = String::new();

   push_attribute(&attr, &mut rendered);

   println!("{}", rendered);
}
