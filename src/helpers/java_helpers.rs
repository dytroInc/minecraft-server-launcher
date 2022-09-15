use std::process::Command;

// Took code from https://github.com/dolphin2410/server-script/blob/master/src/util/java_util.rs#L5-L31

pub fn check_if_java_command_exists() -> bool {
    if let Ok(_) = Command::new("java").output() {
        true
    } else {
        false
    }
}

pub fn find_java_home() -> Option<String> {
    if let Ok(data) = std::env::var("JAVA_HOME") {
        Some(data)
    } else {
        None
    }
}

pub fn find_java_executable() -> Result<String, String> {
    if check_if_java_command_exists() {
        Ok(String::from("java"))
    } else if let Some(home) = find_java_home() {
        Ok(home)
    } else {
        Err(String::from("Java executable could not be found!"))
    }
}