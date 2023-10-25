# TFTP

The goal of this project is to implement a simple server for the TFTP protocol. Different RRQ/WRQ modes won't be supported.

# Protocol

- https://www.rfc-editor.org/rfc/rfc1350

## Packet Opcodes
| opcode | operation |
|---|---|
| 1 | Read request (RRQ) |
| 2 | Write request (WRQ) |
| 3 | Data (DATA) |
| 4 | Acknowledgment (ACK) |
| 5 | Error (Error) |

## Packet Formats
```
   Type   Op #     Format without header

          2 bytes    string   1 byte     string   1 byte
          -----------------------------------------------
   RRQ/  | Opcode |  Filename  |   0  |    Mode    |   0  |
   WRQ    -----------------------------------------------
          2 bytes    2 bytes       n bytes
          ---------------------------------
   DATA  | Opcode |  Block #  |    Data    |
          ---------------------------------
          2 bytes    2 bytes
          -------------------
   ACK   | Opcode |  Block #  |
          --------------------
          2 bytes  2 bytes        string    1 byte
          ----------------------------------------
   ERROR | Opcode |  ErrorCode |   ErrMsg   |   0  |
          ----------------------------------------
```