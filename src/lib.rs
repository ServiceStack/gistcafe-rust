#[cfg(test)]
mod gistcafe {

    use::serde_json::*;

    mod inspect {

        use serde_json::*;
        use std::*;
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;
        use std::collections::BTreeMap;
        
        pub fn vars<T>(args : &T)
        where
            T: ?Sized + serde::ser::Serialize,
        {
            if let Ok(inspect_vars_path) = env::var("INSPECT_VARS") {
                if let Ok(json) = to_string(&args) {
                    let vars_path = inspect_vars_path.replace("\\","/");
                    if vars_path.contains("/") {
                        let dir = Path::new(&vars_path).parent().unwrap().to_str().unwrap();
                        let _ = fs::create_dir_all(dir);
                    }

                    let _ = match File::create(&vars_path) {
                        Ok(mut file) => file.write_all(json.as_bytes()),
                        Err(err) => Err(err),
                    };
                }
            }
        }
        
        pub fn dump<T>(args : &T) -> Option<String>
        where
            T: ?Sized + serde::ser::Serialize,
        {
            match to_string_pretty(&args) {
                Ok(json) => {
                    Some(json.replace("\"",""))
                },
                _ => None
            }
        }
        
        pub fn print_dump<T>(args : &T)
        where
            T: ?Sized + serde::ser::Serialize,
        {
            match dump(args) {
                Some(s) => println!("{}", s),
                _ => (),
            }
        }

        pub fn to_list_map<T: Sized>(rows : &Vec<T>) -> Vec<Map<String,Value>>
        where
            T: ?Sized + serde::ser::Serialize,
        {
            let mut to: Vec<Map<String,Value>> = Vec::new();
            for row in rows {
                let map = from_str(&to_string(row).unwrap());
                to.push(map.unwrap());
            }
            return to;
        }

        pub fn all_keys(rows : &Vec<Map<String,Value>>) -> Vec<String>
        {
            let mut to: Vec<String> = Vec::new();
            for row in rows {
                for key in row.keys() {
                    if !to.contains(key) {
                        to.push(key.to_string());
                    }
                }
            }
            return to;
        }

        pub fn dump_table<T: Sized>(rows : &Vec<T>) -> Option<String>
        where
            T: ?Sized + serde::ser::Serialize,
        {
            return dump_table_columns(rows, Vec::new());
        }

        pub fn dump_table_columns<T: Sized>(rows: &Vec<T>, columns: Vec<&str>) -> Option<String>
        where
            T: ?Sized + serde::ser::Serialize,
        {
            if rows.is_empty() {
                return None;
            }
            let map_rows = to_list_map(rows);
            let keys = if columns.is_empty() {
                all_keys(&map_rows) 
            } else { 
                columns.iter().map(|x| x.to_string()).collect()
            };
            let mut col_sizes: BTreeMap<String,usize> = BTreeMap::new();

            for k in &keys {
                let mut max = k.len();
                for row in &map_rows {
                    if let Some(col) = &row.get(k) {
                        max = cmp::max(format!("{}", col).len(), max);
                    }
                }
                col_sizes.insert(k.to_string(), max);
            }

            let col_sizes_length = col_sizes.len();
            let row_width: usize = col_sizes.values().sum::<usize>()
                + (col_sizes_length * 2)
                + (col_sizes_length + 1);

            let dashes = "-".repeat(row_width - 2);
            let mut sb: Vec<String> = Vec::new();
            sb.push(format!("+{}+", dashes));
            let mut head = "|".to_string();
            for k in &keys {
                head = format!("{}{}|", &head, &align_center(k, col_sizes[k], " "));
            }
            sb.push(head.to_string());
            sb.push(format!("|{}|", dashes));

            for row in &map_rows {
                let mut to = "|".to_string();
                for k in &keys {
                    to = format!("{}{}!", &to, &align_auto(&row[k], col_sizes[k], " "));
                }
                sb.push(to);
            }
            sb.push(format!("+{}+", dashes));

            return Some(sb.join("\n"));
        }
        
