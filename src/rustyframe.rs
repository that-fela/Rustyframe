/*
Version 1.0 of RustyFrame
 */
use std::fs;
use std::path::Path;

pub struct Rustyframe {
    pub headers: Vec<String>,
    pub main_frame: Vec<Vec<String>>,
}

// public self methods
impl Rustyframe {
    pub fn print_all(&self) {
        println!("{:?}", &self.headers);

        for l in &self.main_frame {
            println!("{:?}", l);
        }
    }

    pub fn print(&self) {
        println!("{:?}", &self.headers);

        let mut i = 0;
        for l in &self.main_frame {
            println!("{:?}", l);
            i += 1;
            if (i >= 5) { break; }
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
            for c in &self.main_frame[i].to_owned() {
                let t = c.to_owned().to_string() + ",";
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
            r.push("0".to_string());
        }
    }

    pub fn remove_column(&mut self, name: String) {
        let index = &self.headers.iter().position(|x| *x == name).unwrap();
        &self.headers.remove(*index);

        for i in 0..self.main_frame.len().to_string().parse().unwrap() {
            &self.main_frame[i].remove(*index);
        }
    }

    pub fn set(&mut self, row: i32, coloum: i32, value: String) {
        self.main_frame[row as usize][coloum as usize] = value;
    }

    pub fn get(&self, row: i32, coloum: i32) -> String {
        self.main_frame[row as usize][coloum as usize].clone()
    }

    pub fn get_as_f32(&self, row: i32, coloum: i32) -> f32 {
        self.main_frame[row as usize][coloum as usize].parse::<f32>().unwrap().clone()
    }
}

// public methods
impl Rustyframe {
    pub fn from_csv(path: &Path) -> Rustyframe {
        let lines: Vec<String> = fs::read_to_string(path)
            .expect("!!ERROR OPENING FILE!!")
            .split("\n")
            .map(|l| l.to_owned())
            .collect();

        let mut rf = Rustyframe {
            headers: lines[0]
                .split(",")
                .map(|p| p.to_owned())
                .collect(),
            main_frame: Vec::new()
        };

        // iterates line by line
        for cur_line in lines.iter() {
            rf.main_frame.push(Vec::new());

            for c in cur_line.split(",") {
                rf.main_frame.last_mut().unwrap().push(c.to_owned());
            }
        }
        rf.main_frame.remove(0);
        return rf;
    }
}

