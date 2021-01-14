use hn_api::{nonblocking::HnClient, Item, Result, User};

async fn get_items(api: &HnClient, items: &[u32]) -> Result<Vec<(Item, User)>> {
    let items = api.get_items(items).await?;
    let authors = api.get_authors(&items).await?;

    let items_and_authors: Vec<_> = items.into_iter().zip(authors.into_iter()).collect();

    Ok(items_and_authors)
}

async fn print(api: &HnClient, items: &[u32]) {
    let items = get_items(api, items).await.expect("Can not retrive items");

    for (item, user) in items {
        let author = format!("{}, karma {}", user.id, user.karma);

        println!(
            "- {}: {} (by {})",
            item.id(),
            item.title().unwrap_or("?"),
            author,
        );
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
