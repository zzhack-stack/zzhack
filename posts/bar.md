```metadata
{
    "cover": "../app/assets/images/into_the_wild.jpg",
    "tag": "asd",
    "title": "My life foo bar"
}
```

This site is building based on Rust & Yew, since the browser can run languages other than JavaScript, the compiler of these language can compile to WASM and run in the browser, I think It's so interesting and innovative, so I found the [Yew](https://yew.rs/).

The [Yew](https://yew.rs/) is a framework of Rust for building client web app, the interesting things is that it makes trick use of Rust's macro to express construction of component, a component may looks like:

```rust
struct Foo {}

impl Component for Foo {
    fn view(&self) -> Html {
        html! {
            <div></div>
        }
    }
}
```

That's fucking awesome! which means that u can write component in Rust just like React or some JSX based framework, but the Yew is based on wasm-bindgen which means the dist code is a bundle of `WASM & JS` (cause the WASM cannot access DOM directly, so u may like access DOM using JS and then lnk the JS module in ur WASM module) rather than JS bundle.

### Where does the blog data comes from?
May u have notice that this site is not a STATIC WEBSITE, it's dynamically, the data comes from CDN nodes, when I write post down and commit it to repository, the post would be upload to CDN automatically, I was writing a script to make the flow execution automatically using Python 3.9, and the tool is also open source, u can find it in [zzhack-stack/zzhack-provider](https://github.com/zzhack-stack/zzhack-provider).

Of course the site source code and data r store separately in two deference repositories. May u wanna asked why not do static compile post file to dist and publish to CDN, it's looks like simpler and faster, cause I wanna make a architecture which like C/S arch, it's easier to expand client endpoint, assume that I have a iOS/Android client or WeChat mini-program client to show my posts, the clients share the same data, if I compile the posts into the dist bundle, and publish it with source code, of course it would be more manageable and it would be simpler, but I don't wanna compile for 4 times.

And It's worth mention that the website were deploying on Vercel, and also there's no "server" here, so this blog site also is a server-less app.

### How to add comments to the post
There have an 1:1 relationship between GitHub issues and blog posts, u can add comments via GitHub API, and u have to login via GitHub OAuth before comment, and the content of comments will appear to the end of the issue. And when u refresh the page, the app will pull the comments data from issues via GitHub API, that's all details that u wanna to know about comments system of the blog site.

Anyway happy landing! 
