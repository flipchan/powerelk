
# PowerElk - A Caching elasticsearch proxy written in Rust   


written in order to take load of elasticsearch


Using tokio, sled and reqwests  

*Secretly written in order to slowly replace elasticsearch with 100% Rust power*


### Build me:    
```shell 
$ git clone https://github.com/flipchan/powerelk
$ cd powerelk && cargo build --release
```

### Run me:  
Edit powerelk.toml: 

```toml
[host]
bind = "0.0.0.0"
port = 18080
elasticsearchindex = "myelkindex"
elasticsearchinstance = "http://0.0.0.0:9200"
cachelocation = "/tmp/mycache"
```

## Run   
```shell
$ ./target/release/powerelk
Listening on http://0.0.0.0:1387
```






# Test me
```shell  
$ curl -v http://0.0.0.0:1387/search  -d'{"search":fluff"}'   
$ curl -v http://0.0.0.0:1387/removekey  -d'{"search":"fluff"}'   
$ curl -v http://0.0.0.0:1387/random -XGET
```



## Default Mode  
You can use Powerelk with Rust own [Default::default](https://doc.rust-lang.org/std/default/trait.Default.html)   

cat src/elk.rs    
```rust
impl Default for Elk{

        fn default() -> Self {
                Self {
                instance: "http://127.0.0.1:9200".into(),
                index: "myindex".into(),}
        }
}
```

cat src/database.rs   

```rust
impl Default for Database {
    fn default() -> Self {
        Self {
            filelocation: "/tmp/database.db".into(),
        }
    }
}

```

cat src/


