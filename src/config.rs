use std::{collections::HashMap, rc::Rc};

pub struct Config {
    pub stylesheet: HashMap<String, HashMap<String, String>>,
    keys: Rc<[String]>,
}

impl Config {
    pub fn load_from(css: &str) -> Self {
        let mut stylesheet: HashMap<String, HashMap<String, String>> = HashMap::new();

        let mut scope_type: u8 = 0; // 0 = outside, 1 = making key, 2 = making value
        let mut sel_acc = "".to_string();
        let mut key_acc = "".to_string();
        let mut val_acc = "".to_string();

        let mut vec_keys = Vec::new();

        let ignores = [' ', '\t', '\n', '\r', '"'];

        let mut previous_char = '\n';
        let mut is_comment = false;

        for c in css.chars() {
            if previous_char == '/' && c == '*' {
                is_comment = true;
            }
            if previous_char == '*' && c == '/' {
                is_comment = false;
            }
            if is_comment {
                previous_char = c;
                continue;
            }

            let mut to_ignore = false;

            for i in 0..ignores.len() {
                if c == ignores[i] {
                    to_ignore = true;
                }
            }

            if to_ignore {
                previous_char = c;
                continue;
            };

            match c {
                '{' => {
                    scope_type = 1;
                    previous_char = c;
                    continue;
                }
                ':' => {
                    if scope_type == 1 {
                        scope_type = 2;
                        previous_char = c;
                        continue;
                    }
                }
                ';' => {
                    for sel in sel_acc.split(",") {
                        stylesheet
                            .entry(sel.to_string())
                            .or_insert_with(|| {
                                vec_keys.push(sel.to_string());
                                HashMap::new()
                            })
                            .insert(key_acc.clone(), val_acc.clone());
                    }

                    key_acc = "".to_string();
                    val_acc = "".to_string();
                    scope_type = 1;
                    previous_char = c;
                    continue;
                }
                '}' => {
                    sel_acc = "".to_string();
                    scope_type = 0;
                    previous_char = c;
                    continue;
                }
                _ => {
                    // nothing
                }
            };

            match scope_type {
                0 => {
                    sel_acc.push(c);
                }
                1 => {
                    key_acc.push(c);
                }
                2 => {
                    val_acc.push(c);
                }
                _ => {
                    panic!("error parsing css");
                }
            };
        }

        let keys = Rc::from_iter(vec_keys);

        Self { stylesheet, keys }
    }

    pub fn get_plugin_args(&self, name: String) -> Vec<String> {
        let q = self.query(format!("#plugin.{}", name));
        let mut args: Vec<String> = vec![];

        for k in q.keys() {
            if !k.starts_with("-") {
                continue;
            }

            let val = q.get(k);
            if let Some(v) = val {
                args.push(k.to_string());
                args.push(v.to_string());
            }
        }

        args
    }

    pub fn query(&self, classes: String) -> HashMap<String, String> {
        let mut applied = HashMap::new();

        if classes.starts_with("#plugin.") {
            return match self.stylesheet.get(&classes) {
                Some(r) => r.clone(),
                None => HashMap::new(),
            };
        };

        let mut copy_from = |key: String| {
            for (k, v) in &self.stylesheet[&key] {
                applied.insert(k.as_str().to_string(), v.as_str().to_string());
            }
        };

        for i in 0..self.keys.len() {
            let key = self.keys[i].clone();
            if key == "*" {
                copy_from(key);
                continue;
            }

            if key.starts_with(classes.as_str()) {
                copy_from(key);
            }
        }

        applied
    }

    pub fn print(&self) {
        println!("{:#?}", self.stylesheet);
        println!("{:#?}", self.keys);
    }
}
