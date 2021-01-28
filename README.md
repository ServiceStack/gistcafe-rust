Useful utils for [gist.cafe](https://gist.cafe) Rust Apps.

## Usage

Simple Usage Example:

```rust
let org_name = "rust-lang";

let res = reqwest::Client::new()
    .get(&format!("https://api.github.com/orgs/{}/repos", org_name))
    .header(reqwest::header::USER_AGENT, "gist.cafe")
    .send()
    .await.unwrap();

let json: Vec<Map<String,Value>> = res.json().await.unwrap();
let mut org_repos: Vec<Map<String,Value>> = Vec::new();
for x in json.iter() {
    org_repos.push(json!({
        "name":        x["name"],
        "description": x["description"],
        "lang":        x["language"],
        "watchers":    x["watchers"],
        "forks":       x["forks"],
    }).as_object().unwrap().clone());
}
org_repos.sort_by(|a, b| b["watchers"].as_i64().cmp(&a["watchers"].as_i64()));

println!("Top 3 {} GitHub Repos:", org_name);
inspect::print_dump(&org_repos[1..=3]);

println!("\nTop 10 {} GitHub Repos:", org_name);
inspect::print_dump_table(&org_repos[1..=10].iter().map(|x| json!({
    "name":        x["name"],
    "lang":        x["lang"],
    "watchers":    x["watchers"],
    "forks":       x["forks"],
}).as_object().unwrap().clone()).collect());

println!("\nTop 10 {} GitHub Repos:", org_name);
inspect::print_dump_table_columns(&org_repos[1..=10].to_vec(), 
    vec!["name", "lang", "watchers", "forks"]);
```

Which outputs:

```
Top 3 rust-lang GitHub Repos:
[
  {
    description: The Rust package manager,
    forks: 1322,
    lang: Rust,
    name: cargo,
    watchers: 6413
  },
  {
    description: A bunch of lints to catch common mistakes and improve your Rust code,
    forks: 755,
    lang: Rust,
    name: rust-clippy,
    watchers: 5438
  },
  {
    description: Learn Rust with examples (Live code editor included),
    forks: 764,
    lang: null,
    name: rust-by-example,
    watchers: 3770
  }
]

Top 10 rust-lang GitHub Repos:
+-----------------------------------------------------+
| forks |     lang     |       name        | watchers |
|-----------------------------------------------------|
|  1322 ! Rust         ! cargo             !     6413 !
|   755 ! Rust         ! rust-clippy       !     5438 !
|   764 ! null         ! rust-by-example   !     3770 !
|  1097 ! Shell        ! rfcs              !     3413 !
|   224 ! Vim script   ! rust.vim          !     2433 !
|   427 ! Rust         ! crates.io         !     1870 !
|   264 ! Rust         ! regex             !     1756 !
|   148 ! Rust         ! log               !      975 !
|   569 ! Rust         ! libc              !      944 !
|   262 ! Rust         ! git2-rs           !      794 !
+-----------------------------------------------------+

Top 10 rust-lang GitHub Repos:
+-----------------------------------------------------+
|       name        |     lang     | watchers | forks |
|-----------------------------------------------------|
| cargo             ! Rust         !     6413 !  1322 !
| rust-clippy       ! Rust         !     5438 !   755 !
| rust-by-example   ! null         !     3770 !   764 !
| rfcs              ! Shell        !     3413 !  1097 !
| rust.vim          ! Vim script   !     2433 !   224 !
| crates.io         ! Rust         !     1870 !   427 !
| regex             ! Rust         !     1756 !   264 !
| log               ! Rust         !      975 !   148 !
| libc              ! Rust         !      944 !   569 !
| git2-rs           ! Rust         !      794 !   262 !
+-----------------------------------------------------+
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ServiceStack/gistcafe-rust.