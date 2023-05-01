use sqlite::{self, State};
use super::*;
use std::collections::BTreeSet;

const DB_PATH: &str = "./src/ar_calculator/ar_data.sqlite";

struct Operations {
    connection: sqlite::Connection
}

impl Operations {
    pub fn new() -> Operations {
        Operations {
            connection: sqlite::open(DB_PATH).unwrap()
        }
    }

    fn check_db_path() {
        todo!()
    }

    fn show_tables(&self) {
        let query = "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name;";
        let mut statement = self.connection.prepare(query).unwrap();
        while let Ok(State::Row) = statement.next() {
            println!("table -- {}", statement.read::<String, _>(0).unwrap());
        }
    }

    /// query the csv to get the attack element parameters given id
    pub fn get_attack_element_param(&self,
        attack_element_correct_id: i32,
    ) -> Result<Vec<i32>, Box<dyn Error>> {
        let query = "SELECT * from attack_element_correct_param WHERE `ROW ID` = ?";
        let mut statement = self.connection.prepare(query).unwrap();
        statement.bind((1, attack_element_correct_id as i64)).unwrap();
        // while let Ok(State::Row) = statement.next() {
        //     println!("Row id = {}", statement.read::<i64, _>("Row ID").unwrap());
        // }

        let mut out: Vec<i32> = Vec::new();

        if let Ok(State::Row) = statement.next() {
            out = (1..statement.column_count())
                .map(|i| {
                    statement.read::<i64, _>(i).unwrap() as i32
                })
                .collect();
        }

        Ok(dbg!(out))
    }

    /// query the csv for modifiers depending on the upgrade
    pub fn get_reinforce_param_modifier(&self, reinforce_param_id: i32) -> Result<Vec<f32>, Box<dyn Error>> {
        let query = "SELECT * from reinforce_param_weapon WHERE `ID` = ?";
        let mut statement = self.connection.prepare(query).unwrap();
        statement.bind((1, reinforce_param_id as i64)).unwrap();

        let mut out: Vec<f32> = Vec::new();

        if let Ok(State::Row) = statement.next() {
            out = (1..12)
                .map(|i| {
                    statement.read::<f64, _>(i).unwrap() as f32
                })
                .collect();
        }

        Ok(dbg!(out))
    }

    /// query the csv to get the calc correct graph
    pub fn get_calc_correct_graph_ids(&self, weapon_name: &str) -> Result<Vec<i32>, Box<dyn Error>> {
        let query = "SELECT * from calc_correct_graph_id WHERE `NAME` = ?";
        let mut statement = self.connection.prepare(query).unwrap();
        statement.bind((1, weapon_name)).unwrap();

        let mut out: Vec<i32> = Vec::new();

        if let Ok(State::Row) = statement.next() {
            out = (2..statement.column_count())
                .map(|i| {
                    statement.read::<i64, _>(i).unwrap() as i32
                })
                .collect();
        }

        Ok(dbg!(out))
    }

    /// query the calc correct info given the id
    pub fn get_calc_correct_graphs(&self,
        calc_correct_ids: &Vec<i32>,
    ) -> Result<HashMap<i32, Vec<f32>>, Box<dyn Error>> {
        let mut calc_correct_graphs: HashMap<i32, Vec<f32>> = HashMap::new();

        Ok(dbg!(calc_correct_graphs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_db_tables() {
        let operations = Operations::new();
        operations.show_tables();
    }

    #[test]
    fn get_attack_element_param_valid() {
        let operations = Operations::new();
        operations.get_attack_element_param(10000);
    }

    #[test]
    fn get_reinforce_param_modifier_valid() {
        let operations = Operations::new();
        operations.get_reinforce_param_modifier(3);
    }

    #[test]
    fn get_calc_correct_graph_ids_valid() {
        let operations = Operations::new();
        operations.get_calc_correct_graph_ids("Ruins Greatsword");
    }

    #[test]
    fn get_calc_correct_graphs_valid() {
        let operations = Operations::new();
        let ids = operations.get_calc_correct_graph_ids("Ruins Greatsword").unwrap();
        operations.get_calc_correct_graphs(&ids);
    }
}
