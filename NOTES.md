in over my head on this one.

notes for later
- Maybe random generator initialization is slow enough to warrant putting it some global struct, but that means more data to pass around.
- Currently sample NTT gets XOF bytes 3 at a time, each time the XOF struct calls the SHAKE128Reader read function, maybe its worth it to get bytes in bigger chunks and store them on the XOF struct.
- Use lazy static for params?? which ones?? maybe k and eta_1 or all of them