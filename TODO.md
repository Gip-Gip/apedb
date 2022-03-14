# To-Do List

## 13.Mar.22:
  - [X] ~~Make README.md~~ **Commit b56b95f**
  - [X] ~~Implement CRC-24 functions~~ **Commit a125709**

    - Added dependency crc-any
    - Provided wrapper module crc24 to allow project-wide use of the ApeDB crc24 standard
      - Also allows the swapping of crc libraries without any major re-writes of code
    - Defined the ApeDB CRC standard in crc24.rs

  - [X] ~~Implement UUID generator~~ **Commit 5432014**

    - Added dependency uuid
    - Provided wrapper module uuid to allow project wide use of UUIDs
      - Also allows the swapping of uuid libraries without any major re-writes of code

  - [X] ~~Implement UUID cache~~ **Commit db3c71a**

    - Added functions to handle UUID vectors
    - Added type UuidV4Cache, equal to a Vec<UuidV4>

  - [ ] Implement basic database file I/O
