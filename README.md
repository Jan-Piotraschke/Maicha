# Maicha
A place for my girlfriend creativities 

## Literate programming

This project was designed with the literate programming paradigm.    

Requirements:    
- install [Literate](https://github.com/zyedidia/Literate) (a literate programming tool) with:    
`brew tap zyedidia/literate` and `brew install --HEAD literate`

Make *maicha* executable with `lit book.lit`   

### Documentation of the internal workings of *maicha*

Open in the directory *_book* the HTML file *Maicha_contents.html*

## Rocket

Rocket requires Rust nightly. To install on MacOS run:
- `brew install rust` to remove your current rust installation. If you have installed rust in another way other than
  brew, this will probably not work.
- `brew install rustup-init` Install the rustup installer.
- `rustup-init` install rustup. Just use the recommended options.
- `cd Maicha`
- `rustup override set nightly` to install and enable the nightly rust builds in the `Maicha` directory.

To start run:
- (`cd Maicha`)
- `cargo run` (or `cargo watch -x run`; install cargo watch with `cargo install cargo-watch`)
- open [localhost:8000](http://localhost:8000/)


## The Setup

- When initially opening `/` one will be redirected to `/login`.
- At `/login` one can login with the user name `JNP` and any password.
- When logged in, one will be redirected to `/`.
- Trying to login again will just redirect the user back to `/`.
- One can logout at `/logout` and will be redirected to `/login`.
