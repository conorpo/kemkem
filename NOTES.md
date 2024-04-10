in over my head on this one.

notes for later
- Maybe random generator initialization is slow enough to warrant putting it some global struct, but that means more data to pass around.
- Currently sample NTT gets XOF bytes 3 at a time, each time the XOF struct calls the SHAKE128Reader read function, maybe its worth it to get bytes in bigger chunks and store them on the XOF struct.
- Figure our bit compression / bits to bytes, is it best to use a bitvec, BitArray, or just standard rust?
- Is it fine that some methods are inplace and some are not? Matrix multiplication and inner product sometimes requires a seperate output, but addition and scalar multiplication seem to only be used inplace, aswell as the NTT and NTT^-1.
- Only functions tghat need to input / output bytes are the mlke.rs functions as requested by FIPS 203, inner functions can abstract and organize as needed.
- We want entry-point functions to be easy to use, but we also want to maintain FIPS 203 compliance, so maybe add some sort of serialization / deserialize traits to these functions?
