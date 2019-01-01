use hn_api::Api;

fn main() {
    println!("Hello, world!");

    let api = Api::new().unwrap();
    let latest = api.get_latest_item().unwrap();
    println!("Latest: {}", latest);
    let item = api.get_item(latest).unwrap();
    println!("Latest story: {:?}", item);
}
