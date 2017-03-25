use std::collections::BTreeMap;
type FiledIndex = (usize, usize);
#[allow(dead_code)]
pub struct Config {
    str_fileds: Vec<String>,
    vec_fileds: Vec<Vec<String>>,
    ui8_fileds: Vec<u8>,

    filed_map: BTreeMap<String, FiledIndex>,
}

#[allow(dead_code)]
impl Config {
    pub fn new(proj_name: String,
               path: String,
               ignored: Vec<String>,
               ignored_suffix: Vec<String>,
               build: Vec<String>,
               indent_line_1: u8,
               indent_line_2: u8,
               indent_line_3: u8,
               indent_ls_1: u8)
               -> Config {
        Config {
            str_fileds: vec![proj_name, path],
            vec_fileds: vec![ignored, ignored_suffix, build],
            ui8_fileds: vec![indent_ls_1, indent_line_1, indent_line_2, indent_line_3],
            filed_map: Config::filed_map(),
        }
    }

    pub fn default() -> Config {
        Config::new(String::from("Unknown project name"),
                    String::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    32,
                    4,
                    4,
                    32)
    }

    pub fn set_fileds(&mut self, name: &str, item: &str) -> bool {
        if let Some(filed_index) = self.filed_map.get(name) {
            let &(main_index, sub_index) = filed_index;
            if main_index == 0 {
                self.str_fileds[sub_index] = item.to_string();
            } else if main_index == 1 {
                self.vec_fileds[sub_index].push(item.to_string());
            } else if main_index == 2 {
                if let Ok(value) = item.parse() {
                    self.ui8_fileds[sub_index] = value;
                }
            } else {
                panic!("unexpected error")
            }
            true
        } else {
            false
        }
    }

    pub fn filed_map() -> BTreeMap<String, FiledIndex> {
        let mut result: BTreeMap<String, FiledIndex> = BTreeMap::new();
        result.insert("name".to_string(), (0, 0));
        result.insert("path".to_string(), (0, 1));

        result.insert("ign".to_string(), (1, 0));
        result.insert("ign-sfx".to_string(), (1, 1));
        result.insert("build".to_string(), (1, 2));

        result.insert("idt-ls-1".to_string(), (2, 0));
        result.insert("idt-line-1".to_string(), (2, 1));
        result.insert("idt-line-1".to_string(), (2, 2));
        result.insert("idt-line-1".to_string(), (2, 3));

        result
    }

    pub fn proj_name(&self) -> String {
        self.str_fileds[0].clone()
    }

    pub fn path(&self) -> String {
        self.str_fileds[1].clone()
    }

    pub fn ignored(&self) -> Vec<String> {
        self.vec_fileds[0].clone()
    }

    pub fn ignored_suffix(&self) -> Vec<String> {
        self.vec_fileds[1].clone()
    }

    pub fn build(&self) -> Vec<String> {
        self.vec_fileds[2].clone()
    }

    pub fn indent_ls_1(&self) -> u8 {
        self.ui8_fileds[0].clone()
    }

    pub fn indent_line_1(&self) -> u8 {
        self.ui8_fileds[1].clone()
    }

    pub fn indent_line_2(&self) -> u8 {
        self.ui8_fileds[2].clone()
    }

    pub fn indent_line_3(&self) -> u8 {
        self.ui8_fileds[3].clone()
    }

    pub fn is_ignored(&self, name: &String) -> bool {
        self.ignored().contains(name)
    }

    pub fn is_ignored_suffix(&self, name: &String) -> bool {
        self.ignored_suffix().iter().any(|x| name.ends_with(x))
    }
}
