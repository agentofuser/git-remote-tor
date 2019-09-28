<h1 align="center">Welcome to git-remote-tor 👋</h1>
<p>
  <img alt="Version" src="https://img.shields.io/badge/version-0.1.4-blue.svg?cacheSeconds=2592000" />
  <a href="https://agentofuser.com/git-remote-tor/" target="_blank">
    <img alt="Documentation" src="https://img.shields.io/badge/documentation-yes-brightgreen.svg" />
  </a>
  <a href="https://twitter.com/agentofuser" target="_blank">
    <img alt="Twitter: agentofuser" src="https://img.shields.io/twitter/follow/agentofuser.svg?style=social" />
  </a>
</p>

> Seamless .onion and tor-ified git remotes

### 🏠 [Documentation](https://agentofuser.com/git-remote-tor/)

## Install

```sh
cargo install git-remote-tor
```

## Usage

Prepend `tor::` to the remote's URL. Examples:

```sh
# clone .onion address
git clone tor::http://3lytcgmoe2j75c6t.onion/ logit

# clone clearnet address
git clone tor::https://github.com/agentofuser/logit.git logit

# add remote to existing repo
git remote add agentofuser tor::https://github.com/agentofuser/logit.git
```

Then use `git fetch`, `git pull`, etc. as you normally would.

For more information (including how to serve a read-only .onion git repo) check
out [the docs](https://agentofuser.com/git-remote-tor/).

## Author

👤 **Agent of User**

* Keybase: [@agentofuser](https://keybase.io/agentofuser)
* Twitter: [@agentofuser](https://twitter.com/agentofuser)
* Github: [@agentofuser](https://github.com/agentofuser)

## Show your support

Give a ⭐️ if this project helped you!

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
