
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


