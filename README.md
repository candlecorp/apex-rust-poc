# Shopify Function project
## Apexlang Docs
https://apexlang.io

## Apexlang Codegens
https://github.com/apexlang/codegen
Custom code generators can be created.  Referenced on line 4 of apex.yaml

## Step 1) Specify your input and output format.

Edit the `types.apexlang` file to change the input and output format to your expected types.

## Step 2) Generate code automatically

Run the command below to generate the appropriate code for your types.

```shell-session
user@host:~$ apex generate
```

## Step 3) Add your function logic.

Modify `src/function.rs` to include your custom function logic.

## Step 4) Build

Run the command below to execute your WebAssembly function.

```shell-session
user@host:~$ make
```

## Step 5) Run

You can use `echo` or similar to pipe test data into your function and observe the output directly.

```shell-session
user@host:~$ echo '{"id":1,"price":32.45}' | wasmtime build/my-function.wasm
{
  "id": 1,
  "price": 32.45,
  "tax": 0.095
}
```
