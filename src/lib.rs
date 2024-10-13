use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    file_path: String,
    pattern: String
}
pub mod test {

}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self,String> {
        args.next();
        let file_path = match args.next() {
            Some(s) => s,
            None => return Err("missing first argument: file path".to_string())
        };

        let pattern = match args.next() {
            Some(s) => s,
            None => return Err("missing second argument: pattern".to_string())
        };

        Ok(Self {
            file_path,
            pattern
        })
    }
}
pub fn run(config: &Config) -> Result<(),Box<dyn Error>> {
    let text = fs::read_to_string(&config.file_path)?;

    let _ = search(&text,&config.pattern)
        .into_iter()
        .inspect(|line| println!("{line}"))
        .collect::<Vec<_>>();

    Ok(())
}

fn search<'a>(text: &'a str, pattern: &str) -> Vec<&'a str> {
    text
        .lines()
        .filter(|&line| line.contains(pattern))
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build_passed() {
        let config1 = Config::build(
                vec!["main.rs".to_string(),"f".to_string(),"p".to_string()].into_iter()
                );
        let config2 = Config {file_path:"f".to_string(), pattern:"p".to_string()};
        assert_eq!(config1.unwrap(),config2);
    }
    #[test]
    #[should_panic(expected = "missing first argument: file path")]
    fn config_build_failed1() {
        let _ = Config::build(
                vec!["main.rs".to_string()].into_iter()
                ).unwrap();
    }
    #[test]
    #[should_panic(expected = "missing second argument: pattern")]
    fn config_build_failed2() {
        let _ = Config::build(
                vec!["main.rs".to_string(),"f".to_string()].into_iter()
                ).unwrap();
    }
    #[test]
    fn test_run_passed() {
        let config = Config {file_path:".gitignore".to_string(), pattern:"t".to_string()};
        let _ = run(&config);
        
    }
    #[test]
    fn test_run_failed() {
        let config = Config {file_path:"input.txt".to_string(), pattern:"t".to_string()};
        if let Ok(_) = run(&config) {
            panic!("run needed to fail");
        }
    }
    #[test]
    fn test_search1() {
        assert_eq!(search("yolo\nhey\ncool","o"),vec!["yolo","cool"]);
    }
    #[test]
    fn test_search2() {
        let empty: Vec<&str> = Vec::new();
        assert_eq!(search("yolo\nhey\ncool","aa"),empty);
    }
}
