![Hits](https://hitcounter.pythonanywhere.com/count/tag.svg?url = https://github.com/ateschan/rusty-reportcard)

# rusty-reportcard
This is a cli tool for canvas.


## Usage

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

### Installing
Click [here](https://github.com/ateschan/rusty-reportcard/releases) and download latest release.

### Compiling from source
Make sure you have rust installed
```
git clone https://github.com/ateschan/rusty-reportcard
cd rusty-reportcard
cargo run
```

Todo 
- [ ] add cli functionality to print out only certain <ul>classes</ul>
- [ ] custom color menu
- [ ] add tests for the api key
- [ ] proper error checking
- [ ] improve readability :)
