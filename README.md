# deadlinkrs

Search for dead links in any kind of file.

# Why

## Why yet another dead links tool

I wanted a tool that matched these criteria (or get as close as possible) :

- Easy to download for CI, wathever the CI tool (GitHub, GitLab ...).  
  So no `cargo install`, `npm install`, `gem install` or anything language related (apt would have been fine)
- An executable  
  No `.py` or whatever language that require an interpreter, I wanted something that run natively
- Configurable without config file required.  
  Parse only `html` files, or both `html` and `markdown` ...
- Filter URLs  
  Ignore `tel:` or `mail:` for instance
- Parseable output format
- Quick if possible

To sum up, something easy to install configure and use, suitable for automation locally or in CI.  
At the time, **I found** (maybe it existed but I didn't find it) nothing that would quite suit me needs.

## Why Rust

Then I decided to write my own.  
I like high performance languages like C, but I also know that
parsing user input, selecting files with globs, parsing files with regex, and sending http request
isn't exactly easy.  
Definetly could be done, but not easy and it would take some time.

So I wanted speed with Javascript / Python like high level utilities more or less our of the box.  
What better choice than **Rust**.
