# Simple-Primitives ReadMe
**Commands that you need to put a Rust WASM application on a website**
You only need this readme if you are new to Rust WASM.

Documentation about this website can be found on the Google Drive [Drive Location](https://drive.google.com/drive/folders/1xUZnBYVugI-iMI3zimdjQmwDcNvYzN_r).

## Cmd commands
This chapter is a summary of [rustwasm.github.io](https://rustwasm.github.io/docs/book/game-of-life/setup.html) chapter 4.1
I've writen this chapter for the Windows Command Prompt

### Compiling and creating a pkg directory
```
wasm-pack build
```
Run this in the root of yor project. It makes a pkg dir. Inside this dir you will find your created WASM module which is used for testing. If certain Rust Crates cannot be found it probably means that you didn't add them to your Cargo.toml file which you can find in the root. You an find the missing crates on https://crates.io/ where you can also find out how you add them to the toml file.

### Making the pkg dir ready for the website
```
npm init wasm-app www
```
Makes a www dir where you can create the website that uses your WASM module.

### Installment of all the dependencies
```
npm install 
```
Run this in your www dir. It should download all the dependencies that you'll need.

### Local testing
```
npm run start
```
Run this in your www dir to test your website locally http://localhost:8080/. If you get the error Can't resolve '../pkg/your-app-name' it probably means that the name of your pkg isn't correct in your index.js file.

### Package JSON file
Add dependencies to your www/package.json file.
```
{
  // ...
  "dependencies": {
    "Insert your app name here": "file:../pkg"
  },
  "devDependencies": {
    //...
  }
}  
```

### Building
```
npm run build
```
Run this in your www dir. If everytings goes well you can find your website files in www/dist dir!