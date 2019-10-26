# InstagramJSON
<p align="center"><img src="https://i.imgur.com/eyUG8iK.png" alt="Terminal showing InstagramJSON" width="60%" height="auto" align="middle"></img></p>

Take the messages.json from your Instagram download archive and turn it to something humanly readable!
InstagramJSON extracts your messages from your download archive to individual .txts with timestamps, usernames, messages and links!

<img src="https://i.imgur.com/eXWWaHN.png" alt="Not actually how it works" width="50%" height="auto" align="right"></img>
### Installation
The best way to get InstagramJSON is to grab a binary, download the latest release [here](https://github.com/Mnpn03/InstagramJSON/releases)!

If you instead want to compile InstagramJSON, you can do so by getting [Rust](https://www.rust-lang.org/).
Once that is installed, clone the repository:
`git clone git@github.com:Mnpn03/InstagramJSON.git`
Then you simply build it by running `cargo build --release`.

### Usage
```
macOS/Linux:
$ ./instagramjson messages.json
Windows:
> instagramjson.exe messages.json
```

An example of a line from one of the exported conversations:
```
(2019-12-25 14:56:12) user1: This is rather neat, isn't it?
```

### Contribution
If you want to help improve this utility, feel free to create a [pull request](https://github.com/Mnpn03/InstagramJSON/pulls) or an [issue](https://github.com/Mnpn03/InstagramJSON/issues).

### License
InstagramJSON is FOSS that comes with no warranty. Read more about the license used [here](https://github.com/Mnpn03/InstagramJSON/blob/master/LICENSE).
