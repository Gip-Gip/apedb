# To-Do List

##13.Mar.22:
  0. Make README.md
  1. ~~Implement CRC-24 functions~~ **Commit a125709**
    - Added dependency CRC_ANY
    - Provided wrapper module crc24 to allow project-wide use of the ApeDB crc24 standard
      - Also allows the swapping of crc libraries without any major re-writes of code
    - Defined the ApeDB CRC standard in crc24.rs
  2. Implement UUID generator
  3. Implement UUID cache
  4. Implement basic database file I/O
