![GitHub Build Status](https://img.shields.io/github/actions/workflow/status/donet-server/donet/build.yml?logo=github&label=Build)
[![Coverage Status](https://codecov.io/gh/donet-server/donet/branch/master/graph/badge.svg)](https://codecov.io/gh/donet-server/donet)
[![Discord](https://img.shields.io/discord/1066973060357443644?color=blue&label=Discord&logo=discord&logoColor=white)](https://discord.gg/T6jGjEutfy)

# donet

<img src="logo/donet_banner.png" alt="Donet logo artwork by honeymatsu." align="right" width="40%"/>

Donet is a free and open source network engine designed after the Distributed Networking protocol, 
as defined in the high-level networking API of the [Panda3D](https://panda3d.org) game engine,
which was originally developed by Disney Interactive (*formerly known as Disney VR Studios*) to connect 
with their in-house server technology, the OTP (*Online Theme Park*) server, which was used to power 
their massive multiplayer online games, such as Toontown Online and Pirates of the Caribbean Online, 
from 2001 to 2013.

## Getting Started
The Donet repository houses two different Rust projects:
- **donet** - The Donet daemon source, which includes all the Donet services. See [donet-server.org](https://www.donet-server.org).
- **libdonet** - The core utilities for Donet services, including datagram utilities and the DC file parser. See [libdonet.rs](https://libdonet.rs).

Please read the [introduction to Donet](./docs/01-Introduction.md) for an overview of the project 
and how the engine works.

Before starting your own contribution to Donet, please read over the [Contributing Guidelines](./CONTRIBUTING.md). If you are a first
time contributor to the project, please add your name and Git email address to the [CONTRIBUTORS.md](./CONTRIBUTORS.md) markdown file.
You may also use your GitHub username as your name. If your contribution includes modifications to source code, please add your
name and Git email in the [Cargo.toml](./Cargo.toml) file as an author of this project.

To build Donet, run the following under the project directory:
```sh
cargo build --release
```

If you are working on a contribution to either the Donet daemon or libdonet, please run code linting and unit testing before pushing:
```sh
cargo clippy
cargo fmt --all -- --check
cargo test
```
These checks should go over all source files in both `donet/` and `libdonet/` source directories.

If you have any further questions, feel free to join [our community Discord server](https://discord.gg/T6jGjEutfy).

## Documentation
Currently there is not much documentation on Donet, as libdonet is still under development.

For the libdonet rust library documentation, visit [libdonet.rs](https://libdonet.rs).

## Software License
The Donet engine is released under the GNU Affero General Public License, version 3.0 (AGPL-3.0), which 
is a copyleft open source software license. The terms of this license are available in the 
"[LICENSE](./LICENSE)" file.

### Distributed Networking architecture resources

Resources for more info on Panda's Distributed Networking (Sources listed in chronological order):

- [October 2003: Building a MMOG for the Million - Disney's Toontown Online](https://dl.acm.org/doi/10.1145/950566.950589)
- [Apr 16, 2008: The DistributedObject System, client side](https://www.youtube.com/watch?v=JsgCFVpXQtQ)
- [Apr 23, 2008: DistributedObjects and the OTP server](https://www.youtube.com/watch?v=r_ZP9SInPcs)
- [Apr 30, 2008: OTP Server Internals](https://www.youtube.com/watch?v=SzybRdxjYoA)
- [October 2010: (GDC Online) MMO 101 - Building Disney's Server System](https://www.gdcvault.com/play/1013776/MMO-101-Building-Disney-s)
- [(PDF Slideshow) MMO 101 - Building Disney's Server System](https://ubm-twvideo01.s3.amazonaws.com/o1/vault/gdconline10/slides/11516-MMO_101_Building_Disneys_Sever.pdf)

<br>

Donet logo artwork created by [honeymatsu](https://honeymatsu.carrd.co/). 🍩

Older revisions of the Donet logo created and designed by [Karla Valeria Rodriguez](https://github.com/karla-valeria). 🍩
