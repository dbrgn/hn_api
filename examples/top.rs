use futures::future::{join_all, OptionFuture};
use hn_api::{
    types::{Item, User},
    HnClient,
};

async fn get_item_and_author(
    api: &HnClient,
    id: u32,
) -> reqwest::Result<(Option<Item>, Option<User>)> {
    let item = api.get_item(id).await?;

    let author_future: OptionFuture<_> = item
        .as_ref()
        .and_then(|item| item.author().map(|a| a.to_string()))
        .map(|a| api.get_user(a))
        .into();

    let author = author_future.await.transpose()?.flatten();

    Ok((item, author))
}

async fn get_items<'a, I>(
    api: &HnClient,
    items: I,
) -> reqwest::Result<Vec<(Option<Item>, Option<User>)>>
where
    I: IntoIterator<Item = &'a u32>,
{
    join_all(items.into_iter().map(|id| get_item_and_author(api, *id)))
        .await
        .into_iter()
        .collect()
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
