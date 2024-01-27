The best way to run this program is with the command:

```cargo run --package COP4520-PA1 --bin COP4520-PA1 --release```

This will build the program using cargo and run it.

It takes about 45 seconds on my AMD Ryzen 2600x (6 cores 12 threads) to find 5761455 prime numbers smaller than 10^8.

I went through quite a few iterations to try and come up with an optimal algorithm. I wanted to do a Sieve of Eratosthenes
implementation using multithreading and spent a lot of time trying to figure out where I could utilize multithreading within
that algorithm.

Implementing thread synchronization in a sieve is an incredibly delicate operation, and I could not come up with a feasible 
solution with my current knowledge of multithreading. I hope to learn more in this class how to implement shared memory spaces
in a thread safe manner that would enable this.

The optimization I am most proud of is using the integral of the sqrt(x) function to split the area of the runtime curve
into 8 even parts. The work shared between threads is not completely even due to certain threads having to do more loop iterations,
but all 8 threads are utilized and the algorithm runs about 5-6x faster than the single threaded approach.