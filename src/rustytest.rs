/*
Version 1.0 of RustyFrame
 */
use std::fs;
use std::path::Path;

use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Typee {
    F32,
    STRING,
    NU
}

pub struct Block {
    pub type_f32: f32,
    pub type_String: String,
    pub type_Typee: Typee
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.type_Typee == Typee::F32 {
            return write!(f, "{}", self.type_f32);
        } else if self.type_Typee == Typee::STRING {
            return write!(f, "{}", self.type_String);
        } else if self.type_Typee == Typee::NU {
            return write!(f, "0");
        }
        write!(f, "")
    }
}

impl Block {
    pub fn getf(&self) -> f32 {
        self.type_f32.clone()
    }
    pub fn gets(&self) -> String {
        self.type_String.clone()
    }
}

pub struct RustyTest {
    pub headers: Vec<String>,
    pub main_frame: Vec<Vec<Block>>,
}

// public self methods
impl RustyTest {
    pub fn pp(l: &Vec<Block>) {
        for b in l {
            print!("{} | ", b);
        }

        print!("\n");
    }

    pub fn print_all(&self) {
        println!("{:?}", &self.headers);

        for l in &self.main_frame {
            RustyTest::pp(l);
        }
    }

    pub fn print(&self) {
        println!("{:?}", &self.headers);

        let mut i = 0;
        for l in &self.main_frame {
            RustyTest::pp(l);

            i += 1;
            if i >= 5 { break; }
        }

        println!("...");
    }

    pub fn print_type(&self) {
        println!("{:?}", &self.headers);

        let mut i = 0;
        for l in &self.main_frame {
            for b in l {
                print!("{:?} | ", b.type_Typee);
            }

            print!("\n");

            i += 1;
            if i >= 5 { break; }
        }

        println!("...");
    }

    pub fn to_csv(&self, path: &Path) {
        let mut file_string = String::from("");

        for h in &self.headers {
            let t = h.to_owned() + ",";
            file_string += &t;
        }
        file_string.pop(); file_string.push_str("\n");

        for i in 0..self.main_frame.len().to_string().parse().unwrap() {
            for c in &self.main_frame[i] {
                let t = c.type_String.clone() + ",";
                file_string += &t;
            }
            file_string.pop(); file_string.push_str("\n");
        }
        file_string.pop();

        fs::write(path, file_string).expect("!!ERROR WRITING!!");
    }

    pub fn add_column(&mut self, name: String) {
        self.headers.push(name);

        for r in self.main_frame.iter_mut() {
            r.push(Block{
                type_f32: 0.0,
                type_String: "0.0".to_string(),
                type_Typee: Typee::NU
            });
        }
    }

    pub fn remove_column(&mut self, name: String) {
        let index = &self.headers.iter().position(|x| *x == name).unwrap();
        let _ = &self.headers.remove(*index);

        for i in 0..self.main_frame.len().to_string().parse().unwrap() {
            let _ = &self.main_frame[i].remove(*index);
        }
    }

    pub fn set(&mut self, row: i32, coloum: i32, value: Block) {
        self.main_frame[row as usize][coloum as usize] = value;
    }

    pub fn get(&self, row: i32, coloum: i32) -> String {
        self.main_frame[row as usize][coloum as usize].type_String.clone()
    }

    pub fn get_as_f32(&self, row: i32, coloum: i32) -> f32 {
        self.main_frame[row as usize][coloum as usize].type_f32.clone()
    }
}

// public methods
impl RustyTest {
    pub fn from_csv(path: &Path) -> RustyTest {
        let lines: Vec<String> = fs::read_to_string(path)
            .expect("!!ERROR OPENING FILE!!")
            .split("\n")
            .map(str::to_string)
            .collect();

        let mut rf = RustyTest {
            headers: lines[0]
                .split(",")
                .map(str::to_string)
                .collect(),
            main_frame: Vec::new()
        };

        // iterates line by line
        for i in 1..(lines.len()) {
            let cur_line: Vec<String> = lines[i]
                .split(",")
                .map(|x| x.to_string())
                .collect();

            rf.main_frame.push(Vec::new());
            for c in cur_line {
                let mut triggered = false;
                let ci = c.parse::<f32>().unwrap_or_else(|_| {
                    rf.main_frame[i-1].push(Block {
                        type_f32: 0.0,
                        type_String: c.to_string(),
                        type_Typee: Typee::STRING
                    });
                    triggered = true;
                    return 0.0;
                });

                if {!triggered} {
                rf.main_frame[i-1].push(Block {
                    type_f32: ci,
                    type_String: c.to_string(),
                    type_Typee: Typee::F32
                });}
            }
        }

        return rf;
    }
}
