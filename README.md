# Dilithium Implementation
#### Cyprien JORANT

This project is a rust implementation of the dilithium project made and published on  [GitHub by pq-crystals](https://github.com/pq-crystals/dilithium). The aim is to create a code the make a demonstration of the dilithium signature algorithm. 


# Tutorial
there are two files in this project
### Source
the source directory contain the rust code to :
- create a key,
- encrypt a message (nativly placed in "input.txt")
- create the key.txt file and signature.txt

To run it, you just have to use "cargo r" in a command shell launched from the source 
(in the rust file, there is an option so that the software checks the enscription localy) 
### Destination

the destination directory contain the rust code to verify the signature. The files needed are (native names):
- the source file ("input.txt")
- the key file ("key.txt")
- the signature file ("signature.txt")

The files need to be placed in the destination directory.

The source code and the destination code can run on different divices, to demonstration how the dilithium works
