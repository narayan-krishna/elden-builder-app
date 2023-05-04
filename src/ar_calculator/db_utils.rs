use super::*;
use rusqlite::Connection;
use std::collections::BTreeSet;

const DB_PATH: &str = "./src/ar_calculator/ar_data.sqlite";

/// Object used for connecting to and querying the weapon database
pub struct Operations {
    connection: Connection,
}

impl Operations {
    pub fn new() -> Operations {
        Operations {
            connection: Connection::open(DB_PATH).unwrap(),
        }
    }

    fn check_db_path() {
        todo!()
    }

    fn show_tables(&self) -> Result<(), Box<dyn Error>> {
        let query = "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name;";
        let mut stmt = self.connection.prepare(query)?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            eprintln!("table -- {}", row.get::<usize, String>(0)?);
        }

        Ok(())
    }

    /// Queries the csv to get the attack element parameters given id
    pub fn get_attack_element_param(
        &self,
        attack_element_correct_id: i32,
    ) -> Result<Vec<i32>, Box<dyn Error>> {
        let mut out: Vec<i32> = Vec::new();

        let query = "SELECT * from attack_element_correct_param WHERE `Row ID` = ?1";
        let mut stmt = self.connection.prepare(query)?;
        let end = stmt.column_count();
        let mut rows = stmt.query([attack_element_correct_id])?;

        if let Some(row) = rows.next()? {
            out = (1..end)
                .map(|i| row.get::<usize, String>(i).unwrap().parse().unwrap())
                .collect()
        }

        Ok(out)
    }

    /// Queries the csv for modifiers depending on the upgrade
    pub fn get_reinforce_param_modifier(
        &self,
        reinforce_param_id: i32,
    ) -> Result<Vec<f32>, Box<dyn Error>> {
        let mut out: Vec<f32> = Vec::new();

        let query = "SELECT * from reinforce_param_weapon WHERE `ID` = ?1";
        let mut stmt = self.connection.prepare(query)?;
        let mut rows = stmt.query([reinforce_param_id])?;

        if let Some(row) = rows.next()? {
            out = (1..12)
                .map(|i| row.get::<usize, String>(i).unwrap().parse().unwrap())
                .collect()
        }

        Ok(out)
    }

    /// Queries the csv to get the calc correct graph
    pub fn get_calc_correct_graph_ids(
        &self,
        weapon_name: &str,
    ) -> Result<Vec<i32>, Box<dyn Error>> {
        let mut out: Vec<i32> = Vec::new();

        let query = "SELECT * from calc_correct_graph_id WHERE `NAME` = ?1";
        let mut stmt = self.connection.prepare(query)?;
        let end = stmt.column_count();
        let mut rows = stmt.query([weapon_name])?;

        if let Some(row) = rows.next()? {
            out = (2..end)
                .map(|i| row.get::<usize, String>(i).unwrap().parse().unwrap())
                .collect()
        }

        Ok(out)
    }

    /// Queres the calc correct info given the id
    pub fn get_calc_correct_graphs(
        &self,
        calc_correct_ids: &Vec<i32>,
    ) -> Result<HashMap<i32, Vec<f32>>, Box<dyn Error>> {
        let mut calc_correct_graphs: HashMap<i32, Vec<f32>> = HashMap::new();
        let sorted_ids: String = calc_correct_ids
            .into_iter()
            .copied()
            .collect::<BTreeSet<i32>>()
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let query = format!(
            "SELECT * from calc_correct_graph WHERE `ID` IN ({})",
            sorted_ids
        );
        let mut stmt = self.connection.prepare(&query)?;
        let end = stmt.column_count();

        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            let key = row.get::<usize, String>(0)?.parse()?;
            let vals = (2..end)
                .map(|i| row.get::<usize, String>(i).unwrap().parse().unwrap())
                .collect();

            calc_correct_graphs.insert(key, vals);
        }

        Ok(calc_correct_graphs)
    }

    /// Returns a row of weapon data as strings which can be parsed as needed
    pub fn get_raw_weapon_data(&self, weapon_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let query = "SELECT * from raw_data WHERE `NAME` = ?1";
        let mut stmt = self.connection.prepare(&query)?;
        let end = stmt.column_count();
        let mut rows = stmt.query([weapon_name])?;

        if let Some(row) = rows.next().unwrap() {
            let out: Vec<String> = (0..end)
                .map(|i| row.get::<usize, String>(i).unwrap())
                .collect();

            return Ok(out);
        }

        Err("could not find any rows matching weapon name".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_db_tables() {
        let operations = Operations::new();
        assert_eq!(Some(()), operations.show_tables().ok());
    }

    #[test]
    fn valid_get_attack_element_param() {
        let operations = Operations::new();
        let attack_element_param = operations.get_attack_element_param(10013).unwrap();
        assert_eq!(
            attack_element_param,
            vec![1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0]
        );
    }

    #[test]
    fn valid_get_reinforce_param_modifier() {
        let operations = Operations::new();
        let reinforce_param_modifier = operations.get_reinforce_param_modifier(402).unwrap();
        assert_eq!(
            reinforce_param_modifier,
            vec![0.872, 0.872, 0.872, 0.872, 0.872, 1.08, 1.38, 0.56, 1.08, 1.08, 1.08]
        )
    }

    #[test]
    fn valid_get_calc_correct_graph_ids() {
        let operations = Operations::new();
        let calc_correct_graph_ids = operations
            .get_calc_correct_graph_ids("Miquellan Knight's Sword")
            .unwrap();
        assert_eq!(calc_correct_graph_ids, vec![0, 4, 0, 0, 4]);
    }

    #[test]
    fn valid_get_calc_correct_graphs() {
        let operations = Operations::new();
        let calc_correct_graph_ids = vec![0, 4, 0, 0, 4];
        let calc_correct_graph = operations
            .get_calc_correct_graphs(&calc_correct_graph_ids)
            .unwrap();

        assert_eq!(
            calc_correct_graph.get(&0).unwrap(),
            &vec![
                1.0, 18.0, 60.0, 80.0, 150.0, 0.0, 25.0, 75.0, 90.0, 110.0, 1.2, -1.2, 1.0, 1.0,
                1.0
            ]
        );
        assert_eq!(
            calc_correct_graph.get(&4).unwrap(),
            &vec![
                1.0, 20.0, 50.0, 80.0, 99.0, 0.0, 40.0, 80.0, 95.0, 100.0, 1.0, 1.0, 1.0, 1.0, 1.0
            ]
        );
    }
}
