# rescue-hash
This is a wasm version of rescue-hash.

## 1. What this can do?
1. It can do standardized rescue hash for 2 data. Such as Hash(A,B) => C (A,B and C shoud be `[u64;4]`, each data should contain 4 `u64` elements).
2. It can do rescue hash consecutively, in other word, it can Hash(A,B,C,D,....) => Hash(A,(B,(C,D)...)) Each data should contain 4 `u64` elements as well).
   

## 2. Input and Outputs
The `inputs` specifies the rescue input, it should contain 8 elements or more(over 8 but should be some multiple of 4);

The `outputs` is the rescue result, which contains 4 elements as `Vec<u64>`.


## 3. How to use it ?
### a. For local test
Run the following commands:

```bash
wasm-pack build --target web
```

```bash
python -m http.server 8008
```


And then you could see the hash result through your browser at `http://localhost:8008/`

### b. For Js
Run the following commands:

```bash
wasm-pack build --target nodejs
```

Move the pkg file to the workspace and run:


```bash
npm install ./pkg
```

and then you can use this rescue as a module.

