# rusty-reportcard

![ezgif-5-2d19f3b093](https://github.com/ateschan/rusty-reportcard/assets/89411709/9693d158-f49c-4ff6-b7e7-d57911ec3199)

This is a cli tool for alamo colleges canvas.


## Usage

### Setting up API Key

- [Go to Alamo Profile Settings.](https://alamo.instructure.com/profile/settings)

- Click on the API Access Tokens tab.

- Click on New Access Token.

- Enter a name for the token and click Generate Token.

- Copy the generated token.

#### Windows
```
setx API_KEY="Bearer pasteapikeyhere"
```


#### Mac/Linux
```
export API_KEY="Bearer pasteapikeyhere"
```

### Installing
Click [here](https://github.com/ateschan/rusty-reportcard/releases) and download latest release.

### Compiling from source
Make sure you have rust installed
```
git clone git@github.com:ateschan/rusty-reportcard.git
cd rusty-reportcard
cargo run
```
