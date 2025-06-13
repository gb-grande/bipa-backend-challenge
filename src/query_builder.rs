use crate::nodes;

//structure responsible for creating a transaction to insert nodes in dbs
//at the time of implementation, I didn't knew that there is a similar construction in tokio_postgres     
pub struct UpdateDbQuery {
    querry : String,
    rows : i32,
    finished : bool
}

//receives a node and return node's sql representation
fn get_values(node : &nodes::Node) -> String {
    let value_string : String = format!("('{}', '{}', {}, '{}')", node.public_key, node.alias.replace("'", "''"), node.capacity.to_string(), node.first_seen.to_string());
    return value_string;

}

//methods for the query builder
impl UpdateDbQuery {
    //returns new querry, if no nodes are provided it will clean the database (assuming there aren't any nodes in the api the server fetches its data from)
    pub fn new() -> Self {
        let mut new_querry = Self {
            querry : String::new(),
            rows : 0,
            finished : false
        };
        //begin transaction
        new_querry.querry.push_str("BEGIN;\n");
        //delete previous nodes
        new_querry.querry.push_str("DELETE FROM nodes;\n");
        return new_querry;
    }
    //adds insert into values to querry
    pub fn add_insert(&mut self) {
        if self.finished {
            panic!("Tried to modify finished querry");
        }
        self.querry.push_str("INSERT INTO nodes (public_key, alias, capacity, first_seen)\nVALUES\n");
        return;
    }
    //add node to querry
    pub fn add_node(&mut self, node : &nodes::Node) {
        if self.finished {
            panic!("Tried to modify finished querry");
        }
        //if exists because there is a possibility of empty node list to update db, which means there would be no insert part in the querry
        if self.rows == 0 {
            self.add_insert();
        }
        else {
            self.querry.push_str(",\n");
        }
        //adds values to querry
        self.querry.push_str(&get_values(node));
        self.rows += 1;
        return;
    }
    //finishes querry so it can be used
    pub fn finish_transaction(&mut self) {
        if self.finished {
            panic!("Tried to modify finished querry");
        }
        self.querry.push_str(";\nCOMMIT;");
        self.finished = true;
    }
    //build querry from vector of nodes
    pub fn build_from_nodes(nodes_vec : &Vec<nodes::Node>) -> Self {
        let mut new_querry = UpdateDbQuery::new();
        for node in nodes_vec.iter() {
            new_querry.add_node(node);
        }
        new_querry.finish_transaction();
        return new_querry;
    }
    //returns the finished querry, panics if it isn't finished
    pub fn get_querry(&self) -> &String {
        if self.finished {
            let binding = &self.querry;
            return binding;
        }
        else {
            panic!("Tried to use unfinished querry")
        }
    }
    
}