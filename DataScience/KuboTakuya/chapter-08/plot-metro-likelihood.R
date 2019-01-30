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

qs <- seq(0, 1, by = 0.01)
ls <- vector(length = length(qs))
for(i in 1:length(qs)) {
  ls[i] <- prod(dbinom(data, N, qs[i]))
}
plot(qs, ls, type = "l", xlab = "q", ylab = "Likelihood")

samples <- sampling(10000, 0.3)
hist(samples, breaks = seq(0, 1, by = 0.01), main = "10000 samples", xlab = "q")
