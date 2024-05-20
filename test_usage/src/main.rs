// Add the procedural macro dependency in Cargo.toml
// [dependencies]
// mytypeid_macro = { path = "../mytypeid_macro" }

// extern crate mytypeid_macro;

use lazy_static::lazy_static;
use mytypeid_macro::DerMyTypeId;
use mytypeid_trait::{MyTypeId, MyTypeIdStatic};
use other_lib::OtherLibEvent;

#[derive(DerMyTypeId, Clone, Debug)]
struct Foo {}

#[derive(DerMyTypeId, Clone, Debug)]
struct Bar {}

fn main() {
    println!("Foo type id: {}", Foo::get_type_id_static());
    println!("Bar type id: {}", Bar::get_type_id_static());

    println!("Foo type id: {}", Foo::get_type_id_static());
    println!("Bar type id: {}", Bar::get_type_id_static());

    let b: Box<dyn MyTypeId> = Box::new(Foo {});
    let b2: Box<dyn MyTypeId> = Box::new(Bar {});

    println!("Foo type id: {}", b.get_type_id());
    println!("Bar type id: {}", b2.get_type_id());
}
