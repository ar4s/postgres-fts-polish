use postgres::{Client, NoTls};

pub struct DbOperations {
    client: Client,
    dict_name: String,
}
impl DbOperations {
    pub fn new(dict_name: String, host: &str, user: &str, password: &str) -> DbOperations {
        let conn_config = format!("host={host} user={user} password={password} dbname=pkd");
        let client = match Client::connect(&conn_config, NoTls) {
            Ok(conn) => conn,
            Err(err) => panic!("Connection error: {err}"),
        };
        DbOperations {
            dict_name: dict_name,
            client: client,
        }
    }
    pub fn create(&mut self) {
        self.client
            .execute(&create_dictionary_query(&self.dict_name), &[])
            .expect("Create dictionary");
        self.client
            .execute(
                &create_text_search_configuration_query(&self.dict_name),
                &[],
            )
            .expect("Create text search configuration");
        self.client
            .execute(&alter_text_search_configuration_query(&self.dict_name), &[])
            .expect("Alter text search configuration");
    }
    pub fn drop(&mut self) {
        self.client
            .execute(&drop_text_search_configuration_query(&self.dict_name), &[])
            .expect("Drop text search configuration");
        self.client
            .execute(&drop_dictionary_query(&self.dict_name), &[])
            .expect("Drop dicitonary");
    }
    pub fn check(&mut self) {
        let query = check_query(&self.dict_name);
        let res = self.client.query(&query, &[]).expect("Check");
        println!("Check execute successfully!");
    }
}
fn create_dictionary_query(name: &str) -> String {
    format!(
        "CREATE TEXT SEARCH DICTIONARY {name} (
          Template = ispell,
          DictFile = polish,
          AffFile = polish,
          StopWords = polish
        );
    "
    )
}

fn create_text_search_configuration_query(name: &str) -> String {
    format!("CREATE TEXT SEARCH CONFIGURATION {name}(parser = default);")
}

fn alter_text_search_configuration_query(name: &str) -> String {
    format!(
        "ALTER TEXT SEARCH CONFIGURATION {name}
      ALTER MAPPING FOR asciiword, asciihword, hword_asciipart, word, hword, hword_part
      WITH {name};"
    )
}

fn drop_dictionary_query(name: &str) -> String {
    format!("DROP TEXT SEARCH DICTIONARY {name};")
}

fn drop_text_search_configuration_query(name: &str) -> String {
    format!("DROP TEXT SEARCH CONFIGURATION {name};")
}

fn check_query(name: &str) -> String {
    format!("SELECT to_tsvector('{name}', 'Litwo ojczyzno moja');")
}
