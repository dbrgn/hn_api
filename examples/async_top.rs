use hn_api::{nonblocking::HnClient, Item, Result, User};

async fn get_items(api: &HnClient, items: &[u32]) -> Result<Vec<(Item, Option<User>)>> {
    let items = api.get_items(items).await?;
    // To convert a form that try_get_authers accepts
    let items: Vec<Option<Item>> = items.into_iter().map(Some).collect();
    let authors = api.try_get_authors(&items).await?;
    // Convert back to the original form
    let items = items.into_iter().map(Option::unwrap);

    let items_and_authors: Vec<_> = items.zip(authors.into_iter()).collect();

    Ok(items_and_authors)
}

async fn print(api: &HnClient, items: &[u32]) {
    let items = get_items(api, items).await.expect("Can not retrive items");

    for (item, user) in items {
        print!("- {}: {}", item.id(), item.title().unwrap_or("?"),);

        if let Some(user) = user {
            print!(" (by {})", format!("{}, karma {}", user.id, user.karma));
        }

        println!();
    }
}

#[tokio::main]
async fn main() {
    println!("What's new on HN:\n");

    let api = HnClient::init().unwrap();

    let top = api.get_top_stories().await.unwrap();
    let new = api.get_new_stories().await.unwrap();
    let best = api.get_best_stories().await.unwrap();

    let count = 10;

    println!("Top {} stories:", count);
    print(&api, &top[..count]).await;

    println!("\nNewest count stories:");
    print(&api, &new[..count]).await;

    println!("\nBest count stories:");
    print(&api, &best[..count]).await;
}
