dataSize <- 20
N <- 8

data <- rbinom(dataSize, N, 0.45)

likelihood <- function (q) {
  sum(log(dbinom(data, N, q)))
}

develop <- function(q) {
  if (runif(1, 0, 1) < 0.5) {
    q + 0.01
  } else {
    q - 0.01
  }
}
  
sampling <- function(n, init) {
  q <- init
  ql <- likelihood(q)
  samples <- vector(length = n)
  i <- 1
  while(i <= n) {
    p <- develop(q)   
    pl <- likelihood(p)
    r <- log(runif(1, 0, 1))
    if (ql < pl || r < pl - ql) {
      q <- p
      ql <- pl
    }
    samples[i] <- p
    i <- i + 1
  }
  samples
}
samples1 <- sampling(500, 0.3)
hist(samples1, breaks = seq(0, 1, by = 0.01), main = "500 samples", xlab = "q")

samples2 <- c(samples1, sampling(500, samples1[500]))
hist(samples2, breaks = seq(0, 1, by = 0.01), main = "1000 samples", xlab = "q")

samples3 <- c(samples2, sampling(1000, samples2[1000]))
hist(samples3, breaks = seq(0, 1, by = 0.01), main = "2000 samples", xlab = "q")

samples4 <- c(samples3, sampling(8000, samples3[2000]))
hist(samples4, breaks = seq(0, 1, by = 0.01), main = "10000 samples", xlab = "q")
