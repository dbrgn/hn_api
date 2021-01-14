use hn_api::{nonblocking::HnClient, Item, User};

async fn get_items<'a, I>(
    api: &HnClient,
    items: I,
) -> reqwest::Result<Vec<(Option<Item>, Option<User>)>>
where
    I: IntoIterator<Item = &'a u32>,
{
    let items = api.get_items(items).await?;
    let authors = api.get_authors(&items).await?;

    let items_and_authors: Vec<_> = items.into_iter().zip(authors.into_iter()).collect();

    Ok(items_and_authors)
}

async fn print(api: &HnClient, items: &[u32]) {
    let items = get_items(api, items).await.expect("Can not retrive items");

    for (item, user) in items {
        let author = user.map(|user| format!("{}, karma {}", user.id, user.karma));

        if let Some(item) = item {
            println!(
                "- {}: {} (by {})",
                item.id(),
                item.title().unwrap_or("?"),
                author.unwrap_or_else(|| "?".to_string()),
            );
        };
    }
}

#[tokio::main]
async fn main() {
    println!("What's new on HN:\n");

    let api = HnClient::init().unwrap();

    let top = api.get_top_stories().await.unwrap();
    let new = api.get_new_stories().await.unwrap();
    let best = api.get_best_stories().await.unwrap();

    let count = 3;

    println!("Top {} stories:", count);
    print(&api, &top[..count]).await;

    println!("\nNewest count stories:");
    print(&api, &new[..count]).await;

    println!("\nBest count stories:");
    print(&api, &best[..count]).await;
}
