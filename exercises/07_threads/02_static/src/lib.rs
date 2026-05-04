// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    let mid = slice.len() / 2;

    let left = &slice[..mid];
    let right = &slice[mid..];

    let res1 = thread::spawn(move || left.iter().sum::<i32>());
    let res2 = thread::spawn(move || right.iter().sum::<i32>());

    res1.join().unwrap() + res2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}


use std::env;

#[derive(Debug)]
enum AppError {
    ConfigMissing(&'static str),
    InvalidConfig(&'static str),
}

#[derive(Debug)]
struct AppConfig {
    db_url: String,
    port: u16,
    debug: bool,
}

fn load_config() -> Result<AppConfig, AppError> {
    
    let db_url = env::var("DATABASE_URL").map_err(|_|AppError::ConfigMissing("DATABASE_URL"))?;

    let port : u16 = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse().map_err(|_| AppError::InvalidConfig("PORT"))?;

    let debug : bool = env::var("DEBUG").unwrap_or_else(|_| "false".to_string()).parse().map_err(|_| AppError::InvalidConfig("DEBUG"))?;

    if db_url.is_empty() {
        return Err(AppError::InvalidConfig("DATABASE_URL"))
    };

    if port == 0 {
        return Err(AppError::InvalidConfig("PORT"))
    }

    Ok( AppConfig{
        db_url,
        port,
        debug,
    })

}