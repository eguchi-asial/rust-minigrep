use std::{io::{Error, Read}, fs::File};

pub struct Config {
  pub query: String,
  pub filename: String
}

impl Config {
  pub fn new(args: &Vec<String>) -> Result<Config, &str> {
      if args.len() < 3 {
          return Err("not enouch arguments.");
      }
      let query = args[1].clone();
      let filename = args[2].clone();
      return Ok(Config { query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
  let mut f = File::open(config.filename)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  for line in search(&config.query, &contents) {
    println!("{}", line);
  }
  Ok(())
}


fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // Rustは
        // 安全で速く生産性も高い。
        // 3つ選んで。
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
