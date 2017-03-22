#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }

    #[test]
    #[should_panic]
    fn bad_move() {
        assert!(false)
    }

    #[test]
    fn events_exists() {
        // TODO: check that lib/data/events.json is present and valid JSON
    }
}
