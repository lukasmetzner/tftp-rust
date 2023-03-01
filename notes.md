# TFTP

- https://www.rfc-editor.org/rfc/rfc1350

- ontop of udp
- TID: transfer identifiers
- WRQ: write request
- RRQ: read request
- 512 byte packets
- packet < 512 bytes -> termination
- send pack; ack pack; send pack; ack pack;....
- resend pack if not ack
- three transfer modes
    - 8 bit ascii
    - raw 8 bit bytes
    - mail, netascii
- packets have a internet header, datagram header, tftp header
- block number starts with one
    - on write it starts with zero to signal a positive response
- if block number does not increase -> error -> retransmit
- each connection chooses a random TID
- every packet has source and destination TID at the end
- TID should remain same for source and dest during the whole transfer (not same -> error packet -> retransmit)

### Packet Types
| opcode | operation |
|---|---|
| 1 | Read request (RRQ) |
| 2 | Write request (WRQ) |
| 3 | Data (DATA) |
| 4 | Acknowledgment (ACK) |
| 5 | Error (Error) |