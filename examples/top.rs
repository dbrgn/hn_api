use hn_api::Api;

fn print(api: &Api, items: &[u32]) {
    for id in items {
        let item = api.get_item(*id).unwrap().unwrap();
        let author = item.author().map(|username| {
            let user = api.get_user(username).unwrap().unwrap();
            format!("{}, karma {}", username, user.karma)
        });
        println!(
            "- {}: {} (by {})",
            item.id(),
            item.title().unwrap_or("?"),
            author.unwrap_or_else(|| "?".to_string()),
        )
    }
}

fn main() {
    println!("What's new on HN:\n");

    let api = Api::new().unwrap();

    let top = api.get_top_stories().unwrap();
    let new = api.get_new_stories().unwrap();
    let best = api.get_best_stories().unwrap();

    let count = 3;

    println!("Top {} stories:", count);
    print(&api, &top[..count]);

    println!("\nNewest count stories:");
    print(&api, &new[..count]);

    println!("\nBest count stories:");
    print(&api, &best[..count]);
}
