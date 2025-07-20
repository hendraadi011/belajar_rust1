#[cfg(feature = "hello")]
pub fn say_hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(feature = "hello")]
pub fn say_hello_to_everyone() -> String {
    "Hello everyone".to_string()
}

#[cfg(feature = "bye")]

pub fn say_goodbye(name: &str) -> String {
    format!("Goodbye, {}!", name)
}

#[cfg(feature = "bye")]

pub fn say_goodby_to_everyone() -> String {
    "Say goodby everyone".to_string()
}
