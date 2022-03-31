# To-Do List

## 31.Mar.22
  - [ ] Implement "I" type
  - [ ] **Outline code formatting and standards**
  - [ ] Elaborate on definitions
  - [ ] Implements structs
    - [ ] Implement requirements
  - [ ] Implement basic database file I/O
    - [ ] Re-implement a database struct
      - [ ] Lay out a few properties likely needed for a database header
      - [ ] Automatically update the database chunk
    - [ ] Implement file "chunks"
    - [ ] Implement chunk continuation
      - [ ] Implement chunk updates

## 22.Mar.22:
- [X] ~~Document existing code with comments~~ **Commit a29ec14**
- [ ] Implement "I" type

  - Partially implemented **Commit a29ec14**

- [ ] Implement basic database file I/O
  - [ ] Re-implement a database struct
    - [ ] Lay out a few properties likely needed for a database header

      - Partially implemented **Commit a29ec14**
      

    - [ ] Automatically update the database chunk
  - [X] ~~Design standard to store fields of data for header and possibly other purposes~~ **Commit 8b5040c**
  - [ ] Implement file "chunks"
    - [X] ~~Implement a header chunk~~ **Commit 8b5040c**
    - [ ] Implement chunk continuation
    - [ ] Implement chunk updates

## 13.Mar.22:
  - [ ] Implement basic database file I/O
    - [X] ~~Implement Database structs~~ **Commit 35271ac** **Nullified 8b5040c**
      - [X] ~~Implement function to create a new database~~ **Commit 35271ac** **Nullified 8b5040c**
      - [X] ~~Implement function to open an existing database~~ **Commit 35271ac** **Nullified 8b5040c**
    - [ ] Implement file "chunks"
      - [X] ~~Implement functions to add generic chunks~~ **Commit 35271ac** **Nullified 8b5040c**
      - [X] ~~Implement functions to verify the CRC of a chunk~~ **Commit 35271ac** **Nullified 8b5040c**

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