        pub fn print_dump_table<T: Sized>(rows : &Vec<T>)
        where
            T: ?Sized + serde::ser::Serialize,
        {
            match dump_table(rows) {
                Some(s) => println!("{}", s),
                _ => (),
            }
        }
        
        pub fn print_dump_table_columns<T: Sized>(rows : &Vec<T>, columns: Vec<&str>)
        where
            T: ?Sized + serde::ser::Serialize,
        {
            match dump_table_columns(rows, columns) {
                Some(s) => println!("{}", s),
                _ => (),
            }
        }

        pub fn align_left(str: &str, len: usize, pad: &str) -> String {
            if len > 0 { 
                let a_len = len + 1 - str.len();
                if a_len > 0 {
                    return format!("{}{}{}", pad, str, pad.repeat(a_len));
                }
            }
            return "".into();
        }

        pub fn align_center(str: &str, len: usize, pad: &str) -> String {
            if len > 0 {
                let n_len = str.len();
                let half = (len as f64 / 2.0 - n_len as f64 / 2.0).floor() as usize;
                let odds = ((n_len % 2) as i64 - (len % 2) as i64).abs() as usize;
                return format!("{}{}{}", pad.repeat(half + 1), str, pad.repeat(half + 1 + odds));
            }
            return "".into();
        }

        pub fn align_right(str: &str, len: usize, pad: &str) -> String {
            if len > 0 { 
                let a_len = len + 1 - str.len();
                if a_len > 0 {
                    return format!("{}{}{}", pad.repeat(a_len), str, pad);
                }
            }
            return "".into();
        }

        pub fn align_auto(obj: &Value, len: usize, pad: &str) -> String {
            let str = match obj {
                Value::String(s) => format!("{}", s),
                _ => format!("{}", obj),
            };
            if str.len() <= len {
                return match obj {
                    Value::Number(_) => align_right(&str, len, &pad),
                    _ => align_left(&str, len, &pad),
                }
            }
            return str;
        }

    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn args() -> Value {
        return json!([
            { "a": 1, "b":"foo" },
            { "a": 2.1, "b":"barbar" },
            { "a": 3.21, "b":"bazbazbaz" }
        ]);
    }

    #[test]
    fn inspect_vars() {
        inspect::vars(&args());
    }

    #[test]
    fn inspect_print_dump() {
        inspect::print_dump(&args());
    }

    #[test]
    fn inspect_print_dump_table() {
        inspect::print_dump_table(&args().as_array().unwrap());
    }

    #[tokio::test]
    async fn inspect_print_dump_table_json_api() {

        let org_name = "rust-lang";

        let res = reqwest::Client::new()
            .get(&format!("https://api.github.com/orgs/{}/repos", org_name))
            .header(reqwest::header::USER_AGENT, "gist.cafe")
            .send()
            .await.unwrap();

        let json: Vec<Map<String,Value>> = res.json().await.unwrap();
        let mut org_repos: Vec<Map<String,Value>> = Vec::new();
        for x in json.iter() {
            org_repos.push(json!({
                "name":        x["name"],
                "description": x["description"],
                "lang":        x["language"],
                "watchers":    x["watchers"],
                "forks":       x["forks"],
            }).as_object().unwrap().clone());
        }
        org_repos.sort_by(|a, b| b["watchers"].as_i64().cmp(&a["watchers"].as_i64()));
        
        println!("Top 3 {} GitHub Repos:", org_name);
        inspect::print_dump(&org_repos[1..=3]);

        println!("\nTop 10 {} GitHub Repos:", org_name);
        inspect::print_dump_table(&org_repos[1..=10].iter().map(|x| json!({
            "name":        x["name"],
            "lang":        x["lang"],
            "watchers":    x["watchers"],
            "forks":       x["forks"],
        }).as_object().unwrap().clone()).collect());

        println!("\nTop 10 {} GitHub Repos:", org_name);
        inspect::print_dump_table_columns(&org_repos[1..=10].to_vec(), 
            vec!["name", "lang", "watchers", "forks"]);
    }
}
