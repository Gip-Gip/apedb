# To-Do List

##13.Mar.22:
  - [X] ~~Make README.md~~ **Commit b56b95f**
  - [X] ~~Implement CRC-24 functions~~ **Commit a125709**

    - Added dependency CRC_ANY
    - Provided wrapper module crc24 to allow project-wide use of the ApeDB crc24 standard
      - Also allows the swapping of crc libraries without any major re-writes of code
    - Defined the ApeDB CRC standard in crc24.rs

  - [ ] Implement UUID generator
  - [ ] Implement UUID cache
  - [ ] Implement basic database file I/O
