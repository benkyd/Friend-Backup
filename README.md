# Friend Backup

Ever had friends and not wanted to pay for a backup provider?

Your friend can partition `n`GB of their drives for an encrypted portion of yours! Prioritise your most important files to fit within your friends budget, and set a schedule and bitrate for like-for-like backups! And to return the favour, you can offer allocation for your friends! 

### Model

Friend backup is a P2P file transport protocol as well as the application that encapsulates this idea and protocol. Every client plays two roles.
- Owner: Has data and pushes encrypted backups to peers
- Holder: Stores other peets encrypted backups and enforces grants

Ben and Jake list each other as peers. Each one is the owner of his backups and the holder of the others. The owner dials the holder and handshakes with the owner to negotiate the grant and how much the next transaction is going to cost. This is implemented with [Iroh](https://docs.iroh.computer/) as a QUIC UDP transport protocol that can handle large transfers and small messages.

Friend Backup runs as a persistant process with cron-like timings that can initiate backups. It runs persistantly in order to accept incoming requests from other peers.

### Identity

Key | Function
| -- | -- |
Iroh Secret Key | transport identity and signing headers
[age](https://github.com/filosottile/age) recipient | encrypt backups
[age](https://github.com/filosottile/age) identity + signing pub | restore backups


## AI Usage

Generative AI/LLMs were not used in the creation of this software.

