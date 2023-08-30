# news-curator

## Generate Site

To generate the static site, first ensure you have the [Rust toolchain](https://www.rust-lang.org/tools/install) and a local [http-server](https://www.npmjs.com/package/http-server) installed.

1. Execute `cargo run` to generate the HTML in the `/site` directory.
2. Copy the contents of the `/resources` directory to the generated `/site` directory using the command `cp resources/* site`.
3. Serve the static files in the `/site` directory using an http-server like `npx http-server site`.

## Configure Sources

The news sources can be configured in the `news_config.yml` file.

To add a new news source, find the RSS/Atom link for the feed and add a new item to the `sources` list:

```yaml
sources:
  - title: Example
    link: https://www.example.com/link-to-rss.xml
    category: Tech
    s_type: RSS # or Atom
```
