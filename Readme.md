# Welcome to Bitcoin Warp
Bitcoin Warp is an ongoing implementation of a new Bitcoin Client in Rust. You can track our progress on [Github](https://github.com/preston-evans98/bitcoin-warp) or view our docs at [bitcoinwarp.com](bitcoinwarp.com).

Bitcoin Warp is a work in progress.  It is developed as a collection of 
libraries implementing the different components of a Bitcoin node (such as networking, cryptography, shared primitives, etc.), and a `warpd` binary which uses them. Expect the structure of this project to change very frequently up until the 1.0 release.

Almost all of our work so far has gone into `networking`, an implementation of the Bitcoin Wire Protocol
inspired by [Zcash Zebra's](https://doc.zebra.zfnd.org) new networking stack. However, many of the core components of the `shared` library have also begun to stabilize.
## Why Another Bitcoin Implementation?
There are at least seven separate implementations of the Bitcoin protocol. Why do we need another one?

### 1. Bitcoin Core has about 98% Market Share
Core has earned its place at the top. It's battle-tested, actively maintained, and blazing fast. But too much concentration isn't a good thing. 
1. **It places an undue burden on the core devs.** Since a bug in Core could effectively bring down the entire Bitcoin network, the devs have to be perfect. They've done remarkably well so far, but its unreasonable to expect that they'll keep it up forever. 
1. **It slows innovation.** In Ethereum, both Geth and Parity (and now Nethermind) can try out new features. If a feature doesn't work out and part of the network crashes, it's bad but not catastrophic. Bitcoin Core doesn't have that luxury. They need to maintain 100% uptime, or Bitcoin dies. In addition, potential contributors need to be advanced C++ programmers willing to dive into a highly complex codebase. Obviously, this somewhat limits the pool of contributors.
1. **It leaves us vulnerable to supply chain attacks**. As Bitcoin grows to trillion dollar market cap and beyond, it may very well be targeted by national intelligence services. Expect to see a variety of attacks on Bitcoin Core's supply chain - from tampering with dev machines to stealing release keys. Heck, someone might even try to mess with C++ compilers. In any case, *Vires in Numeris.*

### 2. There is no Viable Alternative Client
1. **Sync times for the next best client are [3x longer](https://blog.lopp.net/bitcoin-node-performance-sync-tests/) than Bitcoin Core.** Syncing Core is already painful enough. 
1. **Most Alt-Clients are vulnerable to supply chain attacks**. The best performing Alt-Client (Bcoin) is written in Node JS. With NPM. Yikes. The next best (GoCoin) is also vulnerable (lock files anyone?). This is a common pattern. Most languages don't take supply chain attacks seriously, and it shows.
1. **Alt-Clients have restrictive licenses**. GoCoin, Parity - I'm looking at you. 

## What Will Make Bitcoin Warp Different?
1. **Performance.**. Unlike clients written in Go (or C#, or JS, or Python, or...) there's no reason Bitcoin Warp needs to be slower than Bitcoin Core. Hopefully, it will be even faster.
1. **Security.** Bitcoin Warp will be (almost) completely immune to memory safety bugs. It will have a secure supply chain. 
1. **Simplicity.** We expect Bitcoin Warp to be significantly smaller (in terms of SLOC) than Bitcoin Core. 
1. **Free as in Freedom.** And Free as in Beer. Bitcoin Warp is open source and MIT licensed.

How is all of that possible? Mostly, it comes down to writing in Rust.  In case you're not familiar, Rust is a systems programming language that combines blazing speed with complete memory safety (assuming you don't need `unsafe` features). It doesn't have garbage collection, so it produces binaries that run just as fast as `C` programs, and don't take up any more space.  It has a package manager that provides reasonable protections against supply chain attacks (lock files with hashes are a big plus). It also has a bunch of neat features like iterators, closures, generics, and macros. 

This means that if we do our job, Bitcoin Warp can be just as fast as a C/C++ Bitcoin implementation like Bitcoin Core (or even faster) and we can write a lot less code. That means easier maintenance, faster iteration, and (hopefully) more innovation.

# Roadmap 
Implementation of Bitcoin Warp is split into several phases with accompanying milestones. 

## Pre-Alpha

### 1. Basic Networking (Feature Complete - in QA)
Implement the Bitcoin wire protocol. Allows messages to be exchanged with nodes running Bitcoin Core to allow fetching blocks etc. during development. Does not include peer management, mempool, etc. 
### 2. Basic Validation (Implementation In Progress)
Implement the basic logic to validate blocks and transactions. This includes things like checking signatures and block hashes. Partially dependent on 3. This implementation will be unoptimized, and may not include changes introduced into Bitcoin via soft fork. These changes will be added in the next phase. 
### 3. Basic Database
Implement a minimum viable database layer to store blocks and UTXOs. Likely LevelDB initially. 
### 4. Basic Interface
Implement config files and simple CLI. 
### 5. Advanced Networking (Implementation in Progress)
Implement a connection manager. Accomplishment of this milestone marks the client ready for Alpha Launch.


## Alpha
### Advanced Validation
Implement soft-fork features like SegWit and TapRoot. Optimize. 

### Advanced Database (Optional)
Optimize the database. Explore alternatives to LevelDB, including potentially designing a Bitcoin-specific DB engine. 

### Production Level Integration Testing
'Nuff Said. 

## Beta
### Advanced Interface
Electron + Typescript = Better UX. 

### State of the Art Networking
Explore using libP2p, or implementing a new networking protocol based on more recent academic literature. This might be unnecessary. It might also be fun. In any case, we definitely need authenticated messaging.

...

That seems like more than enough to get started. If we make it this far, we should be able to get funding for an audit and launch this thing for real. 


**N.B.** There's nothing about wallet functionality on the roadmap. That's intentional. ***You shouldn't store BTC on a hot device unless you really know what you're doing***, so having a wallet on your node is asking for trouble. We won't add this feature unless there's a very compelling need to do so. 

# Getting Started
1. [Install the Rust Compiler](https://www.rust-lang.org/tools/install)
1. Clone our git repository `git clone https://github.com/preston-evans98/bitcoin-warp.git`
1. `cd bitcoin-warp`
1. Run `cargo test --all` to build the client and run all unit tests. 
1. (Optional) `cargo run` to see the experimental warp shell in action. This shell lets you connect to any other client running Bitcoin and send and receive messages from the command line. Note: This is very much a work in progress. It depends on some functionality from the connection manager, which won't be fully implemented until the end of the Alpha stage. 
