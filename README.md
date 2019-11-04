# JiraC | [![Build Status](https://travis-ci.org/hazardfn/jirac.svg?branch=master "Build Status")](http://travis-ci.org/hazardfn/jirac) [![GitHub release](https://img.shields.io/github/release/hazardfn/jirac.svg)](https://github.com/hazardfn/jirac/releases/latest)

> A fully featured jira client library written in the glorious powerhouse
> that is Rust!

<p align="center">
<img src="https://applitools.com/blog/wp-content/uploads/2018/07/Jira-new-logo.png" height="238" width="549">
</p>

## DISCLAIMER

This is my first rust project, it probably isn't idiomatic, sane or even useable outside of the laptop I wrote it on (though Travis is on my side).

In short the library is pre-alpha and will probably remain that way for some time, only a small subset of calls are implemented and they aren't even the good ones you probably want...

You can track the implemented APIs [here](https://github.com/hazardfn/jirac/issues/4).

## Contents

1. [Prerequisites](#prerequisites)
    * [Rust](#rust)
    * [JIRA](#jira)
2. [Installation](#installation)
3. [Examples](#examples)
    * [Get User](#get-user)
4. [Contributions](#contribute)
5. [Acknowledgements](#ack)

## 1. Prerequisites<a name="prerequisites"></a>

As always there are prerequisites required before using JiraC, most of these are obvious but contain some information on which versions are tested and supported.

### Rust<a name="rust"></a>

Currently all rust versions ~> 1.34 are supported. All commits are tested against the stable, beta and nightly channels.

Tested by travis:

* Rust: `1.34`
* Rust: `1.36`
* Rust: `Stable, Beta, Nightly`

### Jira<a name="jira"></a>

I have only personally tested the provided functionality on a JIRA Server instance and can confirm it works on v7.x.x.

I have tried to make the client robust to subtle changes in format, if your version of JIRA doesn't have a field a later one does it should just give you back a default value for that field type.

For example if your instance doesn't have `isLast` on pagination you will see it but it will always be false.

## 2. Installation<a name="installation"></a>

For now JiraC does not publish to cargo because it is in early development. I recommend you add it as a git dependency:

```toml
[dependencies]
jirac = { git = "https://github.com/hazardfn/jirac" }
```

## 3. Examples<a name="examples"></a>

Using JiraC is hopefully fairly intuitive (even in its nerfed state), I have provided one example below but fetching/manipulating data for the other jira types work in the same way.

### Get User<a name="get-user"></a>

Simply add the following to your `lib.rs`/`main.rs`:

```
use jirac::Client;
use jirac::Credentials;
use jirac::v2::{UserExpand, User};
use jirac::Resp;

pub fn main() {
    // Only basic auth is supported at the moment
    let url = "https://whereisyourjira.com"
    let credentials = Credentials::new("username", "password").unwrap();
    let client = Client::new(&url, credentials);

    // Let's expand all the fields for fun
    let e = vec![UserExpand::ApplicationRoles, UserExpand::Groups];
    let Resp{data: user, headers: _h} = User::from_username(&client, "username", &e).unwrap();
    println("{}", user);
}
```

## 4. Contributions<a name="contribute"></a>

Contributions are warmly received, check out the Projects section for some ideas I have written down and for the latest on what is underway.

### Guidelines<a name="guidelines"></a>

Currently there are no hard guidelines, during the projects infancy only the following things are crucial:

* **PLEASE!** Report Bugs! If you tried this client out and couldn't get something to work I want to know about it.

* **PLEASE!** Give tips on any code smells here, as I mentioned above I am new to rust and these things really help.

* **PLEASE!** Submit any kind of merge request you want to make this library better! There are over 300 methods in version 2 of the JIRA API alone so if you want to knock one out over the weekend as part of your rust practice please do!

## 5. Acknowledgements<a name="ack"></a>

Thank you to the rust community for everything! The documentation for rust is unparalleled and there is so much wonderful material to dig through and well written guides.
