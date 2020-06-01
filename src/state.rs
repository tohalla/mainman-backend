use actix_web::web::Data;
use std::collections::HashMap;
use std::sync::Mutex;

pub type State<'a, T> = HashMap<&'a str, T>;
pub type AppState<'a, T> = Data<Mutex<State<'a, T>>>;

pub fn new_state<'a, T>() -> AppState<'a, T> {
    let state = State::<T>::new();
    Data::new(Mutex::new(state))
}

#[allow(dead_code)]
pub fn set<'a, T>(data: AppState<'a, T>, key: &'a str, value: T) -> Option<T> {
    let mut hashmap = data.lock().expect("Could not acquire lock");
    hashmap.insert(key, value)
}

#[allow(dead_code)]
pub fn get<'a, T>(data: AppState<'a, T>, key: &'a str) -> Option<T>
where
    T: Clone,
{
    let hashmap = data.lock().expect("Could not acquire lock");
    Some(hashmap.get(key)?.to_owned())
}

#[allow(dead_code)]
pub fn delete<'a, T>(data: AppState<'a, T>, key: &'a str) -> Option<T> {
    let mut hashmap = data.lock().expect("Could not acquire lock");
    hashmap.remove(key)
}
