# Actix Web File Upload with Async/Await

## Run

`cargo run --bin actixfileupload`

## Test

```bash
echo test > test.txt
curl -i -X POST -H "Content-Type: multipart/form-data" -F "data=@test.txt" http://localhost:3000/api/v1/log/upload
```

## Result

file(s) will show up in `./tmp` in the same directory as the running process

Note: this is a naive implementation and will panic on any error
