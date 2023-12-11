# rusty-reportcard
This is a cli tool for canvas.


## usage

### Setting up API Key

- [Go to Alamo Profile Settings.](https://alamo.instructure.com/profile/settings)

- Click on the API Access Tokens tab.

- Click on New Access Token.

- Enter a name for the token and click Generate Token.

- Copy the generated token.

#### Windows
```
setx API_KEY="{pasteapikeyhere}"
```


#### Mac/Linux
```
export API_KEY="{pasteapikeyhere}"
```



### Installing (linux)
- click [here](https://github.com/ateschan/rusty-reportcard/releases) and download latest release.

### compiling from source
```
yay -S rustup
git clone https://github.com/ateschan/rusty-reportcard
cd rusty-reportcard
cargo run

```

todo 
- [ ] add cli functionality to print out only certain <ul>classes</ul>
- [ ] custom color menu
- [ ] add tests for the api key
- [ ] proper error checking
- [ ] improve readability :)
