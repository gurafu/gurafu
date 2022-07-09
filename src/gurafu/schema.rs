pub struct SchemaBuilder {}

impl SchemaBuilder {
    pub fn new() -> SchemaBuilder {
        println!("{}", "new");
        SchemaBuilder {}
    }

    pub fn create_vertex(&mut self, name: &str) -> &mut SchemaBuilder {
        println!("{}", name);
        self
    }

    pub fn property(&mut self, name: &str, typ: &'static str) -> &mut SchemaBuilder {
        println!("{} {}", name, typ);
        self
    }

    pub fn create(&mut self) -> () {
        println!("{}!", "create");
        ()
    }
}
